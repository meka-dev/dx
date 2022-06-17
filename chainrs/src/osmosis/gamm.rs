use common::{Report, Result};
use serde::{Deserialize, Serialize};

use crate::{
    cosmos::base::Coin,
    proto::osmosis::gamm,
    tx::{Msg, MsgProto},
    MsgProto as ChainMsgProto,
};

impl MsgProto for gamm::v1beta1::MsgJoinPool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinPool";
}

impl ChainMsgProto for gamm::v1beta1::MsgJoinPool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinPool";
}

impl MsgProto for gamm::v1beta1::MsgExitPool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgExitPool";
}

impl ChainMsgProto for gamm::v1beta1::MsgExitPool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgExitPool";
}

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SwapAmountInRoute {
    pub pool_id: String,
    pub token_out_denom: String,
}

impl From<SwapAmountInRoute> for crate::proto::osmosis::gamm::v1beta1::SwapAmountInRoute {
    fn from(s: SwapAmountInRoute) -> Self {
        crate::proto::osmosis::gamm::v1beta1::SwapAmountInRoute {
            pool_id: s.pool_id.parse::<u64>().unwrap(), // unwrap :|
            token_out_denom: s.token_out_denom,
        }
    }
}

impl From<crate::proto::osmosis::gamm::v1beta1::SwapAmountInRoute> for SwapAmountInRoute {
    fn from(s: crate::proto::osmosis::gamm::v1beta1::SwapAmountInRoute) -> Self {
        SwapAmountInRoute {
            pool_id: s.pool_id.to_string(),
            token_out_denom: s.token_out_denom,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MsgSwapExactAmountIn {
    pub sender: String,
    pub routes: Vec<SwapAmountInRoute>,
    pub token_in: Option<Coin>,
    pub token_out_min_amount: String,
}

impl From<MsgSwapExactAmountIn> for crate::proto::osmosis::gamm::v1beta1::MsgSwapExactAmountIn {
    fn from(s: MsgSwapExactAmountIn) -> Self {
        crate::proto::osmosis::gamm::v1beta1::MsgSwapExactAmountIn {
            sender: s.sender,
            routes: s.routes.into_iter().map(From::from).collect::<Vec<_>>(),
            token_in: s.token_in.map(Into::into),
            token_out_min_amount: s.token_out_min_amount,
        }
    }
}

impl TryFrom<crate::proto::osmosis::gamm::v1beta1::MsgSwapExactAmountIn> for MsgSwapExactAmountIn {
    type Error = Report;

    fn try_from(s: crate::proto::osmosis::gamm::v1beta1::MsgSwapExactAmountIn) -> Result<Self> {
        Ok(MsgSwapExactAmountIn {
            sender: s.sender,
            routes: s
                .routes
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            token_in: s.token_in.and_then(|t| TryFrom::try_from(t).ok()),
            token_out_min_amount: s.token_out_min_amount,
        })
    }
}

impl Msg for MsgSwapExactAmountIn {
    type Proto = crate::proto::osmosis::gamm::v1beta1::MsgSwapExactAmountIn;
}

impl MsgProto for gamm::v1beta1::MsgSwapExactAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn";
}

impl ChainMsgProto for gamm::v1beta1::MsgSwapExactAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn";
}

impl MsgProto for gamm::v1beta1::MsgSwapExactAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgSwapExactAmountOut";
}

impl ChainMsgProto for gamm::v1beta1::MsgSwapExactAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgSwapExactAmountOut";
}

impl MsgProto for gamm::v1beta1::MsgJoinSwapExternAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn";
}

impl ChainMsgProto for gamm::v1beta1::MsgJoinSwapExternAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn";
}

impl MsgProto for gamm::v1beta1::MsgJoinSwapShareAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOut";
}

impl ChainMsgProto for gamm::v1beta1::MsgJoinSwapShareAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOut";
}

impl MsgProto for gamm::v1beta1::MsgExitSwapShareAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgExitSwapShareAmountIn";
}

impl ChainMsgProto for gamm::v1beta1::MsgExitSwapShareAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgExitSwapShareAmountIn";
}

impl MsgProto for gamm::v1beta1::MsgExitSwapExternAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgExitSwapExternAmountOut";
}

impl ChainMsgProto for gamm::v1beta1::MsgExitSwapExternAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgExitSwapExternAmountOut";
}

pub mod poolmodels {
    pub mod balancer {
        use cosmrs::tx::MsgProto;

        use crate::proto;

        impl MsgProto for proto::osmosis::gamm::poolmodels::balancer::v1beta1::MsgCreateBalancerPool {
            const TYPE_URL: &'static str =
                "/osmosis.gamm.poolmodels.balancer.v1beta1.Msg/CreateBalancerPool";
        }
    }

    pub mod stableswap {
        use cosmrs::tx::MsgProto;

        use crate::proto;

        impl MsgProto for proto::osmosis::gamm::poolmodels::stableswap::v1beta1::MsgCreateStableswapPool {
            const TYPE_URL: &'static str =
                "/osmosis.gamm.poolmodels.stableswap.v1beta1.Msg/CreateStableswapPool";
        }
    }
}
