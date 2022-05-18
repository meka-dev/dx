use crate::{proto::osmosis::gamm, MsgProto};

impl MsgProto for gamm::v1beta1::MsgJoinPool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinPool";
}

impl MsgProto for gamm::v1beta1::MsgExitPool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgExitPool";
}

impl MsgProto for gamm::v1beta1::MsgSwapExactAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn";
}

impl MsgProto for gamm::v1beta1::MsgSwapExactAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgSwapExactAmountOut";
}

impl MsgProto for gamm::v1beta1::MsgJoinSwapExternAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn";
}

impl MsgProto for gamm::v1beta1::MsgJoinSwapShareAmountOut {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOut";
}

impl MsgProto for gamm::v1beta1::MsgExitSwapShareAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgExitSwapShareAmountIn";
}

impl MsgProto for gamm::v1beta1::MsgExitSwapExternAmountOut {
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
