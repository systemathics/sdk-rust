/// The required input to request the IntradayEmaService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayEmaRequest {
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
    /// \[Mandatory\] The exponential moving average window length (period : points number)
    #[prost(int32, tag="4")]
    pub length: i32,
    /// \[Optional\] The corporate action adjustment (dividends and splits).
    /// By default the value is set to false
    #[prost(bool, tag="5")]
    pub adjustment: bool,
}
/// Represents a intraday exponential moving average response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayEmaResponse {
    /// The intraday exponential moving averages: an array of IntradayEmaData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<IntradayEmaData>,
}
/// Contains the intraday exponential moving average data: date and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayEmaData {
    /// Time stamp of the exponential moving average : open time of the sampling interval
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The data used to calculate the average of the day
    #[prost(double, tag="2")]
    pub value: f64,
    /// The exponential moving average value of the day
    #[prost(double, tag="3")]
    pub average: f64,
}
/// Generated client implementations.
pub mod intraday_ema_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request intraday exponential moving average data.
    #[derive(Debug, Clone)]
    pub struct IntradayEmaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntradayEmaServiceClient<tonic::transport::Channel> {
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
    impl<T> IntradayEmaServiceClient<T>
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
        ) -> IntradayEmaServiceClient<InterceptedService<T, F>>
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
            IntradayEmaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets intraday exponential moving average data
        pub async fn intraday_ema(
            &mut self,
            request: impl tonic::IntoRequest<super::IntradayEmaRequest>,
        ) -> Result<tonic::Response<super::IntradayEmaResponse>, tonic::Status> {
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
                "/systemathics.apis.services.intraday_analytics.v1.IntradayEmaService/IntradayEma",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the IntradaySmaService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradaySmaRequest {
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
    /// \[Mandatory\] The simple moving average window length (period : points number)
    #[prost(int32, tag="4")]
    pub length: i32,
    /// \[Optional\] The corporate action adjustment (dividends and splits).
    /// By default the value is set to false
    #[prost(bool, tag="5")]
    pub adjustment: bool,
}
/// Represents a intraday simple moving average response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradaySmaResponse {
    /// The intraday simple moving averages: an array of IntradaySmaData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<IntradaySmaData>,
}
/// Contains the intraday simple moving average data: date and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradaySmaData {
    /// Time stamp of the simple moving average : open time of the sampling interval
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The data used to calculate the average of the day
    #[prost(double, tag="2")]
    pub value: f64,
    /// The simple moving average value of the day for the last n data points (length)
    #[prost(message, optional, tag="3")]
    pub average: ::core::option::Option<f64>,
}
/// Generated client implementations.
pub mod intraday_sma_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request intraday simple moving average data.
    #[derive(Debug, Clone)]
    pub struct IntradaySmaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntradaySmaServiceClient<tonic::transport::Channel> {
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
    impl<T> IntradaySmaServiceClient<T>
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
        ) -> IntradaySmaServiceClient<InterceptedService<T, F>>
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
            IntradaySmaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets intraday simple moving average data
        pub async fn intraday_sma(
            &mut self,
            request: impl tonic::IntoRequest<super::IntradaySmaRequest>,
        ) -> Result<tonic::Response<super::IntradaySmaResponse>, tonic::Status> {
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
                "/systemathics.apis.services.intraday_analytics.v1.IntradaySmaService/IntradaySma",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the IntradayRsiService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayRsiRequest {
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
    /// \[Mandatory\] The relative strength index window length (period: points number)
    #[prost(int32, tag="4")]
    pub length: i32,
    /// \[Optional\] The corporate action adjustment (dividends and splits).
    /// By default the value is set to false
    #[prost(bool, tag="5")]
    pub adjustment: bool,
}
/// Represents a intraday relative strength index response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayRsiResponse {
    /// The intraday relative strength index: an array of IntradayRsiData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<IntradayRsiData>,
}
/// Contains the intraday relative strength index data: date and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayRsiData {
    /// Time stamp of the relative strength index : open time of the sampling interval
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The data used to calculate the relative strength index of the day
    #[prost(double, tag="2")]
    pub value: f64,
    /// Relative strength index value of the day
    #[prost(double, tag="3")]
    pub rsi: f64,
}
/// Generated client implementations.
pub mod intraday_rsi_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request intraday relative strength index data.
    #[derive(Debug, Clone)]
    pub struct IntradayRsiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntradayRsiServiceClient<tonic::transport::Channel> {
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
    impl<T> IntradayRsiServiceClient<T>
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
        ) -> IntradayRsiServiceClient<InterceptedService<T, F>>
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
            IntradayRsiServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets intraday relative strength index data
        pub async fn intraday_rsi(
            &mut self,
            request: impl tonic::IntoRequest<super::IntradayRsiRequest>,
        ) -> Result<tonic::Response<super::IntradayRsiResponse>, tonic::Status> {
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
                "/systemathics.apis.services.intraday_analytics.v1.IntradayRsiService/IntradayRsi",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the IntradayCmaService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayCmaRequest {
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
/// Represents a intraday cumulative moving average response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayCmaResponse {
    /// The intraday cumulative moving averages: an array of IntradayCmaData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<IntradayCmaData>,
}
/// Contains the intraday cumulative moving average data: date and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayCmaData {
    /// Time stamp of the cumulative moving average : open time of the sampling interval
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The data used to calculate the average of the day
    #[prost(double, tag="2")]
    pub value: f64,
    /// The cumulative moving average value of the day for all the data points
    #[prost(double, tag="3")]
    pub average: f64,
}
/// Generated client implementations.
pub mod intraday_cma_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request intraday cumulative moving average data.
    #[derive(Debug, Clone)]
    pub struct IntradayCmaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntradayCmaServiceClient<tonic::transport::Channel> {
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
    impl<T> IntradayCmaServiceClient<T>
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
        ) -> IntradayCmaServiceClient<InterceptedService<T, F>>
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
            IntradayCmaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets intraday cumulative moving average data
        pub async fn intraday_cma(
            &mut self,
            request: impl tonic::IntoRequest<super::IntradayCmaRequest>,
        ) -> Result<tonic::Response<super::IntradayCmaResponse>, tonic::Status> {
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
                "/systemathics.apis.services.intraday_analytics.v1.IntradayCmaService/IntradayCma",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the IntradayMacdService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayMacdRequest {
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
    /// \[Mandatory\] The long EMA window length (period : points number)
    #[prost(int32, tag="4")]
    pub long: i32,
    /// \[Mandatory\] The short EMA window length (period : points number)
    #[prost(int32, tag="5")]
    pub short: i32,
    /// \[Optional\] The corporate action adjustment (dividends and splits).
    /// By default the value is set to false
    #[prost(bool, tag="6")]
    pub adjustment: bool,
}
/// Represents a intraday moving average convergence divergence response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayMacdResponse {
    /// The intraday moving average convergence/divergence: an array of IntradayMacdData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<IntradayMacdData>,
}
/// Contains the intraday moving average convergence/divergence data: date and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayMacdData {
    /// Time stamp of the moving average convergence/divergence : open time of the sampling interval
    #[prost(message, optional, tag="1")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The data used to calculate the average convergence/divergence of the day
    #[prost(double, tag="2")]
    pub value: f64,
    /// The moving average convergence/divergence value of the day
    #[prost(double, tag="3")]
    pub macd: f64,
    /// The short exponential moving average value of the day
    #[prost(double, tag="4")]
    pub short: f64,
    /// The long exponential moving average value of the day
    #[prost(double, tag="5")]
    pub long: f64,
}
/// Generated client implementations.
pub mod intraday_macd_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request intraday moving average convergence divergence data.
    #[derive(Debug, Clone)]
    pub struct IntradayMacdServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntradayMacdServiceClient<tonic::transport::Channel> {
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
    impl<T> IntradayMacdServiceClient<T>
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
        ) -> IntradayMacdServiceClient<InterceptedService<T, F>>
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
            IntradayMacdServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets intraday moving average convergence divergence data
        pub async fn intraday_macd(
            &mut self,
            request: impl tonic::IntoRequest<super::IntradayMacdRequest>,
        ) -> Result<tonic::Response<super::IntradayMacdResponse>, tonic::Status> {
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
                "/systemathics.apis.services.intraday_analytics.v1.IntradayMacdService/IntradayMacd",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the IntradayBollingerService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayBollingerRequest {
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
    /// \[Mandatory\] The simple moving average window length (period: data points number)
    #[prost(int32, tag="4")]
    pub length: i32,
    /// \[Mandatory\] The standard deviation window length
    #[prost(double, tag="5")]
    pub deviation: f64,
    /// \[Optional\] The corporate action adjustment (dividends and splits).
    /// By default the value is set to false
    #[prost(bool, tag="6")]
    pub adjustment: bool,
}
/// The intraday bars response contains an array of Bollinger bands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayBollingerResponse {
    /// The intraday Bollinger bands: an array of IntradayBollingerData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<IntradayBollingerData>,
}
/// Contains the intraday Bollinger bands data: date, lower, middle and upper.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayBollingerData {
    /// Time stamp of Bollinger bands : open time of the sampling interval
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
/// Generated client implementations.
pub mod intraday_bollinger_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request intraday Bollinger bands data.
    #[derive(Debug, Clone)]
    pub struct IntradayBollingerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntradayBollingerServiceClient<tonic::transport::Channel> {
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
    impl<T> IntradayBollingerServiceClient<T>
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
        ) -> IntradayBollingerServiceClient<InterceptedService<T, F>>
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
            IntradayBollingerServiceClient::new(
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
        /// Gets intraday Bollinger bands data
        pub async fn intraday_bollinger(
            &mut self,
            request: impl tonic::IntoRequest<super::IntradayBollingerRequest>,
        ) -> Result<tonic::Response<super::IntradayBollingerResponse>, tonic::Status> {
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
                "/systemathics.apis.services.intraday_analytics.v1.IntradayBollingerService/IntradayBollinger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the IntradayVolatilityService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayVolatilityRequest {
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
/// Represents a intraday volatility response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntradayVolatilityResponse {
    /// Volatility value
    #[prost(double, tag="1")]
    pub value: f64,
}
/// Generated client implementations.
pub mod intraday_volatility_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request intraday volatility data.
    #[derive(Debug, Clone)]
    pub struct IntradayVolatilityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntradayVolatilityServiceClient<tonic::transport::Channel> {
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
    impl<T> IntradayVolatilityServiceClient<T>
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
        ) -> IntradayVolatilityServiceClient<InterceptedService<T, F>>
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
            IntradayVolatilityServiceClient::new(
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
        /// Gets intraday historical volatility
        pub async fn intraday_volatility(
            &mut self,
            request: impl tonic::IntoRequest<super::IntradayVolatilityRequest>,
        ) -> Result<tonic::Response<super::IntradayVolatilityResponse>, tonic::Status> {
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
                "/systemathics.apis.services.intraday_analytics.v1.IntradayVolatilityService/IntradayVolatility",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
