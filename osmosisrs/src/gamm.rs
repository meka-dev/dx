use cosmrs::tx::MsgProto;

use crate::proto;

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgExitPool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Msg/ExitPool";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgExitSwapShareAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Msg/SwapExternAmountIn";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgExitSwapExternAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Msg/SwapExternAmountOut";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgJoinPool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Msg/JoinPool";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Msg/JoinSwapExternAmountIn";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgJoinSwapShareAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Msg/JoinSwapShareAmountOut";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgSwapExactAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Msg/SwapExactAmountIn";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgSwapExactAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Msg/SwapExactAmountOut";
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
