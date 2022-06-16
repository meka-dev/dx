use async_trait::async_trait;
use common::Result;
use cosmrs::{tendermint::block, Tx};
use cosmwasm_std::Addr;
use futures::stream::BoxStream;
use serde::{de::Error as _, Deserialize, Deserializer};
use strategy::Pool;
use terrars::dex::Pool as TerraPool;

#[async_trait]
pub trait Api: Send + Sync {
    async fn v1_pool_responses(
        &self,
        contract: Addr,
        height: block::Height,
    ) -> Result<Vec<TerraPool>>;

    async fn v1_events_mempooltx(&self) -> Result<BoxStream<'static, Result<MempoolTxMessage>>>;

    async fn v1_events_mempooltx_v2(
        &self,
    ) -> Result<BoxStream<'static, Result<MempoolTxV2Message>>>;

    async fn v1_events_mempooltx_v5(
        &self,
    ) -> Result<BoxStream<'static, Result<MempoolTxV5Message>>>;
}

#[derive(Debug, Deserialize)]
pub struct MempoolTxMessage {
    pub height: block::Height,
    #[serde(deserialize_with = "tx_from_str")]
    pub tx: Tx,
}

#[derive(Debug, Deserialize)]
pub struct MempoolTxV2Message {
    pub height: block::Height,
    #[serde(deserialize_with = "tx_from_str")]
    pub tx: Tx,
    pub pools: Vec<super::v3::Pool>,
}

#[derive(Debug, Deserialize)]
pub struct MempoolTxV5Message {
    pub height: block::Height,
    #[serde(rename = "txhash")]
    pub tx_hash: String,
    #[serde(deserialize_with = "tx_from_str")]
    pub tx: Tx,
    pub pools: Vec<super::pools::PoolV5>,
}

pub trait HydratedMempoolMessage<M: TryInto<Pool>> {
    fn pools(&self) -> Vec<M>;
    fn height(&self) -> block::Height;
}

impl HydratedMempoolMessage<super::pools::PoolV5> for MempoolTxV5Message {
    fn pools(&self) -> Vec<super::pools::PoolV5> {
        self.pools.clone()
    }

    fn height(&self) -> block::Height {
        self.height
    }
}

impl HydratedMempoolMessage<super::v3::Pool> for MempoolTxV2Message {
    fn pools(&self) -> Vec<super::v3::Pool> {
        self.pools.clone()
    }

    fn height(&self) -> block::Height {
        self.height
    }
}

// point.
fn tx_from_str<'de, D>(deserializer: D) -> Result<Tx, D::Error>
where
    D: Deserializer<'de>,
{
    let input: String = Deserialize::deserialize(deserializer)?;
    let bytes = hex::decode(input).map_err(D::Error::custom)?;

    Tx::from_bytes(&bytes).map_err(D::Error::custom)
}
