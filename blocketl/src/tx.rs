use std::convert::TryFrom;

use anyhow::anyhow;
use cosmrs::tx::SignatureBytes;
use serde::Serialize;
use tendermint::{
    abci::{self, responses, transaction::Hash, Code, Data, Gas, Info, Log},
    block::{self, Height},
};
use tendermint_proto::types::TxProof;
use tendermint_rpc::{
    endpoint::tx::Response,
    query::{EventType, Query},
    Client as _,
    Order,
    WebSocketClient,
};

pub async fn get_info(client: WebSocketClient, hash: Hash) -> anyhow::Result<Tx> {
    let query = Query::from(EventType::Tx).and_eq("tx.hash", hash.to_string());
    let res = client
        .tx_search(query, false, 1, 30, Order::Ascending)
        .await?;

    res.txs
        .first()
        .ok_or_else(|| anyhow!("failed to get tx: {}", hash))
        .and_then(Tx::try_from)
}

#[derive(Serialize)]
pub struct Tx {
    pub hash: Hash,
    height: block::Height,
    index: u32,
    tx_result: DeliverTx,
    tx: abci::Transaction,
    proof: Option<TxProof>,

    auth_info: AuthInfo,
    signatures: Vec<SignatureBytes>,

    messages: Vec<()>,
    memo: String,
    tiemout_height: Height,
    extension_options: Vec<()>,
    non_critical_extension_options: Vec<()>,
}

impl TryFrom<&Response> for Tx {
    type Error = anyhow::Error;

    fn try_from(input: &Response) -> Result<Self, Self::Error> {
        let tx = cosmrs::Tx::from_bytes(input.tx.as_bytes())
            .map_err(|err| anyhow!("failed to parse tx body: {}", err))?;

        Ok(Self {
            hash: input.hash,
            height: input.height,
            index: input.index,
            tx_result: DeliverTx::from(&input.tx_result),
            tx: input.tx.clone(),
            proof: input.proof.clone(),

            auth_info: tx.auth_info.into(),
            signatures: tx.signatures,

            messages: vec![],
            memo: tx.body.memo,
            tiemout_height: tx.body.timeout_height,
            extension_options: vec![],
            non_critical_extension_options: vec![],
        })
    }
}

#[derive(Serialize)]
pub struct DeliverTx {
    code: Code,
    data: Data,
    log: Log,
    info: Info,
    gas_wanted: Gas,
    gas_used: Gas,
    events: Vec<Event>,
    codespace: responses::Codespace,
}

impl From<&responses::DeliverTx> for DeliverTx {
    fn from(input: &responses::DeliverTx) -> Self {
        Self {
            code: input.code,
            data: input.data.clone(),
            log: input.log.clone(),
            info: input.info.clone(),
            gas_wanted: input.gas_wanted,
            gas_used: input.gas_used,
            events: input.events.iter().map(Event::from).collect::<Vec<_>>(),
            codespace: input.codespace.clone(),
        }
    }
}

#[derive(Serialize)]
struct Event {
    #[serde(rename = "type")]
    type_str: String,
    attributes: Vec<Tag>,
}

impl From<&responses::Event> for Event {
    fn from(input: &responses::Event) -> Self {
        Self {
            type_str: input.type_str.clone(),
            attributes: input.attributes.iter().map(Tag::from).collect::<Vec<_>>(),
        }
    }
}

#[derive(Serialize)]
struct Tag {
    key: String,
    value: String,
}

impl From<&abci::tag::Tag> for Tag {
    fn from(input: &abci::tag::Tag) -> Self {
        Self {
            key: input.key.as_ref().to_owned(),
            value: input.value.as_ref().to_owned(),
        }
    }
}

#[derive(Serialize)]
struct AuthInfo {
    signer_infos: Vec<()>,
    fee: cosmrs::tx::Fee,
}

impl From<cosmrs::tx::AuthInfo> for AuthInfo {
    fn from(input: cosmrs::tx::AuthInfo) -> Self {
        Self {
            signer_infos: vec![],
            fee: input.fee,
        }
    }
}
