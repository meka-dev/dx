use anyhow::Result;
use async_trait::async_trait;
use cosmrs::{tendermint::block, Tx};
use cosmwasm_std::Addr;
use futures::stream::BoxStream;
use serde::{de::Error as _, Deserialize, Deserializer};

use crate::dex::Pool;

#[async_trait]
pub trait Api: Send + Sync {
    async fn v1_pool_responses(&self, contract: Addr, height: block::Height) -> Result<Vec<Pool>>;

    async fn v1_events_mempooltx(&self) -> Result<BoxStream<'static, Result<MempoolTxMessage>>>;

    async fn v1_events_mempooltx_v2(
        &self,
    ) -> Result<BoxStream<'static, Result<MempoolTxV2Message>>>;
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

// point.
fn tx_from_str<'de, D>(deserializer: D) -> Result<Tx, D::Error>
where
    D: Deserializer<'de>,
{
    let input: String = Deserialize::deserialize(deserializer)?;
    let bytes = hex::decode(input).map_err(D::Error::custom)?;

    Tx::from_bytes(&bytes).map_err(D::Error::custom)
}
