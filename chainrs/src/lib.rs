use std::str::FromStr;

use common::{err, Result};
pub use cosmrs::tendermint;
pub use prost_types::Any;
use serde::Serialize;

pub mod json;
mod prost_ext;

pub mod cosmos;
pub mod ibc;
pub mod osmosis;
pub mod tx;

#[allow(clippy::derive_partial_eq_without_eq)]
pub mod proto {
    pub const OSMOSIS_VERSION: &str = include_str!("proto/OSMOSIS_COMMIT");

    pub mod cosmos {
        /// Authentication of accounts and transactions.
        pub mod auth {
            pub mod v1beta1 {
                include!("proto/cosmos.auth.v1beta1.rs");
            }
        }

        /// Granting of arbitrary privileges from one account to another.
        pub mod authz {
            pub mod v1beta1 {
                include!("proto/cosmos.authz.v1beta1.rs");
            }
        }

        /// Balances.
        pub mod bank {
            pub mod v1beta1 {
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
                    include!("proto/cosmos.base.abci.v1beta1.rs");
                }
            }

            /// Key-value pairs.
            pub mod kv {
                pub mod v1beta1 {
                    include!("proto/cosmos.base.kv.v1beta1.rs");
                }
            }

            /// Query support.
            pub mod query {
                pub mod v1beta1 {
                    include!("proto/cosmos.base.query.v1beta1.rs");
                }
            }

            /// Reflection support.
            pub mod reflection {
                pub mod v1beta1 {
                    include!("proto/cosmos.base.reflection.v1beta1.rs");
                }

                pub mod v2alpha1 {
                    include!("proto/cosmos.base.reflection.v2alpha1.rs");
                }
            }

            /// Snapshots containing Tendermint state sync info.
            pub mod snapshots {
                pub mod v1beta1 {
                    include!("proto/cosmos.base.snapshots.v1beta1.rs");
                }
            }

            /// Data structure that holds the state of the application.
            pub mod store {
                pub mod v1beta1 {
                    include!("proto/cosmos.base.store.v1beta1.rs");
                }
            }

            pub mod v1beta1 {
                include!("proto/cosmos.base.v1beta1.rs");
            }

            pub mod tendermint {
                pub mod v1beta1 {
                    include!("proto/cosmos.base.tendermint.v1beta1.rs");
                }
            }
        }

        /// Crisis handling
        pub mod crisis {
            pub mod v1beta1 {
                include!("proto/cosmos.crisis.v1beta1.rs");
            }
        }

        /// Cryptographic primitives.
        pub mod crypto {
            /// Multi-signature support.
            pub mod multisig {
                include!("proto/cosmos.crypto.multisig.rs");
                pub mod v1beta1 {
                    include!("proto/cosmos.crypto.multisig.v1beta1.rs");
                }
            }
            pub mod ed25519 {
                include!("proto/cosmos.crypto.ed25519.rs");
            }
            pub mod secp256k1 {
                include!("proto/cosmos.crypto.secp256k1.rs");
            }
            pub mod secp256r1 {
                include!("proto/cosmos.crypto.secp256r1.rs");
            }
        }

        /// Messages and services handling token distribution
        pub mod distribution {
            pub mod v1beta1 {
                include!("proto/cosmos.distribution.v1beta1.rs");
            }
        }

        /// Messages and services handling evidence
        pub mod evidence {
            pub mod v1beta1 {
                include!("proto/cosmos.evidence.v1beta1.rs");
            }
        }

        /// Allows accounts to grant fee allowances and to use fees from their accounts.
        pub mod feegrant {
            pub mod v1beta1 {
                include!("proto/cosmos.feegrant.v1beta1.rs");
            }
        }

        /// Messages and services handling gentx's
        pub mod genutil {
            pub mod v1beta1 {
                include!("proto/cosmos.genutil.v1beta1.rs");
            }
        }

        /// Messages and services handling governance
        pub mod gov {
            pub mod v1beta1 {
                include!("proto/cosmos.gov.v1beta1.rs");
            }
        }

        /// Messages and services handling minting
        pub mod mint {
            pub mod v1beta1 {
                include!("proto/cosmos.mint.v1beta1.rs");
            }
        }

        /// Messages and services handling chain parameters
        pub mod params {
            pub mod v1beta1 {
                include!("proto/cosmos.params.v1beta1.rs");
            }
        }

        /// Handling slashing parameters and unjailing
        pub mod slashing {
            pub mod v1beta1 {
                include!("proto/cosmos.slashing.v1beta1.rs");
            }
        }

        /// Proof-of-Stake layer for public blockchains.
        pub mod staking {
            pub mod v1beta1 {
                include!("proto/cosmos.staking.v1beta1.rs");
            }
        }

        /// Transactions.
        pub mod tx {
            /// Transaction signing support.
            pub mod signing {
                pub mod v1beta1 {
                    include!("proto/cosmos.tx.signing.v1beta1.rs");
                }
            }

            pub mod v1beta1 {
                include!("proto/cosmos.tx.v1beta1.rs");
            }
        }

        /// Services for the upgrade module.
        pub mod upgrade {
            pub mod v1beta1 {
                include!("proto/cosmos.upgrade.v1beta1.rs");
            }
        }

        /// Services and tx's for the vesting module.
        pub mod vesting {
            pub mod v1beta1 {
                include!("proto/cosmos.vesting.v1beta1.rs");
            }
        }
    }

    /// IBC protobuf definitions.
    pub mod ibc {
        pub mod applications {
            pub mod transfer {
                pub mod v1 {
                    include!("proto/ibc.applications.transfer.v1.rs");
                }

                pub mod v2 {
                    include!("proto/ibc.applications.transfer.v2.rs");
                }
            }
        }

        pub mod core {
            pub mod channel {
                pub mod v1 {
                    include!("proto/ibc.core.channel.v1.rs");
                }
            }

            pub mod client {
                pub mod v1 {
                    include!("proto/ibc.core.client.v1.rs");
                }
            }

            pub mod commitment {
                pub mod v1 {
                    include!("proto/ibc.core.commitment.v1.rs");
                }
            }

            pub mod connection {
                pub mod v1 {
                    include!("proto/ibc.core.connection.v1.rs");
                }
            }

            pub mod port {
                pub mod v1 {
                    include!("proto/ibc.core.port.v1.rs");
                }
            }

            pub mod types {
                pub mod v1 {
                    include!("proto/ibc.core.types.v1.rs");
                }
            }
        }

        pub mod lightclients {
            pub mod localhost {
                pub mod v1 {
                    include!("proto/ibc.lightclients.localhost.v1.rs");
                }
            }

            pub mod solomachine {
                pub mod v1 {
                    include!("proto/ibc.lightclients.solomachine.v1.rs");
                }

                pub mod v2 {
                    include!("proto/ibc.lightclients.solomachine.v2.rs");
                }
            }

            pub mod tendermint {
                pub mod v1 {
                    include!("proto/ibc.lightclients.tendermint.v1.rs");
                }
            }
        }
    }

    pub mod ics23 {
        include!("proto/ics23.rs");
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

        // pub mod tokenfactory {
        //     pub mod v1beta1 {
        //         include!("proto/osmosis.tokenfactory.v1beta1.rs");
        //     }
        // }

        pub mod txfees {
            pub mod v1beta1 {
                include!("proto/osmosis.txfees.v1beta1.rs");
            }
        }
    }
}

/// Proto types which can be used as a [`Msg`].
pub trait MsgProto: MessageExt + Serialize + Default + Sized {
    /// Type URL value
    const TYPE_URL: &'static str;

    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> Result<Self> {
        if any.type_url == Self::TYPE_URL {
            Ok(Self::decode(&*any.value)?)
        } else {
            Err(cosmrs::Error::MsgType {
                expected: Self::TYPE_URL,
                found: any.type_url.clone(),
            }
            .into())
        }
    }

    /// Serialize this message proto as [`Any`].
    fn to_any(&self) -> Result<Any> {
        self.to_bytes().map(|bytes| Any {
            type_url: Self::TYPE_URL.to_owned(),
            value: bytes,
        })
    }

    fn to_value(&self) -> Result<serde_json::Value> {
        serde_json::to_value(self).map_err(|err| err!("failed to transform to value: {}", err))
    }
}

/// Extension trait for prost messages.
pub trait MessageExt: prost::Message {
    /// Serialize this protobuf message as a byte vector.
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

impl<M> MessageExt for M
where
    M: prost::Message,
{
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        prost::Message::encode(self, &mut bytes)?;
        Ok(bytes)
    }
}

/// Extension traits for optionally parsing non-empty strings.
///
/// This is a common pattern in Cosmos SDK protobufs.
pub trait ParseOptional: AsRef<str> {
    /// Parse optional field.
    fn parse_optional<T: FromStr>(&self) -> Result<Option<T>, T::Err> {
        if self.as_ref().is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.as_ref().parse()?))
        }
    }
}

impl ParseOptional for str {}
impl ParseOptional for String {}
