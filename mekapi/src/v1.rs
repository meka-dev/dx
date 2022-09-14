use std::net::IpAddr;

use async_trait::async_trait;
use common::Result;
use cosmrs::{tendermint::block, Tx};
use cosmwasm_std::Addr;
use futures::stream::BoxStream;
use serde::{de::Error as _, Deserialize, Deserializer};
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

    async fn v1_p2p_txhash(&self, tx_hash: String) -> Result<TxP2POriginsResponse>;
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
    #[serde(deserialize_with = "vec_from_str")]
    pub tx: Vec<u8>,
    pub pools: Vec<super::pools::PoolV5>,
}

#[derive(Debug, Deserialize)]
pub struct TxP2POriginsResponse {
    pub hash: String,
    pub found: bool,
    pub first: String,
    pub delta: String,
    pub all: Vec<TxP2POrigin>,
}

#[derive(Debug, Deserialize)]
pub struct TxP2POrigin {
    pub hash: String,
    pub ts: String,
    pub peer_id: String,
    pub peer_ip: IpAddr,
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

fn vec_from_str<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let input: String = Deserialize::deserialize(deserializer)?;
    hex::decode(input).map_err(D::Error::custom)
}
