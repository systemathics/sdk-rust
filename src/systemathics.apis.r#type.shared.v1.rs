/// Contains the instrument identifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identifier {
    /// Exchange market identifier code (MIC defined in ISO-20022)
    #[prost(string, tag="1")]
    pub exchange: ::prost::alloc::string::String,
    /// Ticker name, associated with market idenfier code defines a unique asset
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// Optional provider tick data name if several tick data providers are used
    #[prost(message, optional, tag="3")]
    pub provider: ::core::option::Option<::prost::alloc::string::String>,
}
/// Contains date interval with start date (included) and end date (excluded).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateInterval {
    /// Start date of the date interval. It must always be smaller than end date
    #[prost(message, optional, tag="1")]
    pub start_date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
    /// End date of the date interval. It must always be greater than start date
    #[prost(message, optional, tag="2")]
    pub end_date: ::core::option::Option<super::super::super::super::super::google::r#type::Date>,
}
/// Contains time interval with start_time (included) and end_time (excluded)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeInterval {
    /// Start time of the time interval. It must always be smaller than end time
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<super::super::super::super::super::google::r#type::TimeOfDay>,
    /// End time of the time interval. It must always be greater than start time
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<super::super::super::super::super::google::r#type::TimeOfDay>,
}
/// <br>- Zero, one or many [Date intervals](#DateInterval). The union of each date interval is used.
/// <br>- Zero, one or many [Time intervals](#TimeInterval). The union of each time interval is used.
/// <br>- Zero, one or many [Excluded dates](#google.type.Date). The union of each excluded dates is used.
/// <br>- Zero, one or many [Excluded days](#google.type.DayOfWeek). The union of each excluded days is used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraints {
    /// Restricts request to an union of date_intervals
    /// <br>Providing no date_intervals will return all the stored data
    #[prost(message, repeated, tag="1")]
    pub date_intervals: ::prost::alloc::vec::Vec<DateInterval>,
    /// Restricts request to an union of time_intervals. (ie: Market hours, trading hours, ...)
    /// <br>Note: Time intervals including midnight needs to be described as an union of time intervals (ie : \[22:00-23:59\] U \[00:00-04:00\])
    #[prost(message, repeated, tag="2")]
    pub time_intervals: ::prost::alloc::vec::Vec<TimeInterval>,
    /// Restricts request to an union of excluded_dates (ie: closed Days, holidays, ...).
    #[prost(message, repeated, tag="3")]
    pub excluded_dates: ::prost::alloc::vec::Vec<super::super::super::super::super::google::r#type::Date>,
    /// Restricts request to an union of excluded_days (ie: weekd-end can be represented with Saturday, Sunday)
    #[prost(enumeration="super::super::super::super::super::google::r#type::DayOfWeek", repeated, tag="4")]
    pub excluded_days: ::prost::alloc::vec::Vec<i32>,
}
/// Contains the level types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Level {
    /// The unspecfied level 
    Unspecified = 0,
    /// The trades only
    Trades = 1,
    /// The trades and book
    TradesAndBook = 2,
}
/// The tick key : mapping between the identifier and the key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    /// The instrument identifier
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<Identifier>,
    /// The instrument mapping key: short code for instrument identifier
    #[prost(uint32, tag="2")]
    pub value: u32,
}
/// The mapping keys array
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keys {
    /// An array of mapping keys
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<Key>,
}
/// Contains the trade details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    /// The price of the trade
    #[prost(double, tag="1")]
    pub price: f64,
    /// The size (volume) of the trade
    #[prost(int64, tag="2")]
    pub size: i64,
    /// The trading condition of the trade
    #[prost(string, tag="3")]
    pub condition: ::prost::alloc::string::String,
    /// The market unique identifier of the trade
    #[prost(string, tag="4")]
    pub id: ::prost::alloc::string::String,
}
/// Contains the trade details and the time stamp.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeData {
    /// The instrument mapping key: short code for instrument identifier.
    #[prost(uint32, tag="1")]
    pub mapping: u32,
    /// The time stamp of the trade (UTC)
    #[prost(message, optional, tag="2")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The trade
    #[prost(message, optional, tag="3")]
    pub trade: ::core::option::Option<Trade>,
}
/// Contains the instrument identifier and stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifierAndLevel {
    /// \[Mandatory\] Exchange market identifier code (MIC defined in ISO-20022).
    #[prost(string, tag="1")]
    pub exchange: ::prost::alloc::string::String,
    /// \[Mandatory\] Ticker name, associated with market idenfier code defines a unique asset.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// \[Optional\] tick data provider name if several tick data providers are available.
    #[prost(message, optional, tag="3")]
    pub provider: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Mandatory\] The data level.
    #[prost(enumeration="Level", tag="4")]
    pub level: i32,
}
/// Contains the types of updates book services are able to deliver
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BookUpdates {
    /// The unspecfied update type
    Unspecified = 0,
    /// Snapshots of the book on each update
    SnapshotsOnly = 1,
    /// Initial Snapshot then incremental updates on each update
    Incrementals = 2,
}
/// Contains side values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Side {
    /// The unspecfied side 
    Unspecified = 0,
    /// The bid side
    Bid = 1,
    /// The ask side
    Ask = 2,
}
/// Contains the quote details : buy or sell order.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quote {
    /// The unique identifier of the quote
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The size of the quote
    #[prost(message, optional, tag="2")]
    pub size: ::core::option::Option<i64>,
    /// The price of the quote 
    #[prost(message, optional, tag="3")]
    pub price: ::core::option::Option<f64>,
    /// The condition of the quote :
    ///<br> 0 : Limit Order
    ///<br> 2 : Market Order
    ///<br> 3 : Market To Limit Order
    ///<br> Note: Market orders will not have a price field, only a size. Similarly for Market To Limit orders during auction phases.
    #[prost(message, optional, tag="4")]
    pub condition: ::core::option::Option<i32>,
    /// The side : Buy (bid) or Sell (ask) order
    #[prost(enumeration="Side", tag="5")]
    pub side: i32,
}
/// The limit : bid or ask price and size at a given depth
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Limit {
    /// The price of the limit
    #[prost(double, tag="1")]
    pub price: f64,
    /// The size (volume) of the limit
    #[prost(int64, tag="2")]
    pub size: i64,
    /// The depth of the limit
    #[prost(uint32, tag="3")]
    pub depth: u32,
    /// The contributors
    #[prost(message, repeated, tag="4")]
    pub quotes: ::prost::alloc::vec::Vec<Quote>,
}
/// The book : bid and ask arrays
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Book {
    /// The bid array : buy side
    #[prost(message, repeated, tag="1")]
    pub bid: ::prost::alloc::vec::Vec<Limit>,
    /// The ask array : sell side
    #[prost(message, repeated, tag="2")]
    pub ask: ::prost::alloc::vec::Vec<Limit>,
}
/// Contains the book or trade data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeAndBookData {
    /// The instrument mapping key: short code for instrument identifier.
    #[prost(uint32, tag="1")]
    pub mapping: u32,
    /// The time stamp of the trade and book (UTC)
    #[prost(message, optional, tag="2")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof="trade_and_book_data::Payload", tags="3, 4")]
    pub payload: ::core::option::Option<trade_and_book_data::Payload>,
}
/// Nested message and enum types in `TradeAndBookData`.
pub mod trade_and_book_data {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The trade
        #[prost(message, tag="3")]
        Trade(super::Trade),
        /// The book
        #[prost(message, tag="4")]
        Book(super::Book),
    }
}
/// Contains the primary data, not processed or normalized (identical to provider's data).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Raw {
    /// The instrument mapping key: short code for instrument identifier.
    #[prost(uint32, tag="1")]
    pub mapping: u32,
    /// The time stamp of the raw data (UTC)
    #[prost(message, optional, tag="2")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The data type.
    #[prost(uint32, tag="3")]
    pub r#type: u32,
    /// The payload data.
    #[prost(bytes="vec", tag="4")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// Contains the market Data update action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    /// The unspecfied action
    Unspecified = 0,
    /// The set action
    Set = 1,
    /// The clear action
    Clear = 2,
}
/// Contains the field tyoes for <a href="#systemathics/apis/type/v1/market_fields_updates.proto">a market field update</a>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Field {
    /// The unspecfied field
    Unspecified = 0,
    /// The open price (associated value type is double)
    OpenPrice = 1,
    /// The open size (associated value type is long)
    OpenSize = 2,
    /// The close price (associated value type is double)
    ClosePrice = 3,
    /// The close size (associated value type is long)
    CloseSize = 4,
    /// The low price (associated value type is double)
    LowPrice = 5,
    /// The low size (associated value type is long)
    LowSize = 6,
    /// The high price (associated value type is double)
    HighPrice = 7,
    /// The high size (associated value type is long)
    HighSize = 8,
    /// The last trade price (associated value type is double)
    TradePrice = 9,
    /// The last trade size (associated value type is long)
    TradeSize = 10,
    /// The trading condition (associated value type is string)
    TradingCondition = 11,
    /// The trade count (associated value type is long)
    TradeCount = 12,
    /// The last otc trade price (associated value type is double)
    TradeOtcPrice = 13,
    /// The last otc trade size (associated value type is long)
    TradeOtcSize = 14,
    /// The otc trade count (associated value type is long)
    TradeOtcCount = 15,
    /// The daily volume (associated value type is long)
    Volume = 16,
    /// The official vwap (associated value type is double)
    VwapOfficial = 17,
    /// The electronic vwap (associated value type is double)
    VwapElectronic = 18,
    /// The theoretical price (associated value type is double)
    TheoreticalPrice = 19,
    /// The theoretical size (associated value type is long)
    TheoreticalSize = 20,
    /// The upper price (associated value type is double)
    UpperPrice = 21,
    /// The lower price (associated value type is double)
    LowerPrice = 22,
    /// The variation (associated value type is double)
    Variation = 23,
    /// The percentage variation (associated value type is double)
    VariationPercentage = 24,
    /// The trade id (associated value type is string)
    TradeId = 25,
}
/// Represents market field updates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketFieldsUpdates {
    /// The instrument mapping key: short code for instrument identifier.
    #[prost(uint32, tag="1")]
    pub mapping: u32,
    /// Timestamp from the market
    #[prost(message, optional, tag="2")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// If set, previous data must be cleared entirely, otherwise current data is an increment on top of previous data
    #[prost(bool, tag="3")]
    pub reset: bool,
    /// The market book updates: an array of MarketFieldUpdate objects
    #[prost(message, repeated, tag="4")]
    pub updates: ::prost::alloc::vec::Vec<MarketFieldUpdate>,
}
/// Represents a market field update
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketFieldUpdate {
    /// Action on market field update
    #[prost(enumeration="Action", tag="1")]
    pub action: i32,
    /// Field of market field update
    #[prost(enumeration="Field", tag="2")]
    pub field: i32,
    /// Value of the market field update
    #[prost(oneof="market_field_update::Value", tags="3, 4, 5")]
    pub value: ::core::option::Option<market_field_update::Value>,
}
/// Nested message and enum types in `MarketFieldUpdate`.
pub mod market_field_update {
    /// Value of the market field update
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// long value (depends of the field)
        #[prost(int64, tag="3")]
        LongValue(i64),
        /// double value (depends of the field)
        #[prost(double, tag="4")]
        DoubleValue(f64),
        /// string value (depends of the field)
        #[prost(string, tag="5")]
        StringValue(::prost::alloc::string::String),
    }
}
/// Contains Market Book By Limit updates data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MblMarketBookUpdates {
    /// The instrument mapping key: short code for instrument identifier.
    #[prost(uint32, tag="1")]
    pub mapping: u32,
    /// Time stamp from the market (UTC)
    #[prost(message, optional, tag="2")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// If set, previous data must be cleared entirely, otherwise current data is an increment on top of previous data
    #[prost(bool, tag="3")]
    pub reset: bool,
    /// The market book updates: an array of MarketBookUpdate objects
    #[prost(message, repeated, tag="4")]
    pub updates: ::prost::alloc::vec::Vec<MblMarketBookUpdate>,
}
/// Represents a Market Book By Limit update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MblMarketBookUpdate {
    /// Action of the market book update
    #[prost(enumeration="Action", tag="1")]
    pub action: i32,
    /// Side of the market book update
    #[prost(enumeration="Side", tag="2")]
    pub side: i32,
    /// Depth of the market book update
    #[prost(uint32, tag="3")]
    pub depth: u32,
    /// Size of the market book update
    #[prost(message, optional, tag="4")]
    pub size: ::core::option::Option<i64>,
    /// Price of the market book update 
    #[prost(message, optional, tag="5")]
    pub price: ::core::option::Option<f64>,
}
/// Contains Market Book by Order updates data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MboMarketBookUpdates {
    /// The instrument mapping key: short code for instrument identifier.
    #[prost(uint32, tag="1")]
    pub mapping: u32,
    /// Timestamp from the market
    #[prost(message, optional, tag="2")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// If set, previous data must be cleared entirely, otherwise current data is an increment on top of previous data
    #[prost(bool, tag="3")]
    pub reset: bool,
    /// The market book updates: an array of MarketBookUpdate objects
    #[prost(message, repeated, tag="4")]
    pub updates: ::prost::alloc::vec::Vec<MboMarketBookUpdate>,
}
/// Represents a Market Book by Order update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MboMarketBookUpdate {
    /// Action of the market book update
    #[prost(enumeration="Action", tag="1")]
    pub action: i32,
    /// Side of the market book update
    #[prost(enumeration="Side", tag="2")]
    pub side: i32,
    /// Order or Quote unique identifier
    #[prost(string, tag="3")]
    pub order_id: ::prost::alloc::string::String,
    /// Size of the market book update
    #[prost(message, optional, tag="4")]
    pub size: ::core::option::Option<i64>,
    /// Price of the market book update 
    #[prost(message, optional, tag="5")]
    pub price: ::core::option::Option<f64>,
    /// Condition of the market book update
    #[prost(message, optional, tag="6")]
    pub condition: ::core::option::Option<i32>,
}
/// Contains the market book data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookData {
    /// The instrument mapping key: short code for instrument identifier.
    #[prost(uint32, tag="1")]
    pub mapping: u32,
    /// The time stamp of the book
    #[prost(message, optional, tag="2")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The book : bids and asks
    #[prost(message, optional, tag="4")]
    pub book: ::core::option::Option<Book>,
}
/// Contains the normalized quotes data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotesData {
    /// The instrument mapping key: short code for instrument identifier.
    #[prost(uint32, tag="1")]
    pub mapping: u32,
    /// The time stamp of the quotes (UTC)
    #[prost(message, optional, tag="2")]
    pub time_stamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The quotes array
    #[prost(message, repeated, tag="3")]
    pub quotes: ::prost::alloc::vec::Vec<Quote>,
}
/// Contains the tick trading conditions's data: value, description, regular and auction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// The trading condition value
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
    /// The trading condition description
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// The trading condition comment
    #[prost(string, tag="3")]
    pub comment: ::prost::alloc::string::String,
    /// Is regular market condition
    #[prost(bool, tag="4")]
    pub regular: bool,
    /// Is auction market condition
    #[prost(bool, tag="5")]
    pub auction: bool,
}
/// Contains the intraday sampling intervals.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Sampling {
    /// The unspecifed interval
    Unspecified = 0,
    /// The one minute interval
    OneMinute = 1,
    /// The five minutes interval
    FiveMinute = 2,
    /// The ten minutes interval
    TenMinute = 3,
    /// The fifteen minutes interval
    FifteenMinute = 4,
    /// The thirty minutes interval
    ThirtyMinute = 5,
    /// The one hour interval
    OneHour = 6,
}
/// Represents a set of data for a financial asset
/// It is built from
/// - A provider (a market data provider)
/// - A group (asset group given by the provider)
/// - A stream (which type od data : Open High Low Close, Level 1, Level 1 and Level 2, ...)
/// - A ticker (asset name given by the provider)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Memo {
    /// Provider name
    #[prost(string, tag="1")]
    pub provider: ::prost::alloc::string::String,
    /// Group name, associated with provider
    #[prost(string, tag="2")]
    pub group: ::prost::alloc::string::String,
    /// Stream name, associated with the type of data needed
    #[prost(string, tag="3")]
    pub stream: ::prost::alloc::string::String,
    /// Ticker name, associated with provider and group defines a unique asset
    #[prost(string, tag="4")]
    pub ticker: ::prost::alloc::string::String,
}
/// Contains the market Data update action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Stream {
    /// The unspecfied stream
    Unspecified = 0,
    /// The stream only containing fields updates (see field.proto).
    FieldsOnly = 1,
    /// The stream with market book updates (see mbl_market_book_updates.proto and mbo_market_book_updates.proto) along with fields updates.
    MarketbookAndFields = 2,
}
/// Represents a mapping between an eventSource and a memo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mapping {
    /// An Identifier from the market data request
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<Identifier>,
    /// Memo associated with Identifier
    #[prost(message, optional, tag="2")]
    pub memo: ::core::option::Option<Memo>,
    /// Short name for a memo
    #[prost(uint32, tag="3")]
    pub event_source: u32,
}
/// Represents several Mapping
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mappings {
    /// An array of Mapping
    #[prost(message, repeated, tag="1")]
    pub table: ::prost::alloc::vec::Vec<Mapping>,
}
