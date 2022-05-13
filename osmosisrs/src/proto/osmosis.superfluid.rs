/// SuperfluidAsset stores the pair of superfluid asset type and denom pair
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidAsset {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(enumeration="SuperfluidAssetType", tag="2")]
    pub asset_type: i32,
}
/// SuperfluidIntermediaryAccount takes the role of intermediary between LP token
/// and OSMO tokens for superfluid staking
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidIntermediaryAccount {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub val_addr: ::prost::alloc::string::String,
    /// perpetual gauge for rewards distribution
    #[prost(uint64, tag="3")]
    pub gauge_id: u64,
}
/// The Osmo-Equivalent-Multiplier Record for epoch N refers to the osmo worth we
/// treat an LP share as having, for all of epoch N. Eventually this is intended
/// to be set as the Time-weighted-average-osmo-backing for the entire duration
/// of epoch N-1. (Thereby locking whats in use for epoch N as based on the prior
/// epochs rewards) However for now, this is not the TWAP but instead the spot
/// price at the boundary.  For different types of assets in the future, it could
/// change.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsmoEquivalentMultiplierRecord {
    #[prost(int64, tag="1")]
    pub epoch_number: i64,
    /// superfluid asset denom, can be LP token or native token
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub multiplier: ::prost::alloc::string::String,
}
/// SuperfluidDelegationRecord takes the role of intermediary between LP token
/// and OSMO tokens for superfluid staking
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationRecord {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub delegation_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockIdIntermediaryAccountConnection {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub intermediary_account: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SuperfluidAssetType {
    Native = 0,
    /// SuperfluidAssetTypeLendingShare = 2; // for now not exist
    LpShare = 1,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidDelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
    #[prost(string, tag="3")]
    pub val_addr: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidDelegateResponse {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUndelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUndelegateResponse {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUnbondLock {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUnbondLockResponse {
}
// message MsgSuperfluidRedelegate {
//   string sender = 1 [ (gogoproto.moretags) = "yaml:\"sender\"" ];
//   uint64 lock_id = 2;
//   string new_val_addr = 3;
// }
// message MsgSuperfluidRedelegateResponse {}

/// MsgLockAndSuperfluidDelegate locks coins with the unbonding period duration,
/// and then does a superfluid lock from the newly created lockup, to the
/// specified validator addr.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockAndSuperfluidDelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub val_addr: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockAndSuperfluidDelegateResponse {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Msg defines the Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Execute superfluid delegation for a lockup
        pub async fn superfluid_delegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSuperfluidDelegate>,
        ) -> Result<
                tonic::Response<super::MsgSuperfluidDelegateResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/SuperfluidDelegate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Execute superfluid undelegation for a lockup
        pub async fn superfluid_undelegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSuperfluidUndelegate>,
        ) -> Result<
                tonic::Response<super::MsgSuperfluidUndelegateResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/SuperfluidUndelegate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// For a given lock that is being superfluidly undelegated,
        /// also unbond the underlying lock.
        pub async fn superfluid_unbond_lock(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSuperfluidUnbondLock>,
        ) -> Result<
                tonic::Response<super::MsgSuperfluidUnbondLockResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/SuperfluidUnbondLock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Execute lockup lock and superfluid delegation in a single msg
        pub async fn lock_and_superfluid_delegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLockAndSuperfluidDelegate>,
        ) -> Result<
                tonic::Response<super::MsgLockAndSuperfluidDelegateResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/LockAndSuperfluidDelegate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Params holds parameters for the superfluid module
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// the risk_factor is to be cut on OSMO equivalent value of lp tokens for
    /// superfluid staking, default: 5%
    #[prost(string, tag="1")]
    pub minimum_risk_factor: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetTypeRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetTypeResponse {
    #[prost(enumeration="SuperfluidAssetType", tag="1")]
    pub asset_type: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllAssetsRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllAssetsResponse {
    #[prost(message, repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<SuperfluidAsset>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetMultiplierRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetMultiplierResponse {
    #[prost(message, optional, tag="1")]
    pub osmo_equivalent_multiplier: ::core::option::Option<OsmoEquivalentMultiplierRecord>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidIntermediaryAccountInfo {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub val_addr: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub gauge_id: u64,
    #[prost(string, tag="4")]
    pub address: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllIntermediaryAccountsRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllIntermediaryAccountsResponse {
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<SuperfluidIntermediaryAccountInfo>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectedIntermediaryAccountRequest {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectedIntermediaryAccountResponse {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<SuperfluidIntermediaryAccountInfo>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSuperfluidDelegationsRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSuperfluidDelegationsResponse {
    #[prost(string, tag="1")]
    pub total_delegations: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationAmountRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationAmountResponse {
    #[prost(message, repeated, tag="1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByDelegatorRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByDelegatorResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
    #[prost(message, repeated, tag="2")]
    pub total_delegated_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidUndelegationsByDelegatorRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidUndelegationsByDelegatorResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
    #[prost(message, repeated, tag="2")]
    pub total_undelegated_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="3")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<super::lockup::SyntheticLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByValidatorDenomRequest {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByValidatorDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSuperfluidDelegatedAmountByValidatorDenomRequest {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSuperfluidDelegatedAmountByValidatorDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub total_delegated_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Params returns the total set of minting parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/Params",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns superfluid asset type
        pub async fn asset_type(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetTypeRequest>,
        ) -> Result<tonic::Response<super::AssetTypeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/AssetType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all superfluid asset types
        pub async fn all_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::AllAssetsRequest>,
        ) -> Result<tonic::Response<super::AllAssetsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/AllAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns superfluid asset Multiplier
        pub async fn asset_multiplier(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetMultiplierRequest>,
        ) -> Result<tonic::Response<super::AssetMultiplierResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/AssetMultiplier",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all superfluid intermediary account
        pub async fn all_intermediary_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::AllIntermediaryAccountsRequest>,
        ) -> Result<
                tonic::Response<super::AllIntermediaryAccountsResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/AllIntermediaryAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns intermediary account connected to a superfluid staked lock by id
        pub async fn connected_intermediary_account(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectedIntermediaryAccountRequest>,
        ) -> Result<
                tonic::Response<super::ConnectedIntermediaryAccountResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/ConnectedIntermediaryAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the total amount of osmo superfluidly staked
        /// response denominated in uosmo
        pub async fn total_superfluid_delegations(
            &mut self,
            request: impl tonic::IntoRequest<super::TotalSuperfluidDelegationsRequest>,
        ) -> Result<
                tonic::Response<super::TotalSuperfluidDelegationsResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/TotalSuperfluidDelegations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the coins superfluid delegated for a delegator, validator, denom
        /// triplet
        pub async fn superfluid_delegation_amount(
            &mut self,
            request: impl tonic::IntoRequest<super::SuperfluidDelegationAmountRequest>,
        ) -> Result<
                tonic::Response<super::SuperfluidDelegationAmountResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/SuperfluidDelegationAmount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all the superfluid poistions for a specific delegator
        pub async fn superfluid_delegations_by_delegator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SuperfluidDelegationsByDelegatorRequest,
            >,
        ) -> Result<
                tonic::Response<super::SuperfluidDelegationsByDelegatorResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/SuperfluidDelegationsByDelegator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn superfluid_undelegations_by_delegator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SuperfluidUndelegationsByDelegatorRequest,
            >,
        ) -> Result<
                tonic::Response<super::SuperfluidUndelegationsByDelegatorResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/SuperfluidUndelegationsByDelegator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all the superfluid positions of a specific denom delegated to one
        /// validator
        pub async fn superfluid_delegations_by_validator_denom(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SuperfluidDelegationsByValidatorDenomRequest,
            >,
        ) -> Result<
                tonic::Response<super::SuperfluidDelegationsByValidatorDenomResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/SuperfluidDelegationsByValidatorDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the amount of a specific denom delegated to a specific validator
        /// This is labeled an estimate, because the way it calculates the amount can
        /// lead rounding errors from the true delegated amount
        pub async fn estimate_superfluid_delegated_amount_by_validator_denom(
            &mut self,
            request: impl tonic::IntoRequest<
                super::EstimateSuperfluidDelegatedAmountByValidatorDenomRequest,
            >,
        ) -> Result<
                tonic::Response<
                    super::EstimateSuperfluidDelegatedAmountByValidatorDenomResponse,
                >,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/EstimateSuperfluidDelegatedAmountByValidatorDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the module's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub superfluid_assets: ::prost::alloc::vec::Vec<SuperfluidAsset>,
    #[prost(message, repeated, tag="3")]
    pub osmo_equivalent_multipliers: ::prost::alloc::vec::Vec<OsmoEquivalentMultiplierRecord>,
    #[prost(message, repeated, tag="4")]
    pub intermediary_accounts: ::prost::alloc::vec::Vec<SuperfluidIntermediaryAccount>,
    #[prost(message, repeated, tag="5")]
    pub intemediary_account_connections: ::prost::alloc::vec::Vec<LockIdIntermediaryAccountConnection>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_ASSET : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidAsset")] impl :: prost_wkt :: MessageSerde for SuperfluidAsset { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidAsset" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidAsset" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_INTERMEDIARY_ACCOUNT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidIntermediaryAccount")] impl :: prost_wkt :: MessageSerde for SuperfluidIntermediaryAccount { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidIntermediaryAccount" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidIntermediaryAccount" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_OSMO_EQUIVALENT_MULTIPLIER_RECORD : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.OsmoEquivalentMultiplierRecord")] impl :: prost_wkt :: MessageSerde for OsmoEquivalentMultiplierRecord { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "OsmoEquivalentMultiplierRecord" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.OsmoEquivalentMultiplierRecord" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_DELEGATION_RECORD : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationRecord")] impl :: prost_wkt :: MessageSerde for SuperfluidDelegationRecord { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidDelegationRecord" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationRecord" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LOCK_ID_INTERMEDIARY_ACCOUNT_CONNECTION : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.LockIdIntermediaryAccountConnection")] impl :: prost_wkt :: MessageSerde for LockIdIntermediaryAccountConnection { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "LockIdIntermediaryAccountConnection" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.LockIdIntermediaryAccountConnection" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_SUPERFLUID_DELEGATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.MsgSuperfluidDelegate")] impl :: prost_wkt :: MessageSerde for MsgSuperfluidDelegate { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "MsgSuperfluidDelegate" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.MsgSuperfluidDelegate" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_SUPERFLUID_DELEGATE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.MsgSuperfluidDelegateResponse")] impl :: prost_wkt :: MessageSerde for MsgSuperfluidDelegateResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "MsgSuperfluidDelegateResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.MsgSuperfluidDelegateResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_SUPERFLUID_UNDELEGATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.MsgSuperfluidUndelegate")] impl :: prost_wkt :: MessageSerde for MsgSuperfluidUndelegate { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "MsgSuperfluidUndelegate" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.MsgSuperfluidUndelegate" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_SUPERFLUID_UNDELEGATE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.MsgSuperfluidUndelegateResponse")] impl :: prost_wkt :: MessageSerde for MsgSuperfluidUndelegateResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "MsgSuperfluidUndelegateResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.MsgSuperfluidUndelegateResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_SUPERFLUID_UNBOND_LOCK : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.MsgSuperfluidUnbondLock")] impl :: prost_wkt :: MessageSerde for MsgSuperfluidUnbondLock { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "MsgSuperfluidUnbondLock" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.MsgSuperfluidUnbondLock" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_SUPERFLUID_UNBOND_LOCK_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.MsgSuperfluidUnbondLockResponse")] impl :: prost_wkt :: MessageSerde for MsgSuperfluidUnbondLockResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "MsgSuperfluidUnbondLockResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.MsgSuperfluidUnbondLockResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_LOCK_AND_SUPERFLUID_DELEGATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.MsgLockAndSuperfluidDelegate")] impl :: prost_wkt :: MessageSerde for MsgLockAndSuperfluidDelegate { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "MsgLockAndSuperfluidDelegate" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.MsgLockAndSuperfluidDelegate" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_LOCK_AND_SUPERFLUID_DELEGATE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.MsgLockAndSuperfluidDelegateResponse")] impl :: prost_wkt :: MessageSerde for MsgLockAndSuperfluidDelegateResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "MsgLockAndSuperfluidDelegateResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.MsgLockAndSuperfluidDelegateResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PARAMS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.Params")] impl :: prost_wkt :: MessageSerde for Params { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "Params" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.Params" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.QueryParamsRequest")] impl :: prost_wkt :: MessageSerde for QueryParamsRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "QueryParamsRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.QueryParamsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.QueryParamsResponse")] impl :: prost_wkt :: MessageSerde for QueryParamsResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "QueryParamsResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.QueryParamsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ASSET_TYPE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.AssetTypeRequest")] impl :: prost_wkt :: MessageSerde for AssetTypeRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "AssetTypeRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.AssetTypeRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ASSET_TYPE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.AssetTypeResponse")] impl :: prost_wkt :: MessageSerde for AssetTypeResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "AssetTypeResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.AssetTypeResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ALL_ASSETS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.AllAssetsRequest")] impl :: prost_wkt :: MessageSerde for AllAssetsRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "AllAssetsRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.AllAssetsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ALL_ASSETS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.AllAssetsResponse")] impl :: prost_wkt :: MessageSerde for AllAssetsResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "AllAssetsResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.AllAssetsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ASSET_MULTIPLIER_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.AssetMultiplierRequest")] impl :: prost_wkt :: MessageSerde for AssetMultiplierRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "AssetMultiplierRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.AssetMultiplierRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ASSET_MULTIPLIER_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.AssetMultiplierResponse")] impl :: prost_wkt :: MessageSerde for AssetMultiplierResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "AssetMultiplierResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.AssetMultiplierResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_INTERMEDIARY_ACCOUNT_INFO : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidIntermediaryAccountInfo")] impl :: prost_wkt :: MessageSerde for SuperfluidIntermediaryAccountInfo { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidIntermediaryAccountInfo" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidIntermediaryAccountInfo" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ALL_INTERMEDIARY_ACCOUNTS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.AllIntermediaryAccountsRequest")] impl :: prost_wkt :: MessageSerde for AllIntermediaryAccountsRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "AllIntermediaryAccountsRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.AllIntermediaryAccountsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ALL_INTERMEDIARY_ACCOUNTS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.AllIntermediaryAccountsResponse")] impl :: prost_wkt :: MessageSerde for AllIntermediaryAccountsResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "AllIntermediaryAccountsResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.AllIntermediaryAccountsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CONNECTED_INTERMEDIARY_ACCOUNT_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.ConnectedIntermediaryAccountRequest")] impl :: prost_wkt :: MessageSerde for ConnectedIntermediaryAccountRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "ConnectedIntermediaryAccountRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.ConnectedIntermediaryAccountRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CONNECTED_INTERMEDIARY_ACCOUNT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.ConnectedIntermediaryAccountResponse")] impl :: prost_wkt :: MessageSerde for ConnectedIntermediaryAccountResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "ConnectedIntermediaryAccountResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.ConnectedIntermediaryAccountResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TOTAL_SUPERFLUID_DELEGATIONS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.TotalSuperfluidDelegationsRequest")] impl :: prost_wkt :: MessageSerde for TotalSuperfluidDelegationsRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "TotalSuperfluidDelegationsRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.TotalSuperfluidDelegationsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TOTAL_SUPERFLUID_DELEGATIONS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.TotalSuperfluidDelegationsResponse")] impl :: prost_wkt :: MessageSerde for TotalSuperfluidDelegationsResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "TotalSuperfluidDelegationsResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.TotalSuperfluidDelegationsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_DELEGATION_AMOUNT_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationAmountRequest")] impl :: prost_wkt :: MessageSerde for SuperfluidDelegationAmountRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidDelegationAmountRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationAmountRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_DELEGATION_AMOUNT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationAmountResponse")] impl :: prost_wkt :: MessageSerde for SuperfluidDelegationAmountResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidDelegationAmountResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationAmountResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_DELEGATIONS_BY_DELEGATOR_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationsByDelegatorRequest")] impl :: prost_wkt :: MessageSerde for SuperfluidDelegationsByDelegatorRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidDelegationsByDelegatorRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationsByDelegatorRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_DELEGATIONS_BY_DELEGATOR_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationsByDelegatorResponse")] impl :: prost_wkt :: MessageSerde for SuperfluidDelegationsByDelegatorResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidDelegationsByDelegatorResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationsByDelegatorResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_UNDELEGATIONS_BY_DELEGATOR_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidUndelegationsByDelegatorRequest")] impl :: prost_wkt :: MessageSerde for SuperfluidUndelegationsByDelegatorRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidUndelegationsByDelegatorRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidUndelegationsByDelegatorRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_UNDELEGATIONS_BY_DELEGATOR_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidUndelegationsByDelegatorResponse")] impl :: prost_wkt :: MessageSerde for SuperfluidUndelegationsByDelegatorResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidUndelegationsByDelegatorResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidUndelegationsByDelegatorResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_DELEGATIONS_BY_VALIDATOR_DENOM_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationsByValidatorDenomRequest")] impl :: prost_wkt :: MessageSerde for SuperfluidDelegationsByValidatorDenomRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidDelegationsByValidatorDenomRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationsByValidatorDenomRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SUPERFLUID_DELEGATIONS_BY_VALIDATOR_DENOM_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationsByValidatorDenomResponse")] impl :: prost_wkt :: MessageSerde for SuperfluidDelegationsByValidatorDenomResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "SuperfluidDelegationsByValidatorDenomResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.SuperfluidDelegationsByValidatorDenomResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ESTIMATE_SUPERFLUID_DELEGATED_AMOUNT_BY_VALIDATOR_DENOM_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.EstimateSuperfluidDelegatedAmountByValidatorDenomRequest")] impl :: prost_wkt :: MessageSerde for EstimateSuperfluidDelegatedAmountByValidatorDenomRequest { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "EstimateSuperfluidDelegatedAmountByValidatorDenomRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.EstimateSuperfluidDelegatedAmountByValidatorDenomRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ESTIMATE_SUPERFLUID_DELEGATED_AMOUNT_BY_VALIDATOR_DENOM_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.EstimateSuperfluidDelegatedAmountByValidatorDenomResponse")] impl :: prost_wkt :: MessageSerde for EstimateSuperfluidDelegatedAmountByValidatorDenomResponse { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "EstimateSuperfluidDelegatedAmountByValidatorDenomResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.EstimateSuperfluidDelegatedAmountByValidatorDenomResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.superfluid.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "osmosis.superfluid" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.superfluid.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
