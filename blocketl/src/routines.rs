use std::{collections::BTreeMap, ops::Deref as _};

use common::{
    err,
    storage,
    tracing::{debug, error},
    Context as _,
    Error,
    Result,
};
use futures::stream::{self, StreamExt as _, TryStreamExt as _};
use serde::Serialize;
use sha2::{Digest as _, Sha256};
use tendermint::{
    abci::{self, transaction::Hash},
    block::{self, Height},
    chain,
};
use tendermint_rpc::{
    event::{Event, EventData},
    query::{EventType, Query},
    SubscriptionClient as _,
    WebSocketClient,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::{tx, venue::Venue};

pub type Events = Option<BTreeMap<String, Vec<String>>>;

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
    Deadletter(Venue, chain::Id, Height, Error),
    Block {
        venue: Venue,
        network: chain::Id,
        height: Height,
        block: block::Block,
        begin_block: Option<abci::responses::BeginBlock>,
        end_block: Option<abci::responses::EndBlock>,
        events: Events,
        txs: Vec<tx::Tx>,
    },
}

#[allow(clippy::needless_collect)]
#[allow(clippy::single_match)]
pub async fn extract(
    client: WebSocketClient,
    sender: mpsc::Sender<Extract>,
    venue: Venue,
    network: chain::Id,
    max_parallel_requests: u64,
) -> Result<()> {
    debug!("init extract");

    let query = Query::from(EventType::NewBlock);
    let mut blocks = client.subscribe(query).await?;

    while let Some(res) = blocks.next().await {
        match res {
            Ok(Event { data, events, .. }) => match data {
                EventData::NewBlock {
                    block: Some(block),
                    result_begin_block,
                    result_end_block,
                } => {
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

                    let res = stream::iter(hashes.into_iter())
                        .map(|hash| tokio::spawn(tx::get_info(client.clone(), hash)))
                        .buffer_unordered(max_parallel_requests as usize)
                        .try_collect::<Vec<_>>()
                        .await
                        .map_err(|err| err!("failed to complete task: {}", err))
                        .map(|respones| {
                            respones.into_iter().collect::<Result<Vec<tx::Tx>, Error>>()
                        });

                    let msg = match res {
                        Err(err) | Ok(Err(err)) => {
                            Extract::Deadletter(venue, network.clone(), height, err)
                        },
                        Ok(Ok(txs)) => Extract::Block {
                            venue,
                            network: network.clone(),
                            height: block.header.height,
                            block,
                            begin_block: result_begin_block,
                            end_block: result_end_block,
                            events,
                            txs,
                        },
                    };

                    if let Err(err) = sender.send(msg).await {
                        error!("failed to send latest block: {}", err);
                    }
                },
                _ => {},
            },
            Err(err) => error!("failed to unpack event form subscription: {}", err),
        }
    }

    Ok(())
}

pub async fn load<B>(
    bucket: B,
    files: mpsc::Receiver<File>,
    mut shutdown: mpsc::Receiver<()>,
    max_parallel_requests: u64,
) -> Result<()>
where
    B: storage::Bucket + Send + Sync + 'static,
{
    debug!("init load");

    ReceiverStream::new(files)
        .take_until(async move {
            shutdown.recv().await;
            println!("shutdown done");
        })
        .map(|File(filename, data)| tokio::spawn(insert(bucket.clone(), filename, data)))
        .buffer_unordered(max_parallel_requests as usize)
        .for_each(|join| async {
            match join {
                Ok(Err(err)) => error!("failed to store: {:?}", err),
                Err(err) => error!("failed to execute task: {:?}", err),
                _ => {},
            }
        })
        .await;

    println!("load stream terminated");

    Ok(())
}

#[allow(clippy::needless_collect)]
pub async fn transform(
    binary: String,
    hostname: String,
    mut extracts: mpsc::Receiver<Extract>,
    files: mpsc::Sender<File>,
) -> Result<()> {
    debug!("init transform");

    while let Some(extract) = extracts.recv().await {
        let msgs = match extract {
            Extract::Deadletter(venue, network, height, err) => {
                let name = filename(
                    binary.clone(),
                    hostname.clone(),
                    venue,
                    network,
                    format!("deadletters/{}.json", height),
                );
                let data = serde_json::to_vec(&Deadletter {
                    stage: Stage::Extract,
                    venue,
                    height,
                    err: err.to_string(),
                })
                .unwrap();

                vec![File(name, data)]
            },
            Extract::Block {
                venue,
                network,
                height,
                block,
                begin_block,
                end_block,
                events,
                txs,
            } => {
                let mut msgs: Vec<File> = Vec::with_capacity(4 + txs.len());

                msgs.push(File(
                    filename(
                        binary.clone(),
                        hostname.clone(),
                        venue,
                        network.clone(),
                        format!("{}/block.json", height),
                    ),
                    serde_json::to_vec(&block).unwrap(),
                ));
                msgs.push(File(
                    filename(
                        binary.clone(),
                        hostname.clone(),
                        venue,
                        network.clone(),
                        format!("{}/begin_block.json", height),
                    ),
                    serde_json::to_vec(&begin_block).unwrap(),
                ));
                msgs.push(File(
                    filename(
                        binary.clone(),
                        hostname.clone(),
                        venue,
                        network.clone(),
                        format!("{}/end_block.json", height),
                    ),
                    serde_json::to_vec(&end_block).unwrap(),
                ));
                msgs.push(File(
                    filename(
                        binary.clone(),
                        hostname.clone(),
                        venue,
                        network.clone(),
                        format!("{}/events.json", height),
                    ),
                    serde_json::to_vec(&events).unwrap(),
                ));

                for tx in txs {
                    msgs.push(File(
                        filename(
                            binary.clone(),
                            hostname.clone(),
                            venue,
                            network.clone(),
                            format!("{}/txs/{}.json", height, tx.hash),
                        ),
                        serde_json::to_vec(&tx).unwrap(),
                    ));
                }

                msgs
            },
        };

        for m in msgs {
            if let Err(err) = files.send(m).await {
                error!("failed to send {}: {}", err.0 .0, err);
            }
        }
    }

    Ok(())
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
}
