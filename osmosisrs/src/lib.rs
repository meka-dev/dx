pub mod gamm;
pub mod incentives;
pub mod lockup;
pub mod superfluid;
pub mod tokenfactory;

pub mod proto {
    pub const OSMOSIS_VERSION: &str = include_str!("proto/OSMOSIS_COMMIT");

    pub mod cosmos {
        /// Authentication of accounts and transactions.
        pub mod auth {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.auth.v1beta1.rs");
            }
        }

        /// Authentication to execute on behalf of a granter.
        pub mod authz {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.authz.v1beta1.rs");
            }
        }

        /// Authentication to spend transaction fees using granter's balance.
        pub mod feegrant {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.feegrant.v1beta1.rs");
            }
        }

        /// Balances.
        pub mod bank {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.bank.v1beta1.rs");
            }
        }

        /// Base functionality.
        pub mod base {
            /// Application BlockChain Interface (ABCI).
            ///
            /// Interface that defines the boundary between the replication engine
            /// (the blockchain), and the state machine (the application).
            pub mod abci {
                pub mod v1beta1 {
                    use serde::{Deserialize, Serialize};
                    include!("proto/cosmos.base.abci.v1beta1.rs");
                }
            }

            /// Key-value pairs.
            pub mod kv {
                pub mod v1beta1 {
                    use serde::{Deserialize, Serialize};
                    include!("proto/cosmos.base.kv.v1beta1.rs");
                }
            }

            /// Query support.
            pub mod query {
                pub mod v1beta1 {
                    use serde::{Deserialize, Serialize};
                    include!("proto/cosmos.base.query.v1beta1.rs");
                }
            }

            /// Reflection support.
            pub mod reflection {
                pub mod v1beta1 {
                    use serde::{Deserialize, Serialize};
                    include!("proto/cosmos.base.reflection.v1beta1.rs");
                }
            }

            /// Snapshots containing Tendermint state sync info.
            pub mod snapshots {
                pub mod v1beta1 {
                    use serde::{Deserialize, Serialize};
                    include!("proto/cosmos.base.snapshots.v1beta1.rs");
                }
            }

            /// Data structure that holds the state of the application.
            pub mod store {
                pub mod v1beta1 {
                    use serde::{Deserialize, Serialize};
                    include!("proto/cosmos.base.store.v1beta1.rs");
                }
            }

            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.base.v1beta1.rs");
            }

            pub mod tendermint {
                pub mod v1beta1 {
                    use serde::{Deserialize, Serialize};
                    include!("proto/cosmos.base.tendermint.v1beta1.rs");
                }
            }
        }

        /// Crisis handling
        pub mod crisis {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.crisis.v1beta1.rs");
            }
        }

        /// Cryptographic primitives.
        pub mod crypto {
            /// Multi-signature support.
            pub mod multisig {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.crypto.multisig.rs");
                pub mod v1beta1 {
                    use serde::{Deserialize, Serialize};
                    include!("proto/cosmos.crypto.multisig.v1beta1.rs");
                }
            }
            pub mod ed25519 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.crypto.ed25519.rs");
            }
            pub mod secp256k1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.crypto.secp256k1.rs");
            }
        }

        /// Messages and services handling token distribution
        pub mod distribution {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.distribution.v1beta1.rs");
            }
        }

        /// Messages and services handling evidence
        pub mod evidence {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.evidence.v1beta1.rs");
            }
        }

        /// Messages and services handling gentx's
        pub mod genutil {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.genutil.v1beta1.rs");
            }
        }

        /// Messages and services handling governance
        pub mod gov {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.gov.v1beta1.rs");
            }
        }

        /// Messages and services handling minting
        pub mod mint {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.mint.v1beta1.rs");
            }
        }

        /// Messages and services handling chain parameters
        pub mod params {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.params.v1beta1.rs");
            }
        }

        /// Handling slashing parameters and unjailing
        pub mod slashing {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.slashing.v1beta1.rs");
            }
        }

        /// Proof-of-Stake layer for public blockchains.
        pub mod staking {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.staking.v1beta1.rs");
            }
        }

        /// Transactions.
        pub mod tx {
            /// Transaction signing support.
            pub mod signing {
                pub mod v1beta1 {
                    use serde::{Deserialize, Serialize};
                    include!("proto/cosmos.tx.signing.v1beta1.rs");
                }
            }

            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.tx.v1beta1.rs");
            }
        }

        /// Services for the upgrade module.
        pub mod upgrade {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.upgrade.v1beta1.rs");
            }
        }

        /// Services and tx's for the vesting module.
        pub mod vesting {
            pub mod v1beta1 {
                use serde::{Deserialize, Serialize};
                include!("proto/cosmos.vesting.v1beta1.rs");
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
