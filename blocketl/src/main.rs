use std::time::Duration;

use axum::{Router, Server};
use common::{
    err,
    metrics,
    params,
    signals,
    storage,
    tracing::{self, error},
    Result,
};
use config::{Backfill, Mode};
use futures::{stream, FutureExt as _, StreamExt as _};
use tendermint::block;
use tendermint_rpc::{Client, HttpClient};
use tokio::{select, sync::watch, time};
use tokio_stream::wrappers::IntervalStream;

mod config;
mod stage;
mod tx;
mod venue;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init();

    let (args, args_routes) = argh::from_env::<config::Args>().router();

    let (shutdown_sender, shutdown) = watch::channel(false);
    let mut signals_handle = tokio::spawn(signals::routine(shutdown_sender)).fuse();

    let mut registry = metrics::Registry::new(config::BINARY);

    let bucket = storage::Observed::new(
        &mut registry,
        storage::Gcs::new(args.bucket, args.service_account, config::USER_AGENT)
            .map_err(|err| err!("failed to setup storage bucket: {}", err))?,
    );

    let router = Router::new()
        .nest("/params", params::routes(&mut registry, args_routes))
        .nest("/metrics", metrics::routes(registry, config::BINARY));
    let mut op_server_handle =
        tokio::spawn(Server::bind(&args.op_addr).serve(router.into_make_service())).fuse();

    let hostname = common::host().ok_or_else(|| err!("failed to get hostname"))?;
    let client = HttpClient::new(args.rpc_url.as_str())?;

    let heights = match args.mode {
        Mode::Backfill(Backfill {
            start_height,
            end_height,
        }) => stream::iter(start_height.value()..end_height.value())
            .map(|val| block::Height::try_from(val).unwrap())
            .boxed(),
        Mode::Realtime(_) => {
            let client = client.clone();
            IntervalStream::new(time::interval(Duration::from_secs(1)))
                .map(move |_| client.clone())
                .filter_map(|client| async move {
                    match client.status().await {
                        Ok(status) => Some(status.sync_info.latest_block_height),
                        Err(err) => {
                            error!("failed to fetch status: {:?}", err);
                            None
                        },
                    }
                })
                .scan(0, stage::offset_height)
                .filter_map(|height| async move { height })
                .boxed()
        },
    };
    let mut etl_handle = tokio::spawn(
        heights
            .take_until({
                let mut shutdown = shutdown.clone();
                async move {
                    shutdown.changed().await.ok();
                }
            })
            .map(move |height| {
                let bucket = bucket.clone();
                let client = client.clone();
                let hostname = hostname.clone();
                let network = args.chain_id.clone();
                let shutdown = shutdown.clone();
                tokio::spawn(async move {
                    let ex = stage::extract(shutdown.clone(), client, args.venue, network, height)
                        .await?;
                    let files = stage::transform(config::BINARY.to_owned(), hostname, ex)?;
                    stage::load(shutdown, bucket, files).await
                })
            })
            .buffer_unordered(args.max_parallel_requests as usize)
            .for_each(|join| async move {
                match join {
                    Ok(Err(err)) => error!("failed to store: {:?}", err),
                    Err(err) => error!("failed to execute etl task: {:?}", err),
                    _ => {},
                }
            }),
    )
    .fuse();

    loop {
        select! {
            join = &mut signals_handle => {
                join??;
                break;
            },
            join = &mut op_server_handle => join??,
            join = &mut etl_handle => join?,

            else => break
        }
    }

    Ok(())
}
