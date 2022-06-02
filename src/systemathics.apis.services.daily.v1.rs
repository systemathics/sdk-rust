/// The required input to request the DailyBarsService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyBarsRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment (dividends).
    /// By default the value is set to false : the split is applied in all cases
    #[prost(bool, tag="3")]
    pub adjustment: bool,
}
/// The daily bars response contains an array of daily bars.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyBarsResponse {
    /// The daily bars: an array of DailyBar objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<DailyBar>,
}
/// Contains the daily bar's data: date, open, high, low, close and volume.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyBar {
    /// Date of the bar
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Open price of the day
    #[prost(double, tag="2")]
    pub open: f64,
    /// Highest price of the day
    #[prost(double, tag="3")]
    pub high: f64,
    /// Lowest price of the day
    #[prost(double, tag="4")]
    pub low: f64,
    /// Close price of the day
    #[prost(double, tag="5")]
    pub close: f64,
    /// Total traded volume of the day
    #[prost(int64, tag="6")]
    pub volume: i64,
    /// The data quality scoring : from 0 (bad) to 100 (good)
    #[prost(double, tag="7")]
    pub score: f64,
}
/// Generated client implementations.
pub mod daily_bars_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request daily bars data.
    #[derive(Debug, Clone)]
    pub struct DailyBarsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DailyBarsServiceClient<tonic::transport::Channel> {
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
    impl<T> DailyBarsServiceClient<T>
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
        ) -> DailyBarsServiceClient<InterceptedService<T, F>>
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
            DailyBarsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets daily historical data: date, open, high, low, close and volume
        pub async fn daily_bars(
            &mut self,
            request: impl tonic::IntoRequest<super::DailyBarsRequest>,
        ) -> Result<tonic::Response<super::DailyBarsResponse>, tonic::Status> {
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
                "/systemathics.apis.services.daily.v1.DailyBarsService/DailyBars",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the DailyVwapsService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyVwapsRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange 
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment (dividends).
    /// By default the value is set to false : the split is applied in all cases
    #[prost(bool, tag="3")]
    pub adjustment: bool,
}
/// Represents a daily VWAPs response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyVwapsResponse {
    /// The daily vwaps: an array of DailyVwap objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<DailyVwap>,
}
/// Contains the daily vwaps data: date, vwap and volume.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyVwap {
    /// Date of the vwap
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Vwap of the day
    #[prost(double, tag="2")]
    pub price: f64,
    /// Total traded volume of the day
    #[prost(int64, tag="3")]
    pub volume: i64,
    /// The data quality scoring : from 0 (bad) to 100 (good)
    #[prost(double, tag="4")]
    pub score: f64,
}
/// Generated client implementations.
pub mod daily_vwaps_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request daily VWAP data.
    #[derive(Debug, Clone)]
    pub struct DailyVwapsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DailyVwapsServiceClient<tonic::transport::Channel> {
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
    impl<T> DailyVwapsServiceClient<T>
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
        ) -> DailyVwapsServiceClient<InterceptedService<T, F>>
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
            DailyVwapsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets daily historical VWAPs
        pub async fn daily_vwaps(
            &mut self,
            request: impl tonic::IntoRequest<super::DailyVwapsRequest>,
        ) -> Result<tonic::Response<super::DailyVwapsResponse>, tonic::Status> {
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
                "/systemathics.apis.services.daily.v1.DailyVwapsService/DailyVwaps",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the DailyPricesService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyPricesRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange 
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The corporate action adjustment (dividends).
    /// By default the value is set to false : the split is applied in all cases
    #[prost(bool, tag="3")]
    pub adjustment: bool,
}
/// Represents a daily prices response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyPricesResponse {
    /// The daily prices: an array of DailyPrice objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<DailyPrice>,
}
/// Contains the daily prices data: date, price and volume.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyPrice {
    /// Date of the price
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Last price of the day
    #[prost(double, tag="2")]
    pub price: f64,
    /// Total traded volume of the day
    #[prost(int64, tag="3")]
    pub volume: i64,
    /// The data quality scoring : from 0 (bad) to 100 (good)
    #[prost(double, tag="4")]
    pub score: f64,
}
/// Generated client implementations.
pub mod daily_prices_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request daily prices data.
    #[derive(Debug, Clone)]
    pub struct DailyPricesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DailyPricesServiceClient<tonic::transport::Channel> {
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
    impl<T> DailyPricesServiceClient<T>
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
        ) -> DailyPricesServiceClient<InterceptedService<T, F>>
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
            DailyPricesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets daily historical prices
        pub async fn daily_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::DailyPricesRequest>,
        ) -> Result<tonic::Response<super::DailyPricesResponse>, tonic::Status> {
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
                "/systemathics.apis.services.daily.v1.DailyPricesService/DailyPrices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
