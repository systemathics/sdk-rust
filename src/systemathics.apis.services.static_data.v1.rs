/// The required input to request the StaticDataService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticDataRequest {
    /// \[Optional\] Asset type
    #[prost(enumeration="AssetType", tag="1")]
    pub asset_type: i32,
    /// \[Optional\] Asset name
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Asset exchange (mic code)
    #[prost(message, optional, tag="3")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Asset ticker (code as provided by the exchange)
    #[prost(message, optional, tag="4")]
    pub ticker: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Future contract code
    #[prost(message, optional, tag="5")]
    pub future_contract: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Future category
    #[prost(message, optional, tag="6")]
    pub future_category: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Equity sector
    #[prost(message, optional, tag="7")]
    pub equity_sector: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Index if the asset belongs to an index components
    #[prost(message, optional, tag="8")]
    pub index: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Mapping codes: isin, cusip, sedol, ric, figi, etc.
    #[prost(message, optional, tag="9")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Define the first element index to be retrieved
    #[prost(message, optional, tag="10")]
    pub start: ::core::option::Option<i32>,
    /// \[Optional\] Select the first elements retrieved, by default count is set to 100
    #[prost(message, optional, tag="11")]
    pub count: ::core::option::Option<i32>,
}
/// Represents a reference data response grouped by asset type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticDataResponse {
    /// The reference data for futures: an array of FutureEntry objects
    #[prost(message, repeated, tag="1")]
    pub futures: ::prost::alloc::vec::Vec<FutureEntry>,
    /// The reference data for equities: an array of EquityEntry objects
    #[prost(message, repeated, tag="2")]
    pub equities: ::prost::alloc::vec::Vec<EquityEntry>,
    /// The reference data for etfs: an array of EtfEntry objects
    #[prost(message, repeated, tag="3")]
    pub etfs: ::prost::alloc::vec::Vec<EtfEntry>,
    /// The reference data for indices: an array of IndexEntry objects
    #[prost(message, repeated, tag="4")]
    pub indices: ::prost::alloc::vec::Vec<IndexEntry>,
}
/// Contains the reference data for equities.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquityEntry {
    /// Identifiers: asset identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// General information: asset class
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// General information: asset country code (ISO 3166)
    #[prost(string, tag="3")]
    pub country: ::prost::alloc::string::String,
    /// General information: asset full name 
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// General information: asset currency code (ISO 4217)
    #[prost(string, tag="5")]
    pub currency: ::prost::alloc::string::String,
    /// General information: asset primary exchange code (mic)
    #[prost(string, tag="6")]
    pub primary: ::prost::alloc::string::String,
    /// General information: asset tick size rule table
    #[prost(string, tag="7")]
    pub tick_size_rule: ::prost::alloc::string::String,
    /// General information: asset mapping codes
    #[prost(map="string, string", tag="8")]
    pub mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// General information: asset index if it belongs to an index
    #[prost(string, tag="9")]
    pub index: ::prost::alloc::string::String,
    /// Trading hours: open time (continuous trading)
    #[prost(string, tag="10")]
    pub open: ::prost::alloc::string::String,
    /// Trading hours: close time (continuous trading)
    #[prost(string, tag="11")]
    pub close: ::prost::alloc::string::String,
    /// Trading hours: time zone
    #[prost(string, tag="12")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifications: lot size (minimum quantity to trade)
    #[prost(int64, tag="13")]
    pub lot_size: i64,
    /// Specifications: point value
    #[prost(double, tag="14")]
    pub point_value: f64,
    /// Specific values for the equity: isin code (International Securities Identifying Number)
    #[prost(string, tag="15")]
    pub isin: ::prost::alloc::string::String,
    /// Specific values for the equity: cusip code : Committee on Uniform Security Identification Procedures
    #[prost(string, tag="16")]
    pub cusip: ::prost::alloc::string::String,
    /// Specific values for the equity: sedol code : Stock Exchange Daily Official List
    #[prost(string, tag="17")]
    pub sedol: ::prost::alloc::string::String,
    /// Specific values for the equity: sectors definitions
    #[prost(map="string, string", tag="18")]
    pub sectors: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Specific values for the equity: market capitalization
    #[prost(message, optional, tag="19")]
    pub capitalization: ::core::option::Option<f64>,
}
/// Contains the reference data for exchange traded fund (ETF).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtfEntry {
    /// Identifiers: asset identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// General information: asset class
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// General information: asset country code (ISO 3166)
    #[prost(string, tag="3")]
    pub country: ::prost::alloc::string::String,
    /// General information: asset full name 
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// General information: asset currency code (ISO 4217)
    #[prost(string, tag="5")]
    pub currency: ::prost::alloc::string::String,
    /// General information: asset primary exchange code (mic)
    #[prost(string, tag="6")]
    pub primary: ::prost::alloc::string::String,
    /// General information: asset tick size rule table
    #[prost(string, tag="7")]
    pub tick_size_rule: ::prost::alloc::string::String,
    /// General information: asset mapping codes
    #[prost(map="string, string", tag="8")]
    pub mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// General information: asset index if it belongs to an index
    #[prost(string, tag="9")]
    pub index: ::prost::alloc::string::String,
    /// Trading hours: open time (continuous trading)
    #[prost(string, tag="10")]
    pub open: ::prost::alloc::string::String,
    /// Trading hours: close time (continuous trading)
    #[prost(string, tag="11")]
    pub close: ::prost::alloc::string::String,
    /// Trading hours: time zone
    #[prost(string, tag="12")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifications: lot size (minimum quantity to trade)
    #[prost(int64, tag="13")]
    pub lot_size: i64,
    /// Specifications: point value
    #[prost(double, tag="14")]
    pub point_value: f64,
    /// Specific values for the equity: isin code (International Securities Identifying Number)
    #[prost(string, tag="15")]
    pub isin: ::prost::alloc::string::String,
    /// Specific values for the equity: cusip code : Committee on Uniform Security Identification Procedures
    #[prost(string, tag="16")]
    pub cusip: ::prost::alloc::string::String,
    /// Specific values for the equity: sedol code : Stock Exchange Daily Official List
    #[prost(string, tag="17")]
    pub sedol: ::prost::alloc::string::String,
    /// Specific values for the equity: sectors definitions
    #[prost(map="string, string", tag="18")]
    pub sectors: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Specific values for the equity: market capitalization
    #[prost(message, optional, tag="19")]
    pub capitalization: ::core::option::Option<f64>,
}
/// Contains the reference data for futures.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureEntry {
    /// Identifiers: asset identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// General information: asset class
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// General information: asset country code (ISO 3166)
    #[prost(string, tag="3")]
    pub country: ::prost::alloc::string::String,
    /// General information: asset full name 
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// General information: asset currency code (ISO 4217)
    #[prost(string, tag="5")]
    pub currency: ::prost::alloc::string::String,
    /// General information: asset primary exchange code (mic)
    #[prost(string, tag="6")]
    pub primary: ::prost::alloc::string::String,
    /// General information: asset tick size rule table
    #[prost(string, tag="7")]
    pub tick_size_rule: ::prost::alloc::string::String,
    /// General information: asset mapping codes
    #[prost(map="string, string", tag="8")]
    pub mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// General information: asset index if it belongs to an index
    #[prost(string, tag="9")]
    pub index: ::prost::alloc::string::String,
    /// Trading hours: open time (continuous trading)
    #[prost(string, tag="10")]
    pub open: ::prost::alloc::string::String,
    /// Trading hours: close time (continuous trading)
    #[prost(string, tag="11")]
    pub close: ::prost::alloc::string::String,
    /// Trading hours: time zone
    #[prost(string, tag="12")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifications: lot size (minimum quantity to trade)
    #[prost(int64, tag="13")]
    pub lot_size: i64,
    /// Specifications: point value
    #[prost(double, tag="14")]
    pub point_value: f64,
    /// Specific values for the future: underlying asset code
    #[prost(string, tag="15")]
    pub underlying: ::prost::alloc::string::String,
    /// Specific values for the future: contract code
    #[prost(string, tag="16")]
    pub contract: ::prost::alloc::string::String,
    /// Specific values for the future: category
    #[prost(map="string, string", tag="17")]
    pub category: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Specific values for the future: chain
    #[prost(string, tag="18")]
    pub chain: ::prost::alloc::string::String,
    /// Specific values for the future: maturity date
    #[prost(message, optional, tag="19")]
    pub maturity: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Specific values for the future: month code
    #[prost(string, tag="20")]
    pub month: ::prost::alloc::string::String,
    /// Specific values for the future: year
    #[prost(int32, tag="21")]
    pub year: i32,
}
/// Contains the reference data for indices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexEntry {
    /// Identifiers: asset identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// General information: asset class
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// General information: asset country code (ISO 3166)
    #[prost(string, tag="3")]
    pub country: ::prost::alloc::string::String,
    /// General information: asset full name 
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// General information: asset currency code (ISO 4217)
    #[prost(string, tag="5")]
    pub currency: ::prost::alloc::string::String,
    /// General information: asset primary exchange code (mic)
    #[prost(string, tag="6")]
    pub primary: ::prost::alloc::string::String,
    /// General information: asset tick size rule table
    #[prost(string, tag="7")]
    pub tick_size_rule: ::prost::alloc::string::String,
    /// General information: asset mapping codes
    #[prost(map="string, string", tag="8")]
    pub mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// General information: asset index if it belongs to an index
    #[prost(string, tag="9")]
    pub index: ::prost::alloc::string::String,
    /// Trading hours: open time (continuous trading)
    #[prost(string, tag="10")]
    pub open: ::prost::alloc::string::String,
    /// Trading hours: close time (continuous trading)
    #[prost(string, tag="11")]
    pub close: ::prost::alloc::string::String,
    /// Trading hours: time zone
    #[prost(string, tag="12")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifications: lot size (minimum quantity to trade)
    #[prost(int64, tag="13")]
    pub lot_size: i64,
    /// Specifications: point value
    #[prost(double, tag="14")]
    pub point_value: f64,
    /// Specific values for the index: the description of the index
    #[prost(string, tag="15")]
    pub description: ::prost::alloc::string::String,
}
/// Contains the asset type for the reference data request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    /// Not specified ?
    Unspecified = 0,
    /// Search on all asset types
    All = 1,
    /// Search only on equities
    Equity = 2,
    /// Search only on futures
    Future = 3,
    /// Search only on Exchange Traded Fund
    Etf = 4,
    /// Search only on Index
    Index = 5,
}
/// Generated client implementations.
pub mod static_data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request reference data.
    #[derive(Debug, Clone)]
    pub struct StaticDataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StaticDataServiceClient<tonic::transport::Channel> {
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
    impl<T> StaticDataServiceClient<T>
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
        ) -> StaticDataServiceClient<InterceptedService<T, F>>
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
            StaticDataServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets reference data
        pub async fn static_data(
            &mut self,
            request: impl tonic::IntoRequest<super::StaticDataRequest>,
        ) -> Result<tonic::Response<super::StaticDataResponse>, tonic::Status> {
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
                "/systemathics.apis.services.static_data.v1.StaticDataService/StaticData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The required inputs to request the StaticSectorService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticSectorRequest {
    /// \[Mandatory\] The classification provider or taxonomy
    /// <br>Available classifications : ICB, SIC, TRBC and GICS
    #[prost(string, tag="1")]
    pub provider: ::prost::alloc::string::String,
    #[prost(oneof="static_sector_request::Value", tags="2, 3")]
    pub value: ::core::option::Option<static_sector_request::Value>,
}
/// Nested message and enum types in `StaticSectorRequest`.
pub mod static_sector_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// \[Mandatory\] The classification code
        #[prost(string, tag="2")]
        Code(::prost::alloc::string::String),
        /// \[Mandatory\] The classification level (strating from 0 : the top structure or group)
        #[prost(int32, tag="3")]
        Level(i32),
    }
}
/// Represents a sector classifcation response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticSectorResponse {
    /// The sector classification levels: an array of Level objects
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Level>,
    /// The classification description
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// The classification levels count
    #[prost(int32, tag="3")]
    pub count: i32,
}
/// Contains the sector classification structure at a specified level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Level {
    /// The sector classification level's name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The sector classification level's index (order of the strcture)
    #[prost(int32, tag="2")]
    pub index: i32,
    /// The sector classification level's code
    #[prost(string, tag="3")]
    pub code: ::prost::alloc::string::String,
    /// The sector classification level's definition
    #[prost(string, tag="4")]
    pub definition: ::prost::alloc::string::String,
    /// The sector classification level's label
    #[prost(string, tag="5")]
    pub label: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod static_sector_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Called to request static sector and industry classification data.
    #[derive(Debug, Clone)]
    pub struct StaticSectorServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StaticSectorServiceClient<tonic::transport::Channel> {
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
    impl<T> StaticSectorServiceClient<T>
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
        ) -> StaticSectorServiceClient<InterceptedService<T, F>>
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
            StaticSectorServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets static sector and industry data
        pub async fn static_sector(
            &mut self,
            request: impl tonic::IntoRequest<super::StaticSectorRequest>,
        ) -> Result<tonic::Response<super::StaticSectorResponse>, tonic::Status> {
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
                "/systemathics.apis.services.static_data.v1.StaticSectorService/StaticSector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
