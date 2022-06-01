/// The required input to request the StaticDataService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticDataRequest {
    /// \[Optional\] Asset type
    #[prost(enumeration="AssetType", tag="1")]
    pub asset_type: i32,
    /// \[Optional\] Asset name
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Asset exchange (exchange, primary or operating mic code)
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
    /// \[Optional\] Crypto base currency
    #[prost(message, optional, tag="7")]
    pub crypto_base_currency: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Crypto quote currency
    #[prost(message, optional, tag="8")]
    pub crypto_quote_currency: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Equity sector
    #[prost(message, optional, tag="9")]
    pub equity_sector: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Index if the asset belongs to an index components
    #[prost(message, optional, tag="10")]
    pub index: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Mapping codes: isin, cusip, sedol, cik, figi, figic, reuters, bloomberg, morningstar, etc.
    #[prost(message, optional, tag="11")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Optional\] Define the first element index to be retrieved
    #[prost(message, optional, tag="12")]
    pub start: ::core::option::Option<i32>,
    /// \[Optional\] Select the first elements retrieved, by default count is set to 100
    #[prost(message, optional, tag="13")]
    pub count: ::core::option::Option<i32>,
}
/// Represents a reference data response grouped by asset type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticDataResponse {
    /// The reference data for futures: the futures array
    #[prost(message, repeated, tag="1")]
    pub futures: ::prost::alloc::vec::Vec<FutureEntry>,
    /// The reference data for equities: the equities array
    #[prost(message, repeated, tag="2")]
    pub equities: ::prost::alloc::vec::Vec<EquityEntry>,
    /// The reference data for etfs: the etfs array
    #[prost(message, repeated, tag="3")]
    pub etfs: ::prost::alloc::vec::Vec<EtfEntry>,
    /// The reference data for indices: the indices array
    #[prost(message, repeated, tag="4")]
    pub indices: ::prost::alloc::vec::Vec<IndexEntry>,
    /// The reference data for crypto currencies: the cryptos array
    #[prost(message, repeated, tag="5")]
    pub cryptos: ::prost::alloc::vec::Vec<CryptoEntry>,
    /// The reference data for forex pairs: the forex array
    #[prost(message, repeated, tag="6")]
    pub forex: ::prost::alloc::vec::Vec<ForexEntry>,
}
/// Contains the reference data for equities.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquityEntry {
    /// Identifiers: asset identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// General information: the asset class
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// General information: the primary exchange code (mic)
    #[prost(string, tag="3")]
    pub primary: ::prost::alloc::string::String,
    /// General information: the operating exchange code (mic)
    #[prost(string, tag="4")]
    pub operating: ::prost::alloc::string::String,
    /// General information: the full name 
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    /// General information: the currency code (ISO 4217)
    #[prost(string, tag="6")]
    pub currency: ::prost::alloc::string::String,
    /// General information: the country code (ISO 3166)
    #[prost(string, tag="7")]
    pub country: ::prost::alloc::string::String,
    /// General information: the tick size rule table
    #[prost(message, repeated, tag="8")]
    pub tick_size_rule: ::prost::alloc::vec::Vec<TickSize>,
    /// General information: the major index array
    #[prost(string, repeated, tag="9")]
    pub index: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Trading hours: the open time (continuous trading)
    #[prost(message, optional, tag="10")]
    pub open: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the close time (continuous trading)
    #[prost(message, optional, tag="11")]
    pub close: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the time zone
    #[prost(string, tag="12")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifications: the lot size (minimum quantity to trade)
    #[prost(int64, tag="13")]
    pub lot_size: i64,
    /// Specifications: the point value
    #[prost(double, tag="14")]
    pub point_value: f64,
    /// Reference data : the average price
    #[prost(message, optional, tag="15")]
    pub price: ::core::option::Option<f64>,
    /// Reference data : the average daily volume 
    #[prost(message, optional, tag="16")]
    pub volume: ::core::option::Option<i64>,
    /// Reference data : the time stamp of the refernce data
    #[prost(message, optional, tag="17")]
    pub time: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Mapping : the count of sources used to cross validate and complete the asset's information
    #[prost(int32, tag="18")]
    pub sources: i32,
    /// Mapping: the mapping codes
    #[prost(map="string, string", tag="19")]
    pub mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Mapping: the bloomberg code
    #[prost(string, tag="20")]
    pub bloomberg: ::prost::alloc::string::String,
    /// Mapping: the reuters code
    #[prost(string, tag="21")]
    pub reuters: ::prost::alloc::string::String,
    /// Mapping: the morningstar code
    #[prost(string, tag="22")]
    pub morningstar: ::prost::alloc::string::String,
    /// Mapping: the figi code (Financial Instrument Global Identifier, formerly Bloomberg Global Identifier)
    #[prost(string, tag="23")]
    pub figi: ::prost::alloc::string::String,
    /// Mapping: the figic code (Financial Instrument Global Identifier Composite, formerly Bloomberg Global Identifier)
    #[prost(string, tag="24")]
    pub figic: ::prost::alloc::string::String,
    /// Specific values for the equity: isin code (International Securities Identifying Number)
    #[prost(string, tag="25")]
    pub isin: ::prost::alloc::string::String,
    /// Specific values for the equity: cusip code : Committee on Uniform Security Identification Procedures
    #[prost(string, tag="26")]
    pub cusip: ::prost::alloc::string::String,
    /// Specific values for the equity: sedol code : Stock Exchange Daily Official List
    #[prost(string, tag="27")]
    pub sedol: ::prost::alloc::string::String,
    /// Specific values for the equity: cik (Central Index Key number). The Cik is used as a unique identifier for financial filings with the Security and Exchange Commision of the USA
    #[prost(string, tag="28")]
    pub cik: ::prost::alloc::string::String,
    /// Specific values for the equity: sectors definitions
    #[prost(map="string, string", tag="29")]
    pub sectors: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Specific values for the equity: market capitalization
    #[prost(message, optional, tag="30")]
    pub capitalization: ::core::option::Option<f64>,
    /// Specific values for the equity: the description of the company
    #[prost(string, tag="31")]
    pub description: ::prost::alloc::string::String,
    /// Specific values for the equity: the main address of the company
    #[prost(string, tag="32")]
    pub address: ::prost::alloc::string::String,
    /// Specific values for the equity: the main phone of the company
    #[prost(string, tag="33")]
    pub phone: ::prost::alloc::string::String,
    /// Specific values for the equity: the main email of the company
    #[prost(string, tag="34")]
    pub email: ::prost::alloc::string::String,
    /// Specific values for the equity: the website link of the company
    #[prost(string, tag="35")]
    pub url: ::prost::alloc::string::String,
}
/// Contains the reference data for exchange traded fund (ETF).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtfEntry {
    /// Identifiers: asset identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// General information: the asset class
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// General information: the primary exchange code (mic)
    #[prost(string, tag="3")]
    pub primary: ::prost::alloc::string::String,
    /// General information: the operating exchange code (mic)
    #[prost(string, tag="4")]
    pub operating: ::prost::alloc::string::String,
    /// General information: the full name 
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    /// General information: the currency code (ISO 4217)
    #[prost(string, tag="6")]
    pub currency: ::prost::alloc::string::String,
    /// General information: the country code (ISO 3166)
    #[prost(string, tag="7")]
    pub country: ::prost::alloc::string::String,
    /// General information: the tick size rule table
    #[prost(message, repeated, tag="8")]
    pub tick_size_rule: ::prost::alloc::vec::Vec<TickSize>,
    /// General information: the major index array
    #[prost(string, repeated, tag="9")]
    pub index: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Trading hours: the open time (continuous trading)
    #[prost(message, optional, tag="10")]
    pub open: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the close time (continuous trading)
    #[prost(message, optional, tag="11")]
    pub close: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the time zone
    #[prost(string, tag="12")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifications: the lot size (minimum quantity to trade)
    #[prost(int64, tag="13")]
    pub lot_size: i64,
    /// Specifications: the point value
    #[prost(double, tag="14")]
    pub point_value: f64,
    /// Reference data : the average price
    #[prost(message, optional, tag="15")]
    pub price: ::core::option::Option<f64>,
    /// Reference data : the average daily volume 
    #[prost(message, optional, tag="16")]
    pub volume: ::core::option::Option<i64>,
    /// Reference data : the time stamp of the refernce data
    #[prost(message, optional, tag="17")]
    pub time: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Mapping : the count of sources used to cross validate and complete the asset's information
    #[prost(int32, tag="18")]
    pub sources: i32,
    /// Mapping: the mapping codes
    #[prost(map="string, string", tag="19")]
    pub mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Mapping: the bloomberg code
    #[prost(string, tag="20")]
    pub bloomberg: ::prost::alloc::string::String,
    /// Mapping: the reuters code
    #[prost(string, tag="21")]
    pub reuters: ::prost::alloc::string::String,
    /// Mapping: the morningstar code
    #[prost(string, tag="22")]
    pub morningstar: ::prost::alloc::string::String,
    /// Mapping: the figi code (Financial Instrument Global Identifier, formerly Bloomberg Global Identifier)
    #[prost(string, tag="23")]
    pub figi: ::prost::alloc::string::String,
    /// Mapping: the figic code (Financial Instrument Global Identifier Composite, formerly Bloomberg Global Identifier)
    #[prost(string, tag="24")]
    pub figic: ::prost::alloc::string::String,
    /// Specific values for the etf: isin code (International Securities Identifying Number)
    #[prost(string, tag="25")]
    pub isin: ::prost::alloc::string::String,
    /// Specific values for the etf: cusip code : Committee on Uniform Security Identification Procedures
    #[prost(string, tag="26")]
    pub cusip: ::prost::alloc::string::String,
    /// Specific values for the etf: sedol code : Stock Exchange Daily Official List
    #[prost(string, tag="27")]
    pub sedol: ::prost::alloc::string::String,
    /// Specific values for the etf: cik (Central Index Key number). The Cik is used as a unique identifier for financial filings with the Security and Exchange Commision of the USA
    #[prost(string, tag="28")]
    pub cik: ::prost::alloc::string::String,
    /// Specific values for the etf: sectors definitions
    #[prost(map="string, string", tag="29")]
    pub sectors: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Specific values for the etf: market capitalization
    #[prost(message, optional, tag="30")]
    pub capitalization: ::core::option::Option<f64>,
    /// Specific values for the etf: the description of the exchange traded fund
    #[prost(string, tag="31")]
    pub description: ::prost::alloc::string::String,
    /// Specific values for the etf: the main address of the exchange traded fund
    #[prost(string, tag="32")]
    pub address: ::prost::alloc::string::String,
    /// Specific values for the etf: the main phone of the exchange traded fund
    #[prost(string, tag="33")]
    pub phone: ::prost::alloc::string::String,
    /// Specific values for the etf: the main email of the exchange traded fund
    #[prost(string, tag="34")]
    pub email: ::prost::alloc::string::String,
    /// Specific values for the etf: the website link of the exchange traded fund
    #[prost(string, tag="35")]
    pub url: ::prost::alloc::string::String,
}
/// Contains the reference data for futures.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureEntry {
    /// Identifiers: asset identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// General information: the asset class
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// General information: the primary exchange code (mic)
    #[prost(string, tag="3")]
    pub primary: ::prost::alloc::string::String,
    /// General information: the operating exchange code (mic)
    #[prost(string, tag="4")]
    pub operating: ::prost::alloc::string::String,
    /// General information: the full name 
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    /// General information: the currency code (ISO 4217)
    #[prost(string, tag="6")]
    pub currency: ::prost::alloc::string::String,
    /// General information: the country code (ISO 3166)
    #[prost(string, tag="7")]
    pub country: ::prost::alloc::string::String,
    /// General information: the tick size rule table
    #[prost(message, repeated, tag="8")]
    pub tick_size_rule: ::prost::alloc::vec::Vec<TickSize>,
    /// General information: the major index array
    #[prost(string, repeated, tag="9")]
    pub index: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Trading hours: the open time (continuous trading)
    #[prost(message, optional, tag="10")]
    pub open: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the close time (continuous trading)
    #[prost(message, optional, tag="11")]
    pub close: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the time zone
    #[prost(string, tag="12")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifications: the lot size (minimum quantity to trade)
    #[prost(int64, tag="13")]
    pub lot_size: i64,
    /// Specifications: the point value
    #[prost(double, tag="14")]
    pub point_value: f64,
    /// Reference data : the average price
    #[prost(message, optional, tag="15")]
    pub price: ::core::option::Option<f64>,
    /// Reference data : the average daily volume 
    #[prost(message, optional, tag="16")]
    pub volume: ::core::option::Option<i64>,
    /// Reference data : the time stamp of the refernce data
    #[prost(message, optional, tag="17")]
    pub time: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Mapping : the count of sources used to cross validate and complete the asset's information
    #[prost(int32, tag="18")]
    pub sources: i32,
    /// Mapping: the mapping codes
    #[prost(map="string, string", tag="19")]
    pub mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Mapping: the bloomberg code
    #[prost(string, tag="20")]
    pub bloomberg: ::prost::alloc::string::String,
    /// Mapping: the reuters code
    #[prost(string, tag="21")]
    pub reuters: ::prost::alloc::string::String,
    /// Mapping: the morningstar code
    #[prost(string, tag="22")]
    pub morningstar: ::prost::alloc::string::String,
    /// Mapping: the figi code (Financial Instrument Global Identifier, formerly Bloomberg Global Identifier)
    #[prost(string, tag="23")]
    pub figi: ::prost::alloc::string::String,
    /// Mapping: the figic code (Financial Instrument Global Identifier Composite, formerly Bloomberg Global Identifier)
    #[prost(string, tag="24")]
    pub figic: ::prost::alloc::string::String,
    /// Specific values for the future: the future underlying asset code
    #[prost(string, tag="25")]
    pub underlying: ::prost::alloc::string::String,
    /// Specific values for the future: the future contract code
    #[prost(string, tag="26")]
    pub contract: ::prost::alloc::string::String,
    /// Specific values for the future: the future category
    #[prost(map="string, string", tag="27")]
    pub category: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Specific values for the future: the future chain codes
    #[prost(string, tag="28")]
    pub chain: ::prost::alloc::string::String,
    /// Specific values for the future: the future maturity date
    #[prost(message, optional, tag="29")]
    pub maturity: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Specific values for the future: the future month code
    #[prost(string, tag="30")]
    pub month: ::prost::alloc::string::String,
    /// Specific values for the future: the future year
    #[prost(int32, tag="31")]
    pub year: i32,
}
/// Contains the reference data for indices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexEntry {
    /// Identifiers: asset identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// General information: the asset class
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// General information: the primary exchange code (mic)
    #[prost(string, tag="3")]
    pub primary: ::prost::alloc::string::String,
    /// General information: the operating exchange code (mic)
    #[prost(string, tag="4")]
    pub operating: ::prost::alloc::string::String,
    /// General information: the full name 
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    /// General information: the currency code (ISO 4217)
    #[prost(string, tag="6")]
    pub currency: ::prost::alloc::string::String,
    /// General information: the country code (ISO 3166)
    #[prost(string, tag="7")]
    pub country: ::prost::alloc::string::String,
    /// General information: the tick size rule table
    #[prost(message, repeated, tag="8")]
    pub tick_size_rule: ::prost::alloc::vec::Vec<TickSize>,
    /// General information: the major index array
    #[prost(string, repeated, tag="9")]
    pub index: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Trading hours: the open time (continuous trading)
    #[prost(message, optional, tag="10")]
    pub open: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the close time (continuous trading)
    #[prost(message, optional, tag="11")]
    pub close: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the time zone
    #[prost(string, tag="12")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifications: the lot size (minimum quantity to trade)
    #[prost(int64, tag="13")]
    pub lot_size: i64,
    /// Specifications: the point value
    #[prost(double, tag="14")]
    pub point_value: f64,
    /// Reference data : the average price
    #[prost(message, optional, tag="15")]
    pub price: ::core::option::Option<f64>,
    /// Reference data : the average daily volume 
    #[prost(message, optional, tag="16")]
    pub volume: ::core::option::Option<i64>,
    /// Reference data : the time stamp of the refernce data
    #[prost(message, optional, tag="17")]
    pub time: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Mapping : the count of sources used to cross validate and complete the asset's information
    #[prost(int32, tag="18")]
    pub sources: i32,
    /// Mapping: the mapping codes
    #[prost(map="string, string", tag="19")]
    pub mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Mapping: the bloomberg code
    #[prost(string, tag="20")]
    pub bloomberg: ::prost::alloc::string::String,
    /// Mapping: the reuters code
    #[prost(string, tag="21")]
    pub reuters: ::prost::alloc::string::String,
    /// Mapping: the morningstar code
    #[prost(string, tag="22")]
    pub morningstar: ::prost::alloc::string::String,
    /// Mapping: the figi code (Financial Instrument Global Identifier, formerly Bloomberg Global Identifier)
    #[prost(string, tag="23")]
    pub figi: ::prost::alloc::string::String,
    /// Mapping: the figic code (Financial Instrument Global Identifier Composite, formerly Bloomberg Global Identifier)
    #[prost(string, tag="24")]
    pub figic: ::prost::alloc::string::String,
    /// Specific values for the index: the description of the index
    #[prost(string, tag="25")]
    pub description: ::prost::alloc::string::String,
}
/// Contains the reference data for crypto currencies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoEntry {
    /// Identifiers: asset identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// General information: the asset class
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// General information: the primary exchange code (mic)
    #[prost(string, tag="3")]
    pub primary: ::prost::alloc::string::String,
    /// General information: the operating exchange code (mic)
    #[prost(string, tag="4")]
    pub operating: ::prost::alloc::string::String,
    /// General information: the full name 
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    /// General information: the base currency code (ISO 4217)
    #[prost(string, tag="6")]
    pub base_currency: ::prost::alloc::string::String,
    /// General information: the country code (ISO 3166)
    #[prost(string, tag="7")]
    pub country: ::prost::alloc::string::String,
    /// General information: the tick size rule table
    #[prost(message, repeated, tag="8")]
    pub tick_size_rule: ::prost::alloc::vec::Vec<TickSize>,
    /// General information: the major index array
    #[prost(string, repeated, tag="9")]
    pub index: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Trading hours: the open time (continuous trading)
    #[prost(message, optional, tag="10")]
    pub open: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the close time (continuous trading)
    #[prost(message, optional, tag="11")]
    pub close: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the time zone
    #[prost(string, tag="12")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifications: the lot size (minimum quantity to trade)
    #[prost(int64, tag="13")]
    pub lot_size: i64,
    /// Specifications: the point value
    #[prost(double, tag="14")]
    pub point_value: f64,
    /// Reference data : the average price
    #[prost(message, optional, tag="15")]
    pub price: ::core::option::Option<f64>,
    /// Reference data : the average daily volume 
    #[prost(message, optional, tag="16")]
    pub volume: ::core::option::Option<i64>,
    /// Reference data : the time stamp of the refernce data
    #[prost(message, optional, tag="17")]
    pub time: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Mapping : the count of sources used to cross validate and complete the asset's information
    #[prost(int32, tag="18")]
    pub sources: i32,
    /// Mapping: the mapping codes
    #[prost(map="string, string", tag="19")]
    pub mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Mapping: the bloomberg code
    #[prost(string, tag="20")]
    pub bloomberg: ::prost::alloc::string::String,
    /// Mapping: the reuters code
    #[prost(string, tag="21")]
    pub reuters: ::prost::alloc::string::String,
    /// Mapping: the morningstar code
    #[prost(string, tag="22")]
    pub morningstar: ::prost::alloc::string::String,
    /// Mapping: the figi code (Financial Instrument Global Identifier, formerly Bloomberg Global Identifier)
    #[prost(string, tag="23")]
    pub figi: ::prost::alloc::string::String,
    /// Mapping: the figic code (Financial Instrument Global Identifier Composite, formerly Bloomberg Global Identifier)
    #[prost(string, tag="24")]
    pub figic: ::prost::alloc::string::String,
    /// Specific values for the crypto: the quote currency code
    #[prost(string, tag="25")]
    pub quote_currency: ::prost::alloc::string::String,
    /// Specific values for the crypto: the consensus mechanisms array (POW, POS, DPOS)
    #[prost(string, repeated, tag="26")]
    pub consensus: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Specific values for the crypto: the description of the crypto
    #[prost(string, tag="27")]
    pub description: ::prost::alloc::string::String,
    /// Specific values for the crypto: the issue date of the crypto
    #[prost(message, optional, tag="28")]
    pub issue_date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Specific values for the crypto: the market capitalization
    #[prost(message, optional, tag="29")]
    pub capitalization: ::core::option::Option<f64>,
    /// Specific values for the crypto: the number of coins that are circulating in the market and in the general public's hands
    #[prost(int32, tag="30")]
    pub circulating_supply: i32,
    /// Specific values for the crypto: the maximum amount of coins that will ever exist in the lifetime of the cryptocurrency
    #[prost(int32, tag="31")]
    pub max_supply: i32,
    /// Specific values for the crypto: the total amount of coins in existence right now (minus any coins that have been verifiably burned)
    #[prost(int32, tag="32")]
    pub total_supply: i32,
}
/// Contains the reference data for Forex pairs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForexEntry {
    /// Identifiers: asset identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<super::super::super::r#type::shared::v1::Identifier>,
    /// General information: the asset class
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// General information: the primary exchange code (mic)
    #[prost(string, tag="3")]
    pub primary: ::prost::alloc::string::String,
    /// General information: the operating exchange code (mic)
    #[prost(string, tag="4")]
    pub operating: ::prost::alloc::string::String,
    /// General information: the full name 
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    /// General information: the base currency code (ISO 4217)
    #[prost(string, tag="6")]
    pub base_currency: ::prost::alloc::string::String,
    /// General information: the country code (ISO 3166)
    #[prost(string, tag="7")]
    pub country: ::prost::alloc::string::String,
    /// General information: the tick size rule table
    #[prost(message, repeated, tag="8")]
    pub tick_size_rule: ::prost::alloc::vec::Vec<TickSize>,
    /// General information: the major index array
    #[prost(string, repeated, tag="9")]
    pub index: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Trading hours: the open time (continuous trading)
    #[prost(message, optional, tag="10")]
    pub open: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the close time (continuous trading)
    #[prost(message, optional, tag="11")]
    pub close: ::core::option::Option<::prost_types::Duration>,
    /// Trading hours: the time zone
    #[prost(string, tag="12")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifications: the lot size (minimum quantity to trade)
    #[prost(int64, tag="13")]
    pub lot_size: i64,
    /// Specifications: the point value
    #[prost(double, tag="14")]
    pub point_value: f64,
    /// Reference data : the average price
    #[prost(message, optional, tag="15")]
    pub price: ::core::option::Option<f64>,
    /// Reference data : the average daily volume 
    #[prost(message, optional, tag="16")]
    pub volume: ::core::option::Option<i64>,
    /// Reference data : the time stamp of the refernce data
    #[prost(message, optional, tag="17")]
    pub time: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// Mapping : the count of sources used to cross validate and complete the asset's information
    #[prost(int32, tag="18")]
    pub sources: i32,
    /// Mapping: the mapping codes
    #[prost(map="string, string", tag="19")]
    pub mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Mapping: the bloomberg code
    #[prost(string, tag="20")]
    pub bloomberg: ::prost::alloc::string::String,
    /// Mapping: the reuters code
    #[prost(string, tag="21")]
    pub reuters: ::prost::alloc::string::String,
    /// Mapping: the morningstar code
    #[prost(string, tag="22")]
    pub morningstar: ::prost::alloc::string::String,
    /// Mapping: the figi code (Financial Instrument Global Identifier, formerly Bloomberg Global Identifier)
    #[prost(string, tag="23")]
    pub figi: ::prost::alloc::string::String,
    /// Mapping: the figic code (Financial Instrument Global Identifier Composite, formerly Bloomberg Global Identifier)
    #[prost(string, tag="24")]
    pub figic: ::prost::alloc::string::String,
    /// Specific values for the Forex: the quote currency code
    #[prost(string, tag="25")]
    pub quote_currency: ::prost::alloc::string::String,
}
/// Contains the tick size rule item : price and tick.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickSize {
    /// The price level
    #[prost(double, tag="1")]
    pub price: f64,
    /// The tick size
    #[prost(double, tag="2")]
    pub tick: f64,
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
    /// Search only on Crypto currencies
    Crypto = 6,
    /// Search only on Forex pairs
    Forex = 7,
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
