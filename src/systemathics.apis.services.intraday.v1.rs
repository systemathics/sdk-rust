/// The required input to request the IntradayVwapsService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayVwapsRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange 
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Mandatory\] The sampling interval
    #[prost(enumeration="super::super::super::r#type::shared::v1::Sampling", tag="2")]
    pub sampling: i32,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="3")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment (dividends and splits).
    /// By default the value is set to false
    #[prost(bool, tag="4")]
    pub adjustment: bool,
}
/// Represents a intraday VWAPs response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayVwapsResponse {
    /// The intraday vwaps: an array of IntradayVwap objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<IntradayVwap>,
}
/// Contains the intraday vwaps data: date, vwap and volume.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayVwap {
    /// Time stamp of the intraday vwap : open time of the sampling interval
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Vwap of the sampling period
    #[prost(double, tag="2")]
    pub price: f64,
    /// Total traded volume of the sampling period
    #[prost(double, tag="3")]
    pub volume: f64,
    /// The data quality scoring : from 0 (bad) to 100 (good)
    #[prost(double, tag="4")]
    pub score: f64,
}
/// Generated client implementations.
pub mod intraday_vwaps_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request intraday VWAP data.
    #[derive(Debug, Clone)]
    pub struct IntradayVwapsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntradayVwapsServiceClient<tonic::transport::Channel> {
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
    impl<T> IntradayVwapsServiceClient<T>
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
        ) -> IntradayVwapsServiceClient<InterceptedService<T, F>>
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
            IntradayVwapsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets intraday historical VWAPs
        pub async fn intraday_vwaps(
            &mut self,
            request: impl tonic::IntoRequest<super::IntradayVwapsRequest>,
        ) -> Result<tonic::Response<super::IntradayVwapsResponse>, tonic::Status> {
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
                "/systemathics.apis.services.intraday.v1.IntradayVwapsService/IntradayVwaps",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the IntradayPricesService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayPricesRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange 
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Mandatory\] The sampling interval
    #[prost(enumeration="super::super::super::r#type::shared::v1::Sampling", tag="2")]
    pub sampling: i32,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="3")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment (dividends and splits).
    /// By default the value is set to false
    #[prost(bool, tag="4")]
    pub adjustment: bool,
}
/// Represents a intraday prices response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayPricesResponse {
    /// The intraday prices: an array of IntradayPrice objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<IntradayPrice>,
}
/// Contains the intraday prices data: date, price, volume and score.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayPrice {
    /// Time stamp of the intraday price : open time of the sampling interval
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Last price of the sampling period
    #[prost(double, tag="2")]
    pub price: f64,
    /// Total traded volume of the sampling period
    #[prost(double, tag="3")]
    pub volume: f64,
    /// The data quality scoring : from 0 (bad) to 100 (good)
    #[prost(double, tag="4")]
    pub score: f64,
}
/// Generated client implementations.
pub mod intraday_prices_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request intraday prices data.
    #[derive(Debug, Clone)]
    pub struct IntradayPricesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntradayPricesServiceClient<tonic::transport::Channel> {
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
    impl<T> IntradayPricesServiceClient<T>
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
        ) -> IntradayPricesServiceClient<InterceptedService<T, F>>
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
            IntradayPricesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets intraday historical prices
        pub async fn intraday_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::IntradayPricesRequest>,
        ) -> Result<tonic::Response<super::IntradayPricesResponse>, tonic::Status> {
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
                "/systemathics.apis.services.intraday.v1.IntradayPricesService/IntradayPrices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the IntradayBarsService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayBarsRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Mandatory\] The sampling interval
    #[prost(enumeration="super::super::super::r#type::shared::v1::Sampling", tag="2")]
    pub sampling: i32,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="3")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment (dividends and splits).
    /// By default the value is set to false
    #[prost(bool, tag="4")]
    pub adjustment: bool,
}
/// The intraday bars response contains an array of intraday bars.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayBarsResponse {
    /// The intraday bars: an array of IntradayBar objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<IntradayBar>,
}
/// Contains the intraday bar's data: date, open, high, low, close, volume, count, vwap and score.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayBar {
    /// Time stamp of the intraday bar : open time of the sampling interval
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Open price of the sampling period
    #[prost(double, tag="2")]
    pub open: f64,
    /// Highest price of the sampling period
    #[prost(double, tag="3")]
    pub high: f64,
    /// Lowest price of the sampling period
    #[prost(double, tag="4")]
    pub low: f64,
    /// Close price of the sampling period
    #[prost(double, tag="5")]
    pub close: f64,
    /// Total traded volume of the sampling period
    #[prost(double, tag="6")]
    pub volume: f64,
    /// Tick count of the sampling period
    #[prost(int32, tag="7")]
    pub count: i32,
    /// Volume weighted average price of the sampling period
    #[prost(double, tag="8")]
    pub vwap: f64,
    /// The data quality scoring : from 0 (bad) to 100 (good)
    #[prost(double, tag="9")]
    pub score: f64,
}
/// Generated client implementations.
pub mod intraday_bars_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request intraday bars data.
    #[derive(Debug, Clone)]
    pub struct IntradayBarsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntradayBarsServiceClient<tonic::transport::Channel> {
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
    impl<T> IntradayBarsServiceClient<T>
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
        ) -> IntradayBarsServiceClient<InterceptedService<T, F>>
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
            IntradayBarsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets intraday historical data: date, open, high, low, close and volume
        pub async fn intraday_bars(
            &mut self,
            request: impl tonic::IntoRequest<super::IntradayBarsRequest>,
        ) -> Result<tonic::Response<super::IntradayBarsResponse>, tonic::Status> {
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
                "/systemathics.apis.services.intraday.v1.IntradayBarsService/IntradayBars",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
