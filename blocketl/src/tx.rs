use std::convert::TryFrom;

use chainrs::MsgProto;
use common::{err, tracing::error, Report, Result};
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
    Client,
    Order,
};

pub async fn get_info<C>(client: C, hash: Hash) -> Result<Tx>
where
    C: Client + Send + Sync,
{
    let query = Query::from(EventType::Tx).and_eq("tx.hash", hash.to_string());
    let res = client
        .tx_search(query, false, 1, 30, Order::Ascending)
        .await?;

    res.txs
        .first()
        .ok_or_else(|| err!("failed to get tx: {}", hash))
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

    messages: Vec<serde_json::Value>,
    memo: String,
    tiemout_height: Height,
    extension_options: Vec<()>,
    non_critical_extension_options: Vec<()>,
}

impl TryFrom<&Response> for Tx {
    type Error = Report;

    fn try_from(input: &Response) -> Result<Self, Self::Error> {
        let tx = cosmrs::Tx::from_bytes(input.tx.as_bytes())
            .map_err(|err| err!("failed to parse tx body: {}", err))?;

        Ok(Self {
            hash: input.hash,
            height: input.height,
            index: input.index,
            tx_result: DeliverTx::from(&input.tx_result),
            tx: input.tx.clone(),
            proof: input.proof.clone(),

            auth_info: tx.auth_info.into(),
            signatures: tx.signatures,

            messages: tx
                .body
                .messages
                .iter()
                .map(any_to_value)
                .collect::<Result<Vec<serde_json::Value>>>()?,
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

fn any_to_value(any: &cosmrs::Any) -> Result<serde_json::Value> {
    use chainrs::proto::{cosmos, ibc, osmosis};

    let res = match any.type_url.as_str() {
        cosmos::authz::v1beta1::MsgExec::TYPE_URL => {
            cosmos::authz::v1beta1::MsgExec::from_any(any)?.to_value()
        },
        cosmos::authz::v1beta1::MsgGrant::TYPE_URL => {
            cosmos::authz::v1beta1::MsgGrant::from_any(any)?.to_value()
        },
        cosmos::authz::v1beta1::MsgRevoke::TYPE_URL => {
            cosmos::authz::v1beta1::MsgRevoke::from_any(any)?.to_value()
        },

        cosmos::bank::v1beta1::MsgMultiSend::TYPE_URL => {
            cosmos::bank::v1beta1::MsgMultiSend::from_any(any)?.to_value()
        },
        cosmos::bank::v1beta1::MsgSend::TYPE_URL => {
            cosmos::bank::v1beta1::MsgSend::from_any(any)?.to_value()
        },

        cosmos::base::abci::v1beta1::MsgData::TYPE_URL => {
            cosmos::base::abci::v1beta1::MsgData::from_any(any)?.to_value()
        },

        cosmos::base::reflection::v2alpha1::MsgDescriptor::TYPE_URL => {
            cosmos::base::reflection::v2alpha1::MsgDescriptor::from_any(any)?.to_value()
        },

        cosmos::crisis::v1beta1::MsgVerifyInvariant::TYPE_URL => {
            cosmos::crisis::v1beta1::MsgVerifyInvariant::from_any(any)?.to_value()
        },

        cosmos::distribution::v1beta1::MsgFundCommunityPool::TYPE_URL => {
            cosmos::distribution::v1beta1::MsgFundCommunityPool::from_any(any)?.to_value()
        },
        cosmos::distribution::v1beta1::MsgSetWithdrawAddress::TYPE_URL => {
            cosmos::distribution::v1beta1::MsgSetWithdrawAddress::from_any(any)?.to_value()
        },
        cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward::TYPE_URL => {
            cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward::from_any(any)?.to_value()
        },
        cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission::TYPE_URL => {
            cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission::from_any(any)?.to_value()
        },

        cosmos::evidence::v1beta1::MsgSubmitEvidence::TYPE_URL => {
            cosmos::evidence::v1beta1::MsgSubmitEvidence::from_any(any)?.to_value()
        },

        cosmos::feegrant::v1beta1::MsgGrantAllowance::TYPE_URL => {
            cosmos::feegrant::v1beta1::MsgGrantAllowance::from_any(any)?.to_value()
        },
        cosmos::feegrant::v1beta1::MsgRevokeAllowance::TYPE_URL => {
            cosmos::feegrant::v1beta1::MsgRevokeAllowance::from_any(any)?.to_value()
        },

        cosmos::gov::v1beta1::MsgDeposit::TYPE_URL => {
            cosmos::gov::v1beta1::MsgDeposit::from_any(any)?.to_value()
        },
        cosmos::gov::v1beta1::MsgSubmitProposal::TYPE_URL => {
            cosmos::gov::v1beta1::MsgSubmitProposal::from_any(any)?.to_value()
        },
        cosmos::gov::v1beta1::MsgVote::TYPE_URL => {
            cosmos::gov::v1beta1::MsgVote::from_any(any)?.to_value()
        },
        cosmos::gov::v1beta1::MsgVoteWeighted::TYPE_URL => {
            cosmos::gov::v1beta1::MsgVoteWeighted::from_any(any)?.to_value()
        },

        cosmos::slashing::v1beta1::MsgUnjail::TYPE_URL => {
            cosmos::slashing::v1beta1::MsgUnjail::from_any(any)?.to_value()
        },

        cosmos::staking::v1beta1::MsgBeginRedelegate::TYPE_URL => {
            cosmos::staking::v1beta1::MsgBeginRedelegate::from_any(any)?.to_value()
        },
        cosmos::staking::v1beta1::MsgCreateValidator::TYPE_URL => {
            cosmos::staking::v1beta1::MsgCreateValidator::from_any(any)?.to_value()
        },
        cosmos::staking::v1beta1::MsgDelegate::TYPE_URL => {
            cosmos::staking::v1beta1::MsgDelegate::from_any(any)?.to_value()
        },
        cosmos::staking::v1beta1::MsgEditValidator::TYPE_URL => {
            cosmos::staking::v1beta1::MsgEditValidator::from_any(any)?.to_value()
        },
        cosmos::staking::v1beta1::MsgUndelegate::TYPE_URL => {
            cosmos::staking::v1beta1::MsgUndelegate::from_any(any)?.to_value()
        },

        cosmos::vesting::v1beta1::MsgCreateVestingAccount::TYPE_URL => {
            cosmos::vesting::v1beta1::MsgCreateVestingAccount::from_any(any)?.to_value()
        },

        ibc::applications::transfer::v1::MsgTransfer::TYPE_URL => {
            ibc::applications::transfer::v1::MsgTransfer::from_any(any)?.to_value()
        },

        ibc::core::channel::v1::MsgAcknowledgement::TYPE_URL => {
            ibc::core::channel::v1::MsgAcknowledgement::from_any(any)?.to_value()
        },
        ibc::core::channel::v1::MsgChannelCloseConfirm::TYPE_URL => {
            ibc::core::channel::v1::MsgChannelCloseConfirm::from_any(any)?.to_value()
        },
        ibc::core::channel::v1::MsgChannelCloseInit::TYPE_URL => {
            ibc::core::channel::v1::MsgChannelCloseInit::from_any(any)?.to_value()
        },
        ibc::core::channel::v1::MsgChannelOpenAck::TYPE_URL => {
            ibc::core::channel::v1::MsgChannelOpenAck::from_any(any)?.to_value()
        },
        ibc::core::channel::v1::MsgChannelOpenConfirm::TYPE_URL => {
            ibc::core::channel::v1::MsgChannelOpenConfirm::from_any(any)?.to_value()
        },
        ibc::core::channel::v1::MsgChannelOpenInit::TYPE_URL => {
            ibc::core::channel::v1::MsgChannelOpenInit::from_any(any)?.to_value()
        },
        ibc::core::channel::v1::MsgChannelOpenTry::TYPE_URL => {
            ibc::core::channel::v1::MsgChannelOpenTry::from_any(any)?.to_value()
        },
        ibc::core::channel::v1::MsgRecvPacket::TYPE_URL => {
            ibc::core::channel::v1::MsgRecvPacket::from_any(any)?.to_value()
        },
        ibc::core::channel::v1::MsgTimeout::TYPE_URL => {
            ibc::core::channel::v1::MsgTimeout::from_any(any)?.to_value()
        },
        ibc::core::channel::v1::MsgTimeoutOnClose::TYPE_URL => {
            ibc::core::channel::v1::MsgTimeoutOnClose::from_any(any)?.to_value()
        },

        ibc::core::client::v1::MsgCreateClient::TYPE_URL => {
            ibc::core::client::v1::MsgCreateClient::from_any(any)?.to_value()
        },
        ibc::core::client::v1::MsgSubmitMisbehaviour::TYPE_URL => {
            ibc::core::client::v1::MsgSubmitMisbehaviour::from_any(any)?.to_value()
        },
        ibc::core::client::v1::MsgUpdateClient::TYPE_URL => {
            ibc::core::client::v1::MsgUpdateClient::from_any(any)?.to_value()
        },
        ibc::core::client::v1::MsgUpgradeClient::TYPE_URL => {
            ibc::core::client::v1::MsgUpgradeClient::from_any(any)?.to_value()
        },

        ibc::core::connection::v1::MsgConnectionOpenAck::TYPE_URL => {
            ibc::core::connection::v1::MsgConnectionOpenAck::from_any(any)?.to_value()
        },
        ibc::core::connection::v1::MsgConnectionOpenConfirm::TYPE_URL => {
            ibc::core::connection::v1::MsgConnectionOpenConfirm::from_any(any)?.to_value()
        },
        ibc::core::connection::v1::MsgConnectionOpenInit::TYPE_URL => {
            ibc::core::connection::v1::MsgConnectionOpenInit::from_any(any)?.to_value()
        },
        ibc::core::connection::v1::MsgConnectionOpenTry::TYPE_URL => {
            ibc::core::connection::v1::MsgConnectionOpenTry::from_any(any)?.to_value()
        },

        osmosis::gamm::v1beta1::MsgJoinPool::TYPE_URL => {
            osmosis::gamm::v1beta1::MsgJoinPool::from_any(any)?.to_value()
        },
        osmosis::gamm::v1beta1::MsgExitPool::TYPE_URL => {
            osmosis::gamm::v1beta1::MsgExitPool::from_any(any)?.to_value()
        },
        osmosis::gamm::v1beta1::MsgSwapExactAmountIn::TYPE_URL => {
            osmosis::gamm::v1beta1::MsgSwapExactAmountIn::from_any(any)?.to_value()
        },
        osmosis::gamm::v1beta1::MsgSwapExactAmountOut::TYPE_URL => {
            osmosis::gamm::v1beta1::MsgSwapExactAmountOut::from_any(any)?.to_value()
        },
        osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn::TYPE_URL => {
            osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn::from_any(any)?.to_value()
        },
        osmosis::gamm::v1beta1::MsgJoinSwapShareAmountOut::TYPE_URL => {
            osmosis::gamm::v1beta1::MsgJoinSwapShareAmountOut::from_any(any)?.to_value()
        },
        osmosis::gamm::v1beta1::MsgExitSwapShareAmountIn::TYPE_URL => {
            osmosis::gamm::v1beta1::MsgExitSwapShareAmountIn::from_any(any)?.to_value()
        },
        osmosis::gamm::v1beta1::MsgExitSwapExternAmountOut::TYPE_URL => {
            osmosis::gamm::v1beta1::MsgExitSwapExternAmountOut::from_any(any)?.to_value()
        },

        osmosis::incentives::MsgAddToGauge::TYPE_URL => {
            osmosis::incentives::MsgAddToGauge::from_any(any)?.to_value()
        },
        osmosis::incentives::MsgCreateGauge::TYPE_URL => {
            osmosis::incentives::MsgCreateGauge::from_any(any)?.to_value()
        },

        osmosis::lockup::MsgBeginUnlocking::TYPE_URL => {
            osmosis::lockup::MsgBeginUnlocking::from_any(any)?.to_value()
        },
        osmosis::lockup::MsgBeginUnlockingAll::TYPE_URL => {
            osmosis::lockup::MsgBeginUnlockingAll::from_any(any)?.to_value()
        },
        // osmosis::lockup::MsgExtendLockup::TYPE_URL => {
        //     osmosis::lockup::MsgExtendLockup::from_any(any)?.to_value()
        // },
        osmosis::lockup::MsgLockTokens::TYPE_URL => {
            osmosis::lockup::MsgLockTokens::from_any(any)?.to_value()
        },

        osmosis::superfluid::MsgLockAndSuperfluidDelegate::TYPE_URL => {
            osmosis::superfluid::MsgLockAndSuperfluidDelegate::from_any(any)?.to_value()
        },
        osmosis::superfluid::MsgSuperfluidDelegate::TYPE_URL => {
            osmosis::superfluid::MsgSuperfluidDelegate::from_any(any)?.to_value()
        },
        osmosis::superfluid::MsgSuperfluidUnbondLock::TYPE_URL => {
            osmosis::superfluid::MsgSuperfluidUnbondLock::from_any(any)?.to_value()
        },
        osmosis::superfluid::MsgSuperfluidUndelegate::TYPE_URL => {
            osmosis::superfluid::MsgSuperfluidUndelegate::from_any(any)?.to_value()
        },
        osmosis::superfluid::MsgUnPoolWhitelistedPool::TYPE_URL => {
            osmosis::superfluid::MsgUnPoolWhitelistedPool::from_any(any)?.to_value()
        },

        // osmosis::tokenfactory::v1beta1::MsgBurn::TYPE_URL => {
        //     osmosis::tokenfactory::v1beta1::MsgBurn::from_any(any)?.to_value()
        // },
        // osmosis::tokenfactory::v1beta1::MsgChangeAdmin::TYPE_URL => {
        //     osmosis::tokenfactory::v1beta1::MsgChangeAdmin::from_any(any)?.to_value()
        // },
        // osmosis::tokenfactory::v1beta1::MsgCreateDenom::TYPE_URL => {
        //     osmosis::tokenfactory::v1beta1::MsgCreateDenom::from_any(any)?.to_value()
        // },
        // osmosis::tokenfactory::v1beta1::MsgMint::TYPE_URL => {
        //     osmosis::tokenfactory::v1beta1::MsgMint::from_any(any)?.to_value()
        // },
        _ => {
            error!("type_url={}", any.type_url);
            Err(err!("unknown message: {}", any.type_url))
        },
    };

    Ok(res.unwrap())
}
