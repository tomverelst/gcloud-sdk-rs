/// Details on ReportBillableEvent request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportBillableEventLog {
    /// The id of the billable event.
    /// Subject to the following restrictions:
    ///
    /// 1. IDs must be valid Unicode strings.
    /// 2. IDs are limited to a maximum length of 64 characters.
    /// 3. IDs must be normalized according to Unicode Normalization Form C
    /// (<http://www.unicode.org/reports/tr15/> ).
    /// 4. IDs must not contain any of the following ASCII characters: '/', ':',
    /// '?', ',', or '#'.
    #[prost(string, tag = "1")]
    pub billable_event_id: ::prost::alloc::string::String,
    /// Two-letter region code of the country or region where the event takes
    /// place. A list of valid region codes can be found here:
    /// <https://developers.google.com/maps/coverage>
    #[prost(string, tag = "2")]
    pub region_code: ::prost::alloc::string::String,
    /// The identifiers that are directly related to the event being reported.
    /// The customer defines the meaning of these IDs.
    #[prost(string, repeated, tag = "3")]
    pub related_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
