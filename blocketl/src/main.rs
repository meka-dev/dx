use axum::{Router, Server};
use common::{err, metrics, params, signals, storage, tracing, Result};
use futures::FutureExt as _;
use tendermint_rpc::HttpClient;
use tokio::{select, sync::mpsc};

mod config;
mod routines;
mod tx;
mod venue;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init();

    let (args, args_routes) = argh::from_env::<config::Args>().router();

    let (shutdown_sender, shutdown_receiver) = mpsc::channel::<()>(1);
    let mut signals_handle = tokio::spawn(signals::routine(shutdown_sender)).fuse();

    let mut registry = metrics::Registry::new(config::BINARY);

    let bucket = storage::Observed::new(
        &mut registry,
        storage::Gcs::new(args.bucket, args.service_account, config::USER_AGENT)?,
    );

    let (heights_sender, heights_receiver) = mpsc::channel(1024);
    let (extracts_sender, extracts_receiver) = mpsc::channel(1024);
    let (files_sender, files_receiver) = mpsc::channel(4096);

    let client = HttpClient::new(args.rpc_url.as_str())?;

    let router = Router::new()
        .nest("/params", params::routes(&mut registry, args_routes))
        .nest("/metrics", metrics::routes(registry, config::BINARY));
    let mut op_server_handle =
        tokio::spawn(Server::bind(&args.op_addr).serve(router.into_make_service())).fuse();

    let mut latest_height_handle =
        tokio::spawn(routines::latest_height(client.clone(), heights_sender)).fuse();
    let mut extract_handle = tokio::spawn(routines::extract(
        client.clone(),
        heights_receiver,
        extracts_sender,
        args.venue,
        args.chain_id,
        args.max_parallel_requests,
    ))
    .fuse();
    let mut transform_handle = tokio::spawn(routines::transform(
        config::BINARY.to_owned(),
        common::host().ok_or_else(|| err!("failed to get hostname"))?,
        extracts_receiver,
        files_sender,
    ))
    .fuse();
    let mut load_handle = tokio::spawn(routines::load(
        bucket,
        files_receiver,
        shutdown_receiver,
        args.max_parallel_requests,
    ))
    .fuse();

    loop {
        select! {
            join = &mut signals_handle => {
                join??;
                break;
            },
            join = &mut op_server_handle => join??,

            join = &mut latest_height_handle => join??,
            join = &mut extract_handle => join??,
            join = &mut transform_handle => join??,
            join = &mut load_handle => join??,

            else => break
        }
    }

    Ok(())
}
