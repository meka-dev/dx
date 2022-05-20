use axum::{Router, Server};
use common::{err, metrics, params, storage, tracing, Result};
use futures::FutureExt as _;
use hyperlocal::SocketIncoming;
use tokio::{select, sync::mpsc};

use crate::{config::USER_AGENT, segment::Segment};

mod api;
mod config;
mod emitter;
mod error;
mod event;
mod ingester;
mod namespace;
mod producer;
mod segment;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init();

    let (args, args_routes) = argh::from_env::<config::Args>().router();

    let (ingest_sender, ingest_receiver) = mpsc::channel::<Segment>(args.ingest_queue_depth);
    let (flush_sender, flush_receiver) = mpsc::channel::<Segment>(args.flush_queue_depth);
    let mut registry = metrics::Registry::new(config::BINARY);
    let bucket = storage::Observed::new(
        &mut registry,
        storage::Gcs::new(args.bucket.clone(), args.service_account, USER_AGENT)?,
    );

    let api = api::Api::new(&mut registry);
    let mut api_handle =
        tokio::spawn(api.bind(SocketIncoming::bind(args.api_path)?, ingest_sender)).fuse();

    let ingester = ingester::Observed::new(
        &mut registry,
        ingester::InMemory::new(
            args.segment_byte_limit,
            args.segment_count_limit,
            args.flush_interval,
        ),
    );
    let mut ingester_handle = tokio::spawn(ingester::run(
        args.flush_interval,
        ingester,
        ingest_receiver,
        flush_sender,
    ))
    .fuse();

    let mut emitter_handle = tokio::spawn(emitter::run(
        emitter::Observed::new(
            &mut registry,
            args.bucket,
            emitter::Gcs::new(
                config::BINARY.to_string(),
                common::host().ok_or_else(|| err!("failed to get hostname"))?,
                bucket,
            ),
        ),
        flush_receiver,
    ))
    .fuse();

    let router = Router::new()
        .nest("/params", params::routes(&mut registry, args_routes))
        .nest("/metrics", metrics::routes(registry, config::BINARY));
    let mut op_server_handle =
        tokio::spawn(Server::bind(&args.op_addr).serve(router.into_make_service())).fuse();

    loop {
        select! {
            join = &mut api_handle => join??,
            join = &mut ingester_handle => join??,
            join = &mut op_server_handle => join??,
            join = &mut emitter_handle => join??,

            else => break,
        }
    }

    Ok(())
}
