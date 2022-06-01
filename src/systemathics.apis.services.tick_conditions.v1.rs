/// The required inputs to request the TickConditionsService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickConditionsRequest {
    #[prost(oneof="tick_conditions_request::Value", tags="1, 2, 3")]
    pub value: ::core::option::Option<tick_conditions_request::Value>,
}
/// Nested message and enum types in `TickConditionsRequest`.
pub mod tick_conditions_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// \[Optional\] The instrument identifier: a ticker and exchange
        #[prost(message, tag="1")]
        Identifier(super::super::super::super::r#type::shared::v1::Identifier),
        /// \[Optional\] The market identifier code or exchange
        #[prost(string, tag="2")]
        Exchange(::prost::alloc::string::String),
        /// \[Optional\] The source number
        #[prost(int32, tag="3")]
        Source(i32),
    }
}
/// Represents a tick conditions response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickConditionsResponse {
    /// The conditions: an array of Condition objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<super::super::super::r#type::shared::v1::Condition>,
}
/// Generated client implementations.
pub mod tick_conditions_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request tick by tick trading conditions data.
    #[derive(Debug, Clone)]
    pub struct TickConditionsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TickConditionsServiceClient<tonic::transport::Channel> {
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
    impl<T> TickConditionsServiceClient<T>
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
        ) -> TickConditionsServiceClient<InterceptedService<T, F>>
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
            TickConditionsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets tick by tick trading conditions data
        pub async fn tick_conditions(
            &mut self,
            request: impl tonic::IntoRequest<super::TickConditionsRequest>,
        ) -> Result<tonic::Response<super::TickConditionsResponse>, tonic::Status> {
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
                "/systemathics.apis.services.tick_conditions.v1.TickConditionsService/TickConditions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
