/// The required input to request the TopologiesService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopologiesRequest {
    /// \[Mandatory\] The instrument identifier: a ticker and exchange
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// \[Mandatory\] Level representing either Trades only(Level 1) or Trades + Book (Level 1 and Level 2)
    #[prost(enumeration="super::super::super::r#type::shared::v1::Level", tag="2")]
    pub level: i32,
    /// \[Mandatory\] Granularity of the request (daily, weekly, ...)
    #[prost(enumeration="TopologyGranularity", tag="3")]
    pub granularity: i32,
    /// [To do] Start at 0 when using pagination or null
    #[prost(message, optional, tag="4")]
    pub start: ::core::option::Option<i32>,
    /// [To do] Define your count of element received when using pagination or null 
    #[prost(message, optional, tag="5")]
    pub count: ::core::option::Option<i32>,
}
/// Contains the number of ticks between 2 dates
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopologyEntry {
    /// Begin date (included)
    #[prost(message, optional, tag="1")]
    pub begin: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// End date (included)
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Tick count
    #[prost(uint64, tag="3")]
    pub ticks_count: u64,
}
/// Represents a topology response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopologiesResponse {
    /// The toplogoly data: an array of TopologyEntry objects
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<TopologyEntry>,
    /// If using paginated version will return true until everything is returned, it's false otherwise
    #[prost(bool, tag="2")]
    pub has_more: bool,
}
/// Contains the topology granularity
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TopologyGranularity {
    /// Unspecified granularity.
    Unspecified = 0,
    /// Daily granularity.
    Daily = 1,
    /// Weekly granularity.
    Weekly = 2,
    /// Monthly granularity.
    Monthly = 3,
    /// Quaterly granularity.
    Quaterly = 4,
    /// Yearly granularity.
    Yearly = 5,
}
/// Generated client implementations.
pub mod topologies_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request topology over a look back-period with a given time granularity.
    #[derive(Debug, Clone)]
    pub struct TopologiesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TopologiesServiceClient<tonic::transport::Channel> {
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
    impl<T> TopologiesServiceClient<T>
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
        ) -> TopologiesServiceClient<InterceptedService<T, F>>
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
            TopologiesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets topology per a given time granularity
        pub async fn topologies(
            &mut self,
            request: impl tonic::IntoRequest<super::TopologiesRequest>,
        ) -> Result<tonic::Response<super::TopologiesResponse>, tonic::Status> {
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
                "/systemathics.apis.services.topology.v1.TopologiesService/Topologies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
