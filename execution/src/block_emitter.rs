use anyhow::{Context as _, Result};
use prometheus_client::{
    encoding::text::Encode,
    metrics::{family::Family, gauge::Gauge},
};
use tendermint::{block::Height, Block};
use tendermint_rpc::event::{Event, EventData};
use tokio::sync::watch;

use common::metrics::Registry;

use crate::config;

pub trait Emitter {
    fn emit(
        &self,
        res: std::result::Result<Event, tendermint_rpc::Error>,
    ) -> Result<Option<Height>>;
}

pub struct Rpc {
    sender: watch::Sender<Block>,
}

impl Rpc {
    pub fn new(sender: watch::Sender<Block>) -> Self {
        Self { sender }
    }
}

impl Emitter for Rpc {
    fn emit(
        &self,
        res: std::result::Result<Event, tendermint_rpc::Error>,
    ) -> Result<Option<Height>> {
        let event = res.context("failed to extract event out of result")?;

        if let EventData::NewBlock {
            block: Some(block), ..
        } = event.data
        {
            let height = block.header().height;
            self.sender
                .send(block)
                .context("failed to update latest block")?;

            return Ok(Some(height));
        }

        Ok(None)
    }
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub struct LatestHeightLabels {
    pub version: String,
}

pub struct Observed<E> {
    latest_block_height: Family<LatestHeightLabels, Gauge>,
    next: E,
}

impl<E> Observed<E> {
    pub fn new(registry: &mut Registry, next: E) -> Self {
        let latest_block_height = Family::<LatestHeightLabels, Gauge>::default();
        registry.register(
            "latest_block_height",
            "Latest observed block height",
            Box::new(latest_block_height.clone()),
        );

        Self {
            latest_block_height,
            next,
        }
    }
}

impl<E> Emitter for Observed<E>
where
    E: Emitter,
{
    fn emit(
        &self,
        input: std::result::Result<Event, tendermint_rpc::Error>,
    ) -> Result<Option<Height>> {
        let res = self.next.emit(input);

        if let Ok(Some(height)) = res {
            self.latest_block_height
                .get_or_create(&LatestHeightLabels {
                    version: config::VERSION.to_string(),
                })
                .set(height.value());
        }

        res
    }
}
