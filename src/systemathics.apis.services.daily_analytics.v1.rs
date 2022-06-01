/// The required input to request the DailyRsiService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyRsiRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Mandatory\] The relative strength index window length (period: points number)
    #[prost(int32, tag="3")]
    pub length: i32,
    /// \[Optional\] The corporate action adjustment (dividends).
    /// By default the value is set to false : the split is applied in all cases
    #[prost(bool, tag="4")]
    pub adjustment: bool,
}
/// Represents a daily relative strength index response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyRsiResponse {
    /// The daily relative strength index: an array of DailyRsiData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<DailyRsiData>,
}
/// Contains the daily relative strength index data: date and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyRsiData {
    /// Date of the relative strength index
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// The data used to calculate the relative strength index of the day
    #[prost(double, tag="2")]
    pub value: f64,
    /// Relative strength index value of the day
    #[prost(double, tag="3")]
    pub rsi: f64,
}
/// Generated client implementations.
pub mod daily_rsi_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request daily relative strength index data.
    #[derive(Debug, Clone)]
    pub struct DailyRsiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DailyRsiServiceClient<tonic::transport::Channel> {
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
    impl<T> DailyRsiServiceClient<T>
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
        ) -> DailyRsiServiceClient<InterceptedService<T, F>>
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
            DailyRsiServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets daily relative strength index data
        pub async fn daily_rsi(
            &mut self,
            request: impl tonic::IntoRequest<super::DailyRsiRequest>,
        ) -> Result<tonic::Response<super::DailyRsiResponse>, tonic::Status> {
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
                "/systemathics.apis.services.daily_analytics.v1.DailyRsiService/DailyRsi",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the DailySmaService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailySmaRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Mandatory\] The simple moving average window length (period : points number)
    #[prost(int32, tag="3")]
    pub length: i32,
    /// \[Optional\] The corporate action adjustment (dividends).
    /// By default the value is set to false : the split is applied in all cases
    #[prost(bool, tag="4")]
    pub adjustment: bool,
}
/// Represents a daily simple moving average response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailySmaResponse {
    /// The daily simple moving averages: an array of DailySmaData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<DailySmaData>,
}
/// Contains the daily simple moving average data: date and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailySmaData {
    /// Date of the simple moving average
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// The data used to calculate the average of the day
    #[prost(double, tag="2")]
    pub value: f64,
    /// The simple moving average value of the day for the last n data points (length)
    #[prost(message, optional, tag="3")]
    pub average: ::core::option::Option<f64>,
}
/// Generated client implementations.
pub mod daily_sma_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request daily simple moving average data.
    #[derive(Debug, Clone)]
    pub struct DailySmaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DailySmaServiceClient<tonic::transport::Channel> {
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
    impl<T> DailySmaServiceClient<T>
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
        ) -> DailySmaServiceClient<InterceptedService<T, F>>
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
            DailySmaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets daily simple moving average data
        pub async fn daily_sma(
            &mut self,
            request: impl tonic::IntoRequest<super::DailySmaRequest>,
        ) -> Result<tonic::Response<super::DailySmaResponse>, tonic::Status> {
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
                "/systemathics.apis.services.daily_analytics.v1.DailySmaService/DailySma",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the DailyCmaService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyCmaRequest {
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
/// Represents a daily cumulative moving average response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyCmaResponse {
    /// The daily cumulative moving averages: an array of DailyCmaData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<DailyCmaData>,
}
/// Contains the daily cumulative moving average data: date and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyCmaData {
    /// Date of the cumulative moving average
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// The data used to calculate the average of the day
    #[prost(double, tag="2")]
    pub value: f64,
    /// The cumulative moving average value of the day for all the data points
    #[prost(double, tag="3")]
    pub average: f64,
}
/// Generated client implementations.
pub mod daily_cma_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request daily cumulative moving average data.
    #[derive(Debug, Clone)]
    pub struct DailyCmaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DailyCmaServiceClient<tonic::transport::Channel> {
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
    impl<T> DailyCmaServiceClient<T>
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
        ) -> DailyCmaServiceClient<InterceptedService<T, F>>
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
            DailyCmaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets daily cumulative moving average data
        pub async fn daily_cma(
            &mut self,
            request: impl tonic::IntoRequest<super::DailyCmaRequest>,
        ) -> Result<tonic::Response<super::DailyCmaResponse>, tonic::Status> {
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
                "/systemathics.apis.services.daily_analytics.v1.DailyCmaService/DailyCma",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the DailyMacdService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyMacdRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Mandatory\] The long EMA window length (period : points number)
    #[prost(int32, tag="3")]
    pub long: i32,
    /// \[Mandatory\] The short EMA window length (period : points number)
    #[prost(int32, tag="4")]
    pub short: i32,
    /// \[Optional\] The corporate action adjustment (dividends).
    /// By default the value is set to false : the split is applied in all cases
    #[prost(bool, tag="5")]
    pub adjustment: bool,
}
/// Represents a daily moving average convergence divergence response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyMacdResponse {
    /// The daily moving average convergence/divergence: an array of DailyMacdData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<DailyMacdData>,
}
/// Contains the daily moving average convergence/divergence data: date and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyMacdData {
    /// Date of the moving average convergence/divergence
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
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
pub mod daily_macd_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request daily moving average convergence divergence data.
    #[derive(Debug, Clone)]
    pub struct DailyMacdServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DailyMacdServiceClient<tonic::transport::Channel> {
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
    impl<T> DailyMacdServiceClient<T>
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
        ) -> DailyMacdServiceClient<InterceptedService<T, F>>
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
            DailyMacdServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets daily moving average convergence divergence data
        pub async fn daily_macd(
            &mut self,
            request: impl tonic::IntoRequest<super::DailyMacdRequest>,
        ) -> Result<tonic::Response<super::DailyMacdResponse>, tonic::Status> {
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
                "/systemathics.apis.services.daily_analytics.v1.DailyMacdService/DailyMacd",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the DailyVolatilityService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyVolatilityRequest {
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
/// Represents a daily volatility response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyVolatilityResponse {
    /// Volatility value
    #[prost(double, tag="1")]
    pub value: f64,
}
/// Generated client implementations.
pub mod daily_volatility_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request daily volatility data.
    #[derive(Debug, Clone)]
    pub struct DailyVolatilityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DailyVolatilityServiceClient<tonic::transport::Channel> {
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
    impl<T> DailyVolatilityServiceClient<T>
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
        ) -> DailyVolatilityServiceClient<InterceptedService<T, F>>
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
            DailyVolatilityServiceClient::new(
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
        /// Gets daily historical volatility
        pub async fn daily_volatility(
            &mut self,
            request: impl tonic::IntoRequest<super::DailyVolatilityRequest>,
        ) -> Result<tonic::Response<super::DailyVolatilityResponse>, tonic::Status> {
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
                "/systemathics.apis.services.daily_analytics.v1.DailyVolatilityService/DailyVolatility",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the DailyBollingerService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyBollingerRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Mandatory\] The simple moving average window length (period: data points number)
    #[prost(int32, tag="3")]
    pub length: i32,
    /// \[Mandatory\] The standard deviation window length
    #[prost(double, tag="4")]
    pub deviation: f64,
    /// \[Optional\] The corporate action adjustment (dividends).
    /// By default the value is set to false : the split is applied in all cases
    #[prost(bool, tag="5")]
    pub adjustment: bool,
}
/// The daily bars response contains an array of Bollinger bands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyBollingerResponse {
    /// The daily Bollinger bands: an array of DailyBollingerData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<DailyBollingerData>,
}
/// Contains the daily Bollinger bands data: date, lower, middle and upper.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyBollingerData {
    /// Date of Bollinger bands
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
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
pub mod daily_bollinger_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request daily Bollinger bands data.
    #[derive(Debug, Clone)]
    pub struct DailyBollingerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DailyBollingerServiceClient<tonic::transport::Channel> {
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
    impl<T> DailyBollingerServiceClient<T>
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
        ) -> DailyBollingerServiceClient<InterceptedService<T, F>>
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
            DailyBollingerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets daily Bollinger bands data
        pub async fn daily_bollinger(
            &mut self,
            request: impl tonic::IntoRequest<super::DailyBollingerRequest>,
        ) -> Result<tonic::Response<super::DailyBollingerResponse>, tonic::Status> {
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
                "/systemathics.apis.services.daily_analytics.v1.DailyBollingerService/DailyBollinger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required input to request the DailyEmaService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyEmaRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Optional\] The time constraints used to define the look-back period.
    /// If empty, then all the available data is retrieved.
    #[prost(message, optional, tag="2")]
    pub constraints: ::core::option::Option<super::super::super::r#type::shared::v1::Constraints>,
    /// \[Mandatory\] The exponential moving average window length (period : points number)
    #[prost(int32, tag="3")]
    pub length: i32,
    /// \[Optional\] The corporate action adjustment (dividends).
    /// By default the value is set to false : the split is applied in all cases
    #[prost(bool, tag="4")]
    pub adjustment: bool,
}
/// Represents a daily exponential moving average response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyEmaResponse {
    /// The daily exponential moving averages: an array of DailyEmaData objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<DailyEmaData>,
}
/// Contains the daily exponential moving average data: date and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyEmaData {
    /// Date of the exponential moving average
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// The data used to calculate the average of the day
    #[prost(double, tag="2")]
    pub value: f64,
    /// The exponential moving average value of the day
    #[prost(double, tag="3")]
    pub average: f64,
}
/// Generated client implementations.
pub mod daily_ema_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request daily exponential moving average data.
    #[derive(Debug, Clone)]
    pub struct DailyEmaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DailyEmaServiceClient<tonic::transport::Channel> {
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
    impl<T> DailyEmaServiceClient<T>
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
        ) -> DailyEmaServiceClient<InterceptedService<T, F>>
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
            DailyEmaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets daily exponential moving average data
        pub async fn daily_ema(
            &mut self,
            request: impl tonic::IntoRequest<super::DailyEmaRequest>,
        ) -> Result<tonic::Response<super::DailyEmaResponse>, tonic::Status> {
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
                "/systemathics.apis.services.daily_analytics.v1.DailyEmaService/DailyEma",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
