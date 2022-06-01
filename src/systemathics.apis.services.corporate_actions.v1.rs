/// The required input to request the SplitsService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitsRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange 
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
}
/// Represents a split response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitsResponse {
    /// The splits: an array of Split objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Split>,
}
/// Contains the splits data: date, old and new shares.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Split {
    /// Effective date
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// New shares
    #[prost(double, tag="2")]
    pub new_shares: f64,
    /// Old shares
    #[prost(double, tag="3")]
    pub old_shares: f64,
    /// The data quality scoring : from 0 (bad) to 100 (good)
    #[prost(double, tag="4")]
    pub score: f64,
}
/// Generated client implementations.
pub mod splits_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request splits data.
    #[derive(Debug, Clone)]
    pub struct SplitsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SplitsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SplitsServiceClient<T>
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
        ) -> SplitsServiceClient<InterceptedService<T, F>>
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
            SplitsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets splits historical data
        pub async fn splits(
            &mut self,
            request: impl tonic::IntoRequest<super::SplitsRequest>,
        ) -> Result<tonic::Response<super::SplitsResponse>, tonic::Status> {
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
                "/systemathics.apis.services.corporate_actions.v1.SplitsService/Splits",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the ChangesService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangesRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange 
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
}
/// Represents a change response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangesResponse {
    /// The changes: an array of symbol change objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Change>,
}
/// Contains the symbol changes data: date, old and new symbols.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Change {
    /// Effective date
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// New symbol
    #[prost(string, tag="2")]
    pub new_symbol: ::prost::alloc::string::String,
    /// Old symbol
    #[prost(string, tag="3")]
    pub old_symbol: ::prost::alloc::string::String,
    /// The data quality scoring : from 0 (bad) to 100 (good)
    #[prost(double, tag="4")]
    pub score: f64,
}
/// Generated client implementations.
pub mod changes_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request symbol changes data.
    #[derive(Debug, Clone)]
    pub struct ChangesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChangesServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ChangesServiceClient<T>
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
        ) -> ChangesServiceClient<InterceptedService<T, F>>
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
            ChangesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets symbol changes historical data
        pub async fn changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangesRequest>,
        ) -> Result<tonic::Response<super::ChangesResponse>, tonic::Status> {
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
                "/systemathics.apis.services.corporate_actions.v1.ChangesService/Changes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the DividendsService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DividendsRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange 
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
}
/// Represents a dividend response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DividendsResponse {
    /// The dividends: an array of Dividend objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Dividend>,
}
/// Contains the dividends data: date, type and amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dividend {
    /// Effective payment date
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Dividend type
    #[prost(enumeration="DividendType", tag="2")]
    pub r#type: i32,
    /// Dividend amount
    #[prost(double, tag="3")]
    pub amount: f64,
    /// The data quality scoring : from 0 (bad) to 100 (good)
    #[prost(double, tag="4")]
    pub score: f64,
}
/// Contains the dividend types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DividendType {
    /// Unspecified
    Unspecified = 0,
    /// Cash dividend
    Cash = 1,
    /// Stock dividend
    Stock = 2,
}
/// Generated client implementations.
pub mod dividends_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request dividends data.
    #[derive(Debug, Clone)]
    pub struct DividendsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DividendsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DividendsServiceClient<T>
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
        ) -> DividendsServiceClient<InterceptedService<T, F>>
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
            DividendsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets dividends historical data
        pub async fn dividends(
            &mut self,
            request: impl tonic::IntoRequest<super::DividendsRequest>,
        ) -> Result<tonic::Response<super::DividendsResponse>, tonic::Status> {
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
                "/systemathics.apis.services.corporate_actions.v1.DividendsService/Dividends",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
