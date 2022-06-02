/// The required inputs to request the TickSmaService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickSmaRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Mandatory\] The price used to calculate the sma
    #[prost(enumeration="SmaPrice", tag="3")]
    pub field: i32,
    /// \[Mandatory\] The simple moving average window length (period : points number)
    #[prost(int32, tag="4")]
    pub length: i32,
    /// \[Optional\] The period of the sma, by default the period is set to one day
    #[prost(message, optional, tag="5")]
    pub period: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The offset time when the sma is reset, by default the offset is set to 00:00:00
    #[prost(message, optional, tag="6")]
    pub offset: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The sampling of the tick data.
    /// If not set then the tick by tick data is used.
    /// If set, then the sma is built using sampled data.
    #[prost(message, optional, tag="7")]
    pub sampling: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="8")]
    pub adjustment: bool,
    /// \[Optional\] Accept trading / quote conditions, by default the accept is set to null : accept all
    #[prost(string, repeated, tag="9")]
    pub accept: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// \[Optional\] Rejected trading / quote conditions, by default the reject is set to null : do not reject
    #[prost(string, repeated, tag="10")]
    pub reject: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Contains the simple moving average data: timestamp and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickSmaResponse {
    /// The time stamp of the simple moving average
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The data used to calculate the average
    #[prost(double, tag="2")]
    pub value: f64,
    /// The simple moving average value for the last n data points (length)
    #[prost(message, optional, tag="3")]
    pub average: ::core::option::Option<f64>,
}
/// The price types used to compute the avarage
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SmaPrice {
    /// The unspecfied field
    Unspecified = 0,
    /// The last trade price
    Trade = 1,
    /// The best bid price
    Bid = 2,
    /// The best ask price
    Ask = 3,
}
/// Generated client implementations.
pub mod tick_sma_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick simple moving average data.
    #[derive(Debug, Clone)]
    pub struct TickSmaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickSmaServiceClient<tonic::transport::Channel> {
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
    impl<T> TickSmaServiceClient<T>
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
        ) -> TickSmaServiceClient<InterceptedService<T, F>>
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
            TickSmaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets tick by tick simple moving average data
        pub async fn tick_sma(
            &mut self,
            request: impl tonic::IntoRequest<super::TickSmaRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickSmaResponse>>,
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
                "/systemathics.apis.services.tick_analytics.v1.TickSmaService/TickSma",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// The required inputs to request the TickBarService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickBarsRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The price used to calculate the bar
    #[prost(enumeration="BarPrice", tag="3")]
    pub field: i32,
    /// \[Mandatory\] The duration of the bar
    #[prost(message, optional, tag="4")]
    pub sampling: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The period of the bars, by default the period is set to one day
    #[prost(message, optional, tag="5")]
    pub period: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The offset time when the bars is reset, by default the offset is set to 00:00:00
    #[prost(message, optional, tag="6")]
    pub offset: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="7")]
    pub adjustment: bool,
    /// \[Optional\] Accept trading / quote conditions, by default the accept is set to null : accept all
    #[prost(string, repeated, tag="8")]
    pub accept: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// \[Optional\] Rejected trading / quote conditions, by default the reject is set to null : do not reject
    #[prost(string, repeated, tag="9")]
    pub reject: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Contains the tick bar's data: time, open, high, low, close, volume, count and vwap.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickBarsResponse {
    /// The time stamp
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Open price of the current bar
    #[prost(double, tag="2")]
    pub open: f64,
    /// Highest price of the current bar
    #[prost(double, tag="3")]
    pub high: f64,
    /// Lowest price of the current bar
    #[prost(double, tag="4")]
    pub low: f64,
    /// Close price of the current bar
    #[prost(double, tag="5")]
    pub close: f64,
    /// Total traded volume of the current bar
    #[prost(int64, tag="6")]
    pub volume: i64,
    /// Tick count of the current bar
    #[prost(int32, tag="7")]
    pub count: i32,
    /// Volume weighted average price of the current bar
    #[prost(double, tag="8")]
    pub vwap: f64,
}
/// The price types used to compute the bars
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BarPrice {
    /// The unspecfied field
    Unspecified = 0,
    /// The last trade price
    Trade = 1,
    /// The best bid price
    Bid = 2,
    /// The best ask price
    Ask = 3,
}
/// Generated client implementations.
pub mod tick_bars_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick bars data.
    #[derive(Debug, Clone)]
    pub struct TickBarsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickBarsServiceClient<tonic::transport::Channel> {
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
    impl<T> TickBarsServiceClient<T>
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
        ) -> TickBarsServiceClient<InterceptedService<T, F>>
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
            TickBarsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets tick by tick bars data
        pub async fn tick_bars(
            &mut self,
            request: impl tonic::IntoRequest<super::TickBarsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickBarsResponse>>,
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
                "/systemathics.apis.services.tick_analytics.v1.TickBarsService/TickBars",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the TickVwapService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickVwapRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period
    /// If empty, then all the available data is used
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Optional\] The period of the vwap, by default the period is set to one day
    #[prost(message, optional, tag="3")]
    pub period: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The offset time when the vwap is reset, by default the offset is set to 00:00:00
    #[prost(message, optional, tag="4")]
    pub offset: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="5")]
    pub adjustment: bool,
    /// \[Optional\] Accept trading / quote conditions, by default the accept is set to null : accept all
    #[prost(string, repeated, tag="6")]
    pub accept: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// \[Optional\] Rejected trading / quote conditions, by default the reject is set to null : do not reject
    #[prost(string, repeated, tag="7")]
    pub reject: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Contains the vwap data: timestamp, price, volume and ticks count.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickVwapResponse {
    /// The time stamp of the vwap
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The vwap price
    #[prost(double, tag="2")]
    pub vwap: f64,
    /// The trade price
    #[prost(double, tag="3")]
    pub trade: f64,
    /// Total traded volume
    #[prost(int64, tag="4")]
    pub volume: i64,
    /// The ticks count
    #[prost(int32, tag="5")]
    pub ticks: i32,
}
/// Generated client implementations.
pub mod tick_vwap_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tcik Volume Weighted Average Price.
    #[derive(Debug, Clone)]
    pub struct TickVwapServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickVwapServiceClient<tonic::transport::Channel> {
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
    impl<T> TickVwapServiceClient<T>
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
        ) -> TickVwapServiceClient<InterceptedService<T, F>>
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
            TickVwapServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets tick by tick historical VWAPs
        pub async fn tick_vwap(
            &mut self,
            request: impl tonic::IntoRequest<super::TickVwapRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickVwapResponse>>,
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
                "/systemathics.apis.services.tick_analytics.v1.TickVwapService/TickVwap",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// The required inputs to request the TickCmaService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickCmaRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period
    /// If empty, then all the available data is used
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Mandatory\] The field used to calculate the cma
    #[prost(enumeration="CmaPrice", tag="3")]
    pub field: i32,
    /// \[Optional\] The period of the vwap, by default the period is set to one day
    #[prost(message, optional, tag="4")]
    pub period: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The offset time when the vwap is reset, by default the offset is set to 00:00:00
    #[prost(message, optional, tag="5")]
    pub offset: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The sampling of the tick data
    /// If not set then the tick by tick data is used
    /// If set, then the cma is built using sampled data
    #[prost(message, optional, tag="6")]
    pub sampling: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="7")]
    pub adjustment: bool,
    /// \[Optional\] Accept trading / quote conditions, by default the accept is set to null : accept all
    #[prost(string, repeated, tag="8")]
    pub accept: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// \[Optional\] Rejected trading / quote conditions, by default the reject is set to null : do not reject
    #[prost(string, repeated, tag="9")]
    pub reject: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Contains the cumulative moving average data: timestamp and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickCmaResponse {
    /// The time stamp of the cumulative moving average
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The data used to calculate the average
    #[prost(double, tag="2")]
    pub value: f64,
    /// The cumulative moving average value for all the data points
    #[prost(message, optional, tag="3")]
    pub average: ::core::option::Option<f64>,
}
/// The price types used to compute the avarage
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CmaPrice {
    /// The unspecfied field
    Unspecified = 0,
    /// The last trade price
    Trade = 1,
    /// The best bid price
    Bid = 2,
    /// The best ask price
    Ask = 3,
}
/// Generated client implementations.
pub mod tick_cma_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick cumulative moving average data.
    #[derive(Debug, Clone)]
    pub struct TickCmaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickCmaServiceClient<tonic::transport::Channel> {
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
    impl<T> TickCmaServiceClient<T>
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
        ) -> TickCmaServiceClient<InterceptedService<T, F>>
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
            TickCmaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets tick by tick cumulative moving average data
        pub async fn tick_cma(
            &mut self,
            request: impl tonic::IntoRequest<super::TickCmaRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickCmaResponse>>,
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
                "/systemathics.apis.services.tick_analytics.v1.TickCmaService/TickCma",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// The required inputs to request the TickEmaService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickEmaRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Mandatory\] The field used to calculate the ema
    #[prost(enumeration="EmaPrice", tag="3")]
    pub field: i32,
    /// \[Mandatory\] The exponential moving average window length (period : points number)
    #[prost(int32, tag="4")]
    pub length: i32,
    /// \[Optional\] The period of the ema, by default the period is set to one day
    #[prost(message, optional, tag="5")]
    pub period: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The offset time when the ema is reset, by default the offset is set to 00:00:00
    #[prost(message, optional, tag="6")]
    pub offset: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The sampling of the tick data.
    /// If not set then the tick by tick data is used.
    /// If set, then the sma is built using sampled data.
    #[prost(message, optional, tag="7")]
    pub sampling: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="8")]
    pub adjustment: bool,
    /// \[Optional\] Accept trading / quote conditions, by default the accept is set to null : accept all
    #[prost(string, repeated, tag="9")]
    pub accept: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// \[Optional\] Rejected trading / quote conditions, by default the reject is set to null : do not reject
    #[prost(string, repeated, tag="10")]
    pub reject: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Contains the exponential moving average data: timestamp and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickEmaResponse {
    /// The time stamp the exponential moving average
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The data used to calculate the average
    #[prost(double, tag="2")]
    pub value: f64,
    /// The exponential moving average value
    #[prost(message, optional, tag="3")]
    pub average: ::core::option::Option<f64>,
}
/// The price types used to compute the avarage
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EmaPrice {
    /// The unspecfied field
    Unspecified = 0,
    /// The last trade price
    Trade = 1,
    /// The best bid price
    Bid = 2,
    /// The best ask price
    Ask = 3,
}
/// Generated client implementations.
pub mod tick_ema_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick exponential moving average data.
    #[derive(Debug, Clone)]
    pub struct TickEmaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickEmaServiceClient<tonic::transport::Channel> {
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
    impl<T> TickEmaServiceClient<T>
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
        ) -> TickEmaServiceClient<InterceptedService<T, F>>
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
            TickEmaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets tick by tick exponential moving average data
        pub async fn tick_ema(
            &mut self,
            request: impl tonic::IntoRequest<super::TickEmaRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickEmaResponse>>,
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
                "/systemathics.apis.services.tick_analytics.v1.TickEmaService/TickEma",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// The required inputs to request the TickBollingerService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickBollingerRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Mandatory\] The field used to build the Bollinger bands
    #[prost(enumeration="BollingerPrice", tag="3")]
    pub field: i32,
    /// \[Mandatory\] The simple moving average window length (period: data points number)
    #[prost(int32, tag="4")]
    pub length: i32,
    /// \[Mandatory\] The standard deviation window length
    #[prost(double, tag="5")]
    pub deviation: f64,
    /// \[Optional\] The period of the Bollinger bands, by default the period is set to one day
    #[prost(message, optional, tag="6")]
    pub period: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The offset time when the Bollinger bands is reset, by default the offset is set to 00:00:00
    #[prost(message, optional, tag="7")]
    pub offset: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The sampling of the tick data.
    /// If not set then the tick by tick data is used.
    /// If set, then the sma is built using sampled data.
    #[prost(message, optional, tag="8")]
    pub sampling: ::core::option::Option<::prost_types::Duration>,
    /// \[Optional\] The corporate action adjustment, by default the value is set to false
    #[prost(bool, tag="9")]
    pub adjustment: bool,
    /// \[Optional\] Accept trading / quote conditions, by default the accept is set to null : accept all
    #[prost(string, repeated, tag="10")]
    pub accept: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// \[Optional\] Rejected trading / quote conditions, by default the reject is set to null : do not reject
    #[prost(string, repeated, tag="11")]
    pub reject: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Contains the tick Bollinger bands data: date, lower, middle and upper.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickBollingerResponse {
    /// The time stamp of the bollinger bands
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The data used to calculate the Bollinger bands of the day
    #[prost(double, tag="2")]
    pub value: f64,
    /// Bollinger lower band value of the day
    /// <br><i>Lower band = Middle band - (k * n-period standard deviation)</i>
    #[prost(message, optional, tag="3")]
    pub lower: ::core::option::Option<f64>,
    /// Bollinger upper band value of the day
    /// <br><i>Upper band = Middle band + (k * n-period standard deviation)</i>
    #[prost(message, optional, tag="4")]
    pub upper: ::core::option::Option<f64>,
    /// Bollinger middle band value of the day
    /// <br><i>Middle band = n-period moving average</i>
    #[prost(message, optional, tag="5")]
    pub middle: ::core::option::Option<f64>,
}
/// The price types used to compute the bollinger bands
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BollingerPrice {
    /// The unspecfied field
    Unspecified = 0,
    /// The last trade price
    Trade = 1,
    /// The best bid price
    Bid = 2,
    /// The best ask price
    Ask = 3,
}
/// Generated client implementations.
pub mod tick_bollinger_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick Bollinger bands data.
    #[derive(Debug, Clone)]
    pub struct TickBollingerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickBollingerServiceClient<tonic::transport::Channel> {
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
    impl<T> TickBollingerServiceClient<T>
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
        ) -> TickBollingerServiceClient<InterceptedService<T, F>>
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
            TickBollingerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets tick by tick Bollinger bands data
        pub async fn tick_bollinger(
            &mut self,
            request: impl tonic::IntoRequest<super::TickBollingerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TickBollingerResponse>>,
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
                "/systemathics.apis.services.tick_analytics.v1.TickBollingerService/TickBollinger",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
