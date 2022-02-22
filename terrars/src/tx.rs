use anyhow::Result;
use cosmrs::{
    crypto::secp256k1::SigningKey,
    tendermint::{block, chain},
    tx::{Body, Fee, Gas, SignDoc, SignerInfo},
};
use serde::{Deserialize, Serialize};

use crate::{
    base::{self, Coin},
    Message,
};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct ComputeTaxRequest {
    pub tx_bytes: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub(crate) struct ComputeTaxResponse {
    pub tax_amount: Vec<Coin>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SimulateRequest {
    pub tx_bytes: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SimulateResponse {
    pub gas_info: base::abci::GasInfo,
    pub result: base::abci::Result,
}

pub fn sign(
    chain_id: chain::Id,
    private_key: &SigningKey,
    account_number: u64,
    sequence: u64,
    amount: Coin,
    gas_limit: impl Into<Gas>,
    timeout_height: impl Into<block::Height>,
    msgs: Vec<Message>,
) -> Result<Vec<u8>> {
    let body = Body::new(msgs, "", timeout_height);
    let signer_info = SignerInfo::single_direct(Some(private_key.public_key()), sequence);
    let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(amount.into(), gas_limit));
    let doc = SignDoc::new(&body, &auth_info, &chain_id, account_number)
        .map_err(|err| anyhow::anyhow!("SignDoc creation failed: {}", err))?;
    let signed = doc
        .sign(&private_key)
        .map_err(|err| anyhow::anyhow!("signign failed: {}", err))?;
    Ok(signed
        .to_bytes()
        .map_err(|err| anyhow::anyhow!("conversion to bytes failed: {}", err))?)
}
