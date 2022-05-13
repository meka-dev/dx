pub mod gamm;
pub mod incentives;
pub mod lockup;
pub mod superfluid;
pub mod tokenfactory;

pub mod proto {
    pub const OSMOSIS_VERSION: &str = include_str!("proto/OSMOSIS_COMMIT");

    pub mod cosmos {
        pub mod base {
            /// Query support.
            pub mod query {
                pub mod v1beta1 {
                    include!("proto/cosmos.base.query.v1beta1.rs");
                }
            }

            pub mod v1beta1 {
                include!("proto/cosmos.base.v1beta1.rs");
            }
        }
    }

    /// Osmosis protobuf definitions.
    pub mod osmosis {
        pub mod epochs {
            pub mod v1beta1 {
                include!("proto/osmosis.epochs.v1beta1.rs");
            }
        }

        pub mod gamm {
            pub mod poolmodels {
                pub mod balancer {
                    pub mod v1beta1 {
                        include!("proto/osmosis.gamm.poolmodels.balancer.v1beta1.rs");
                    }
                }

                pub mod stableswap {
                    pub mod v1beta1 {
                        include!("proto/osmosis.gamm.poolmodels.stableswap.v1beta1.rs");
                    }
                }
            }

            pub mod v1beta1 {
                include!("proto/osmosis.gamm.v1beta1.rs");
            }
        }

        pub mod incentives {
            include!("proto/osmosis.incentives.rs");
        }

        pub mod lockup {
            include!("proto/osmosis.lockup.rs");
        }

        pub mod mint {
            pub mod v1beta1 {
                include!("proto/osmosis.mint.v1beta1.rs");
            }
        }

        pub mod poolincentives {
            pub mod v1beta1 {
                include!("proto/osmosis.poolincentives.v1beta1.rs");
            }
        }

        pub mod store {
            pub mod v1beta1 {
                include!("proto/osmosis.store.v1beta1.rs");
            }
        }

        pub mod superfluid {
            include!("proto/osmosis.superfluid.rs");

            pub mod v1beta1 {
                include!("proto/osmosis.superfluid.v1beta1.rs");
            }
        }

        pub mod tokenfactory {
            pub mod v1beta1 {
                include!("proto/osmosis.tokenfactory.v1beta1.rs");
            }
        }

        pub mod txfees {
            pub mod v1beta1 {
                include!("proto/osmosis.txfees.v1beta1.rs");
            }
        }
    }
}
