/// The required inputs to request the SustainabilityService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SustainabilityRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
}
/// Represents a sustainability response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SustainabilityResponse {
    /// The sustainabilities: an array of sustainability objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Sustainability>,
}
/// Contains the sustainabilitys's data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sustainability {
    /// The sustainability provider
    #[prost(string, tag="1")]
    pub provider: ::prost::alloc::string::String,
    /// The sustainability name or label
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The sustainability identifier
    #[prost(string, tag="3")]
    pub id: ::prost::alloc::string::String,
    /// The sustainability parent identifier
    #[prost(string, tag="4")]
    pub parent: ::prost::alloc::string::String,
    /// The sustainability value
    #[prost(double, tag="5")]
    pub value: f64,
    /// The sustainability minimum value
    #[prost(double, tag="6")]
    pub min: f64,
    /// The sustainability maximum value
    #[prost(double, tag="7")]
    pub max: f64,
    /// The sustainability description
    #[prost(string, tag="8")]
    pub description: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod sustainability_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request sustainability data.
    #[derive(Debug, Clone)]
    pub struct SustainabilityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SustainabilityServiceClient<tonic::transport::Channel> {
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
    impl<T> SustainabilityServiceClient<T>
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
        ) -> SustainabilityServiceClient<InterceptedService<T, F>>
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
            SustainabilityServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets sustainability data
        pub async fn sustainability(
            &mut self,
            request: impl tonic::IntoRequest<super::SustainabilityRequest>,
        ) -> Result<tonic::Response<super::SustainabilityResponse>, tonic::Status> {
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
                "/systemathics.apis.services.sustainability.v1.SustainabilityService/Sustainability",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
