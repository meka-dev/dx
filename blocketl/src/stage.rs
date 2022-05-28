use std::{future::Future, ops::Deref as _};

use common::{err, storage, Context as _, Error, Report, Result};
use futures::stream::{self, StreamExt as _, TryStreamExt as _};
use serde::Serialize;
use sha2::{Digest as _, Sha256};
use tendermint::{
    abci::{transaction::Hash, Event},
    block::{self, Height},
    chain,
    consensus::Params,
    validator::Update,
};
use tendermint_rpc::Client;
use tokio::sync::watch;

use crate::{tx, venue::Venue};

pub struct File(String, Vec<u8>);

#[derive(Serialize)]
enum Stage {
    Extract,
}

#[derive(Serialize)]
struct Deadletter {
    stage: Stage,
    venue: Venue,
    height: Height,
    err: String,
}

#[allow(clippy::large_enum_variant)]
pub enum Extract {
    Deadletter(Venue, chain::Id, Height, Report),
    Block {
        venue: Venue,
        network: chain::Id,
        height: Height,
        block: block::Block,
        begin_block: Option<Vec<Event>>,
        end_block: Option<Vec<Event>>,
        validator_updates: Vec<Update>,
        consensus_param_updates: Option<Params>,
        txs: Vec<tx::Tx>,
    },
}

#[allow(clippy::needless_collect)]
#[allow(clippy::single_match)]
pub async fn extract<C>(
    mut shutdown: watch::Receiver<bool>,
    client: C,
    venue: Venue,
    network: chain::Id,
    height: block::Height,
) -> Result<Extract>
where
    C: Client + Clone + Send + Sync + 'static,
{
    let (block, results) = tokio::join!(client.block(height), client.block_results(height));

    match (block, results) {
        (Err(err), _) | (_, Err(err)) => Ok(Extract::Deadletter(
            venue,
            network.clone(),
            height,
            err!("failed to fetch block data: {:?}", err),
        )),
        (Ok(tendermint_rpc::endpoint::block::Response { block, .. }), Ok(res)) => {
            let height = block.header.height;
            let hashes = block
                .data
                .iter()
                .map(|body| {
                    Hash::new(
                        <[u8; 32]>::try_from(Sha256::digest(&body).deref())
                            .context("failed to hash transaction")
                            .unwrap(),
                    )
                })
                .collect::<Vec<_>>();

            let txs = stream::iter(hashes.into_iter())
                .take_until(async move {
                    shutdown.changed().await.ok();
                })
                .map(|hash| tokio::spawn(tx::get_info(client.clone(), hash)))
                .buffer_unordered(1)
                .try_collect::<Vec<_>>()
                .await
                .map_err(|err| err!("failed to complete task: {}", err))
                .map(|respones| respones.into_iter().collect::<Result<Vec<tx::Tx>, Error>>());

            match txs {
                Err(err) | Ok(Err(err)) => {
                    Ok(Extract::Deadletter(venue, network.clone(), height, err))
                },
                Ok(Ok(txs)) => Ok(Extract::Block {
                    venue,
                    network: network.clone(),
                    height: block.header.height,
                    block,
                    begin_block: res.begin_block_events,
                    end_block: res.end_block_events,
                    validator_updates: res.validator_updates,
                    consensus_param_updates: res.consensus_param_updates,
                    txs,
                }),
            }
        },
    }
}

pub async fn load<B>(mut shutdown: watch::Receiver<bool>, bucket: B, files: Vec<File>) -> Result<()>
where
    B: storage::Bucket + Send + Sync + 'static,
{
    stream::iter(files)
        .take_until(async move {
            shutdown.changed().await.ok();
        })
        .map(|File(filename, data)| tokio::spawn(insert(bucket.clone(), filename, data)))
        .buffer_unordered(1)
        .try_collect::<Vec<_>>()
        .await
        .map_err(|err| err!("failed to complete task: {}", err))
        .map(|respones| respones.into_iter().collect::<Result<(), Error>>())?
}

pub fn transform(binary: String, hostname: String, extract: Extract) -> Result<Vec<File>> {
    match extract {
        Extract::Deadletter(venue, network, height, err) => {
            let name = filename(
                binary,
                hostname,
                venue,
                network,
                format!("block-deadletters/{}.json", height),
            );
            let data = serde_json::to_vec(&Deadletter {
                stage: Stage::Extract,
                venue,
                height,
                err: err.to_string(),
            })
            .unwrap();

            Ok(vec![File(name, data)])
        },
        Extract::Block {
            venue,
            network,
            height,
            block,
            begin_block,
            end_block,
            validator_updates,
            consensus_param_updates,
            txs,
        } => {
            let mut files: Vec<File> = Vec::with_capacity(5 + txs.len());

            files.push(File(
                filename(
                    binary.clone(),
                    hostname.clone(),
                    venue,
                    network.clone(),
                    format!("blocks/{}/block.json", height),
                ),
                serde_json::to_vec(&block).unwrap(),
            ));
            files.push(File(
                filename(
                    binary.clone(),
                    hostname.clone(),
                    venue,
                    network.clone(),
                    format!("blocks/{}/begin_block.json", height),
                ),
                serde_json::to_vec(&begin_block).unwrap(),
            ));
            files.push(File(
                filename(
                    binary.clone(),
                    hostname.clone(),
                    venue,
                    network.clone(),
                    format!("blocks/{}/end_block.json", height),
                ),
                serde_json::to_vec(&end_block).unwrap(),
            ));
            files.push(File(
                filename(
                    binary.clone(),
                    hostname.clone(),
                    venue,
                    network.clone(),
                    format!("{}/validator_updates.json", height),
                ),
                serde_json::to_vec(&validator_updates).unwrap(),
            ));
            files.push(File(
                filename(
                    binary.clone(),
                    hostname.clone(),
                    venue,
                    network.clone(),
                    format!("{}/consensus_params_updates.json", height),
                ),
                serde_json::to_vec(&consensus_param_updates).unwrap(),
            ));

            for tx in txs {
                files.push(File(
                    filename(
                        binary.clone(),
                        hostname.clone(),
                        venue,
                        network.clone(),
                        format!("blocks/{}/txs/{}.json", height, tx.hash),
                    ),
                    serde_json::to_vec(&tx).unwrap(),
                ));
            }

            Ok(files)
        },
    }
}

fn filename(
    binary: String,
    hostname: String,
    venue: Venue,
    network: chain::Id,
    name: String,
) -> String {
    format!("{}/{}/{:?}/{}/{}", binary, hostname, venue, network, name)
}

async fn insert<B>(bucket: B, filename: String, data: Vec<u8>) -> Result<()>
where
    B: storage::Bucket,
{
    bucket
        .insert(
            filename.clone(),
            data.len().try_into().unwrap(),
            None,
            data.as_slice(),
        )
        .await
        .map(|_| ())
        .map_err(|err| err!("failed to insert into bucket: {}", err))
}

pub fn offset_height(
    current_height: &mut u64,
    latest_height: block::Height,
) -> impl Future<Output = Option<Option<block::Height>>> {
    // NB(xla): When collecting the block data from load-balanced services not
    // all backing nodes are necessarily caught up to the same height. To avoid
    // any inconsistencies the fetch is delayed.
    let res = if latest_height.value() <= 10 || latest_height.value() - 10 <= *current_height {
        Some(None)
    } else {
        *current_height = latest_height.value();
        Some(Some(latest_height))
    };
    async move { res }
}
