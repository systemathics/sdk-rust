/// The required input to request the ComponentsService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentsRequest {
    /// \[Mandatory\] The index name
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
}
/// Represents a components response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentsResponse {
    /// The index components: an array of component objects
    #[prost(message, repeated, tag="1")]
    pub components: ::prost::alloc::vec::Vec<Component>,
}
/// Contains the weight data for one component: identifier and weight value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Component {
    /// The instrument identifier: a ticker and exchange 
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// The weight value
    #[prost(double, tag="2")]
    pub weight: f64,
}
/// Generated client implementations.
pub mod components_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request index weights data.
    #[derive(Debug, Clone)]
    pub struct ComponentsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ComponentsServiceClient<tonic::transport::Channel> {
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
    impl<T> ComponentsServiceClient<T>
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
        ) -> ComponentsServiceClient<InterceptedService<T, F>>
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
            ComponentsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets index components data
        pub async fn components(
            &mut self,
            request: impl tonic::IntoRequest<super::ComponentsRequest>,
        ) -> Result<tonic::Response<super::ComponentsResponse>, tonic::Status> {
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
                "/systemathics.apis.services.indices.v1.ComponentsService/Components",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
