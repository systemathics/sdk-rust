/// Contains the date.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Date {
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without a year
    #[prost(int32, tag="1")]
    pub year: i32,
    /// Month of year. Must be from 1 to 12, or 0 if specifying a year without a month and day
    #[prost(int32, tag="2")]
    pub month: i32,
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0 if specifying a year by itself or a year and month where the day is not significant.
    #[prost(int32, tag="3")]
    pub day: i32,
}
/// Contains the day of week values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DayOfWeek {
    /// The unspecified day-of-week
    Unspecified = 0,
    /// The day-of-week of Monday
    Monday = 1,
    /// The day-of-week of Tuesday
    Tuesday = 2,
    /// The day-of-week of Wednesday
    Wednesday = 3,
    /// The day-of-week of Thursday
    Thursday = 4,
    /// The day-of-week of Friday
    Friday = 5,
    /// The day-of-week of Saturday
    Saturday = 6,
    /// The day-of-week of Sunday
    Sunday = 7,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    /// to allow the value "24:00:00" for scenarios like business closing time.
    #[prost(int32, tag="1")]
    pub hours: i32,
    /// Minutes of hour of day. Must be from 0 to 59.
    #[prost(int32, tag="2")]
    pub minutes: i32,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    /// allow the value 60 if it allows leap-seconds.
    #[prost(int32, tag="3")]
    pub seconds: i32,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    #[prost(int32, tag="4")]
    pub nanos: i32,
}
