/// The required inputs to request the TickTradesService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickTradesRequest {
    /// \[Mandatory\] The instrument identifiers: a list of tickers and exchanges
    #[prost(message, repeated, tag="1")]
    pub identifiers: ::prost::alloc::vec::Vec<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="3")]
    pub adjustment: bool,
}
/// Contains the tick by tick normalized trades data: key, time stamp, price, size, condition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickTradesResponse {
    #[prost(oneof="tick_trades_response::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<tick_trades_response::Payload>,
}
/// Nested message and enum types in `TickTradesResponse`.
pub mod tick_trades_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The trade data
        #[prost(message, tag="1")]
        Data(super::super::super::super::r#type::shared::v1::TradeData),
        /// The mapping data
        #[prost(message, tag="2")]
        Mapping(super::super::super::super::r#type::shared::v1::Keys),
    }
}
/// Generated client implementations.
pub mod tick_trades_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick normalized trades historical data.
    #[derive(Debug, Clone)]
    pub struct TickTradesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickTradesServiceClient<tonic::transport::Channel> {
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
    impl<T> TickTradesServiceClient<T>
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
        ) -> TickTradesServiceClient<InterceptedService<T, F>>
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
            TickTradesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Get tick by tick normalized trades historical Trades
        pub async fn tick_trades(
            &mut self,
            request: impl tonic::IntoRequest<super::TickTradesRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickTradesResponse>>,
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
                "/systemathics.apis.services.tick.v1.TickTradesService/TickTrades",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// The required inputs to request the TickTradesAndBookService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickTradesAndBookRequest {
    /// \[Mandatory\] The instrument and streams identifiers: a list of tickers and exchanges and streams
    #[prost(message, repeated, tag="1")]
    pub identifiers: ::prost::alloc::vec::Vec<super::super::super::r#type::shared::v1::IdentifierAndLevel>,
    /// \[Mandatory\] The book update scheme (snapshots only, or, initial snapshot then incremental updates)
    #[prost(enumeration="super::super::super::r#type::shared::v1::BookUpdates", tag="2")]
    pub book_updates: i32,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="3")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="4")]
    pub adjustment: bool,
    /// \[Optional\] The maximum number of depth, if not set use the default (10).
    #[prost(message, optional, tag="5")]
    pub max_depth: ::core::option::Option<i32>,
    /// \[Optional\] The contributors: get the quotes foreach depth, by default the value is set to false
    #[prost(bool, tag="6")]
    pub contributors: bool,
}
/// Contains the tick by tick normalized trades data: key, time stamp, price, size, condition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickTradesAndBookResponse {
    #[prost(oneof="tick_trades_and_book_response::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<tick_trades_and_book_response::Payload>,
}
/// Nested message and enum types in `TickTradesAndBookResponse`.
pub mod tick_trades_and_book_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The trade and book data
        #[prost(message, tag="1")]
        Data(super::super::super::super::r#type::shared::v1::TradeAndBookData),
        /// The mapping data
        #[prost(message, tag="2")]
        Mapping(super::super::super::super::r#type::shared::v1::Keys),
    }
}
/// Generated client implementations.
pub mod tick_trades_and_book_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick normalized trades and book historical data.
    #[derive(Debug, Clone)]
    pub struct TickTradesAndBookServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickTradesAndBookServiceClient<tonic::transport::Channel> {
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
    impl<T> TickTradesAndBookServiceClient<T>
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
        ) -> TickTradesAndBookServiceClient<InterceptedService<T, F>>
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
            TickTradesAndBookServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Get tick by tick normalized trades and book historical
        pub async fn tick_trades_and_book(
            &mut self,
            request: impl tonic::IntoRequest<super::TickTradesAndBookRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickTradesAndBookResponse>>,
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
                "/systemathics.apis.services.tick.v1.TickTradesAndBookService/TickTradesAndBook",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// The required inputs to request the TickRawService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickRawRequest {
    /// \[Mandatory\] The instruments and streams identifiers: a list of tickers, exchanges and streams.
    #[prost(message, repeated, tag="1")]
    pub identifiers: ::prost::alloc::vec::Vec<super::super::super::r#type::shared::v1::IdentifierAndLevel>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
}
/// Represents the raw data response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickRawResponse {
    #[prost(oneof="tick_raw_response::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<tick_raw_response::Payload>,
}
/// Nested message and enum types in `TickRawResponse`.
pub mod tick_raw_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The raw data
        #[prost(message, tag="1")]
        Raw(super::super::super::super::r#type::shared::v1::Raw),
        /// The mapping data
        #[prost(message, tag="2")]
        Mapping(super::super::super::super::r#type::shared::v1::Keys),
    }
}
/// Generated client implementations.
pub mod tick_raw_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick raw data.
    #[derive(Debug, Clone)]
    pub struct TickRawServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickRawServiceClient<tonic::transport::Channel> {
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
    impl<T> TickRawServiceClient<T>
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
        ) -> TickRawServiceClient<InterceptedService<T, F>>
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
            TickRawServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets tick by tick raw data.
        pub async fn tick_raw(
            &mut self,
            request: impl tonic::IntoRequest<super::TickRawRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickRawResponse>>,
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
                "/systemathics.apis.services.tick.v1.TickRawService/TickRaw",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// The required inputs to request the TickUpdatesService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickUpdatesRequest {
    /// \[Mandatory\] The instrument and streams identifiers: a list of tickers and exchanges and streams
    #[prost(message, repeated, tag="1")]
    pub identifiers: ::prost::alloc::vec::Vec<super::super::super::r#type::shared::v1::IdentifierAndLevel>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="3")]
    pub adjustment: bool,
}
/// Contains the tick by tick normalized historical data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickUpdatesResponse {
    /// Payload: that can be a mapping (event source and instrument identifier), the fields update or the books update
    #[prost(oneof="tick_updates_response::Payload", tags="1, 2, 3, 4")]
    pub payload: ::core::option::Option<tick_updates_response::Payload>,
}
/// Nested message and enum types in `TickUpdatesResponse`.
pub mod tick_updates_response {
    /// Payload: that can be a mapping (event source and instrument identifier), the fields update or the books update
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The fields update
        #[prost(message, tag="1")]
        FieldsUpdates(super::super::super::super::r#type::shared::v1::MarketFieldsUpdates),
        /// The MBL books update
        #[prost(message, tag="2")]
        MblBookUpdates(super::super::super::super::r#type::shared::v1::MblMarketBookUpdates),
        /// The MBO books update
        #[prost(message, tag="3")]
        MboBookUpdates(super::super::super::super::r#type::shared::v1::MboMarketBookUpdates),
        /// The mapping data
        #[prost(message, tag="4")]
        Mapping(super::super::super::super::r#type::shared::v1::Keys),
    }
}
/// Generated client implementations.
pub mod tick_updates_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick normalized historical data.
    #[derive(Debug, Clone)]
    pub struct TickUpdatesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickUpdatesServiceClient<tonic::transport::Channel> {
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
    impl<T> TickUpdatesServiceClient<T>
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
        ) -> TickUpdatesServiceClient<InterceptedService<T, F>>
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
            TickUpdatesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Get tick by tick normalized historical Updates
        pub async fn tick_updates(
            &mut self,
            request: impl tonic::IntoRequest<super::TickUpdatesRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickUpdatesResponse>>,
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
                "/systemathics.apis.services.tick.v1.TickUpdatesService/TickUpdates",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// The required inputs to request the TickBookService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickBookRequest {
    /// \[Mandatory\] The instrument identifiers: a list of tickers and exchanges 
    #[prost(message, repeated, tag="1")]
    pub identifiers: ::prost::alloc::vec::Vec<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Mandatory\] The book update scheme (snapshots only, or, initial snapshot then incremental updates)
    #[prost(enumeration="super::super::super::r#type::shared::v1::BookUpdates", tag="2")]
    pub book_updates: i32,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="3")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="4")]
    pub adjustment: bool,
    /// \[Optional\] The maximum number of depth, if not set use the default (10).
    #[prost(message, optional, tag="5")]
    pub max_depth: ::core::option::Option<i32>,
    /// \[Optional\] The contributors: get the quotes foreach depth, by default the value is set to false
    #[prost(bool, tag="6")]
    pub contributors: bool,
}
/// Contains the tick by tick normalized book : data or mapping.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickBookResponse {
    #[prost(oneof="tick_book_response::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<tick_book_response::Payload>,
}
/// Nested message and enum types in `TickBookResponse`.
pub mod tick_book_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The limit data
        #[prost(message, tag="1")]
        Data(super::super::super::super::r#type::shared::v1::BookData),
        /// The mapping data
        #[prost(message, tag="2")]
        Mapping(super::super::super::super::r#type::shared::v1::Keys),
    }
}
/// Generated client implementations.
pub mod tick_book_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick normalized book data.
    #[derive(Debug, Clone)]
    pub struct TickBookServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickBookServiceClient<tonic::transport::Channel> {
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
    impl<T> TickBookServiceClient<T>
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
        ) -> TickBookServiceClient<InterceptedService<T, F>>
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
            TickBookServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Get tick by tick normalized historical book
        pub async fn tick_book(
            &mut self,
            request: impl tonic::IntoRequest<super::TickBookRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickBookResponse>>,
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
                "/systemathics.apis.services.tick.v1.TickBookService/TickBook",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// The required inputs to request the TickQuotesService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickQuotesRequest {
    /// \[Mandatory\] The instrument identifiers: a list of tickers and exchanges
    #[prost(message, repeated, tag="1")]
    pub identifiers: ::prost::alloc::vec::Vec<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="3")]
    pub adjustment: bool,
}
/// Contains the tick by tick normalized quotes: key and data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickQuotesResponse {
    #[prost(oneof="tick_quotes_response::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<tick_quotes_response::Payload>,
}
/// Nested message and enum types in `TickQuotesResponse`.
pub mod tick_quotes_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The quotes data
        #[prost(message, tag="1")]
        Data(super::super::super::super::r#type::shared::v1::QuotesData),
        /// The mapping data
        #[prost(message, tag="2")]
        Mapping(super::super::super::super::r#type::shared::v1::Keys),
    }
}
/// Generated client implementations.
pub mod tick_quotes_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick normalized quotes data (MBO).
    #[derive(Debug, Clone)]
    pub struct TickQuotesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickQuotesServiceClient<tonic::transport::Channel> {
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
    impl<T> TickQuotesServiceClient<T>
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
        ) -> TickQuotesServiceClient<InterceptedService<T, F>>
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
            TickQuotesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Get tick by tick normalized quotes
        pub async fn tick_quotes(
            &mut self,
            request: impl tonic::IntoRequest<super::TickQuotesRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickQuotesResponse>>,
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
                "/systemathics.apis.services.tick.v1.TickQuotesService/TickQuotes",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
