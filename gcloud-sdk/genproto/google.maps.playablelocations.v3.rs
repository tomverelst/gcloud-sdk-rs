/// A report submitted by a player about a playable location that is considered
/// inappropriate for use in the game.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerReport {
    /// Required. The name of the playable location.
    #[prost(string, tag = "1")]
    pub location_name: ::prost::alloc::string::String,
    /// Required. One or more reasons why this playable location is considered bad.
    #[prost(
        enumeration = "player_report::BadLocationReason",
        repeated,
        packed = "false",
        tag = "2"
    )]
    pub reasons: ::prost::alloc::vec::Vec<i32>,
    /// Required. A free-form description detailing why the playable location is
    /// considered bad.
    #[prost(string, tag = "3")]
    pub reason_details: ::prost::alloc::string::String,
    /// Language code (in BCP-47 format) indicating the language of the freeform
    /// description provided in `reason_details`. Examples are "en", "en-US" or
    /// "ja-Latn". For more information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PlayerReport`.
pub mod player_report {
    /// The reason why the playable location is considered bad.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BadLocationReason {
        /// Unspecified reason. Do not use.
        Unspecified = 0,
        /// The reason isn't one of the reasons in this enumeration.
        Other = 1,
        /// The playable location isn't accessible to pedestrians. For example, if
        /// it's in the middle of a highway.
        NotPedestrianAccessible = 2,
        /// The playable location isn't open to the public. For example, a private
        /// office building.
        NotOpenToPublic = 4,
        /// The playable location is permanently closed. For example, when a business
        /// has been shut down.
        PermanentlyClosed = 5,
        /// The playable location is temporarily inaccessible. For example, when a
        /// business has closed for renovations.
        TemporarilyInaccessible = 6,
    }
    impl BadLocationReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BadLocationReason::Unspecified => "BAD_LOCATION_REASON_UNSPECIFIED",
                BadLocationReason::Other => "OTHER",
                BadLocationReason::NotPedestrianAccessible => "NOT_PEDESTRIAN_ACCESSIBLE",
                BadLocationReason::NotOpenToPublic => "NOT_OPEN_TO_PUBLIC",
                BadLocationReason::PermanentlyClosed => "PERMANENTLY_CLOSED",
                BadLocationReason::TemporarilyInaccessible => "TEMPORARILY_INACCESSIBLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BAD_LOCATION_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "OTHER" => Some(Self::Other),
                "NOT_PEDESTRIAN_ACCESSIBLE" => Some(Self::NotPedestrianAccessible),
                "NOT_OPEN_TO_PUBLIC" => Some(Self::NotOpenToPublic),
                "PERMANENTLY_CLOSED" => Some(Self::PermanentlyClosed),
                "TEMPORARILY_INACCESSIBLE" => Some(Self::TemporarilyInaccessible),
                _ => None,
            }
        }
    }
}
/// Encapsulates impression event details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Impression {
    /// Required. The name of the playable location.
    #[prost(string, tag = "1")]
    pub location_name: ::prost::alloc::string::String,
    /// Required. The type of impression event.
    #[prost(enumeration = "impression::ImpressionType", tag = "2")]
    pub impression_type: i32,
    /// An arbitrary, developer-defined type identifier for each type of game
    /// object used in your game.
    ///
    /// Since players interact with differ types of game objects in different ways,
    /// this field allows you to segregate impression data by type for analysis.
    ///
    /// You should assign a unique `game_object_type` ID to represent a distinct
    /// type of game object in your game.
    ///
    /// For example, 1=monster location, 2=powerup location.
    #[prost(int32, tag = "4")]
    pub game_object_type: i32,
}
/// Nested message and enum types in `Impression`.
pub mod impression {
    /// The type of impression event.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ImpressionType {
        /// Unspecified type. Do not use.
        Unspecified = 0,
        /// The playable location was presented to a player.
        Presented = 1,
        /// A player interacted with the playable location.
        Interacted = 2,
    }
    impl ImpressionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ImpressionType::Unspecified => "IMPRESSION_TYPE_UNSPECIFIED",
                ImpressionType::Presented => "PRESENTED",
                ImpressionType::Interacted => "INTERACTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IMPRESSION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PRESENTED" => Some(Self::Presented),
                "INTERACTED" => Some(Self::Interacted),
                _ => None,
            }
        }
    }
}
///
/// Life of a query:
///
/// - When a game starts in a new location, your game server issues a
/// [SamplePlayableLocations][google.maps.playablelocations.v3.PlayableLocations.SamplePlayableLocations]
/// request. The request specifies the S2 cell, and contains one or more
/// "criteria" for filtering:
///
/// - Criterion 0: i locations for long-lived bases, or level 0 monsters, or...
/// - Criterion 1: j locations for short-lived bases, or level 1 monsters, ...
/// - Criterion 2: k locations for random objects.
/// - etc (up to 5 criterion may be specified).
///
/// `PlayableLocationList` will then contain mutually
/// exclusive lists of `PlayableLocation` objects that satisfy each of
/// the criteria. Think of it as a collection of real-world locations that you
/// can then associate with your game state.
///
/// Note: These points are impermanent in nature. E.g, parks can close, and
/// places can be removed.
///
/// The response specifies how long you can expect the playable locations to
/// last. Once they expire, you should query the `samplePlayableLocations` API
/// again to get a fresh view of the real world.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamplePlayableLocationsRequest {
    /// Required. Specifies the area to search within for playable locations.
    #[prost(message, optional, tag = "1")]
    pub area_filter: ::core::option::Option<sample::AreaFilter>,
    /// Required. Specifies one or more (up to 5) criteria for filtering the
    /// returned playable locations.
    #[prost(message, repeated, tag = "2")]
    pub criteria: ::prost::alloc::vec::Vec<sample::Criterion>,
}
///
/// Response for the
/// [SamplePlayableLocations][google.maps.playablelocations.v3.PlayableLocations.SamplePlayableLocations]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamplePlayableLocationsResponse {
    /// Each PlayableLocation object corresponds to a game_object_type specified
    /// in the request.
    #[prost(map = "int32, message", tag = "1")]
    pub locations_per_game_object_type: ::std::collections::HashMap<
        i32,
        sample::PlayableLocationList,
    >,
    /// Required. Specifies the "time-to-live" for the set of playable locations.
    /// You can use this value to determine how long to cache the set of playable
    /// locations. After this length of time, your back-end game server should
    /// issue a new
    /// [SamplePlayableLocations][google.maps.playablelocations.v3.PlayableLocations.SamplePlayableLocations]
    /// request to get a fresh set of playable locations (because for example, they
    /// might have been removed, a park might have closed for the day, a
    /// business might have closed permanently).
    #[prost(message, optional, tag = "9")]
    pub ttl: ::core::option::Option<::prost_types::Duration>,
}
/// A request for logging your player's bad location reports.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogPlayerReportsRequest {
    /// Required. Player reports. The maximum number of player reports that you can
    /// log at once is 50.
    #[prost(message, repeated, tag = "1")]
    pub player_reports: ::prost::alloc::vec::Vec<PlayerReport>,
    /// Required. A string that uniquely identifies the log player reports request.
    /// This allows you to detect duplicate requests. We recommend that you use
    /// UUIDs for this value. The value must not exceed 50 characters.
    ///
    /// You should reuse the `request_id` only when retrying a request in the case
    /// of a failure. In that case, the request must be identical to the one that
    /// failed.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. Information about the client device (for example, device model
    /// and operating system).
    #[prost(message, optional, tag = "3")]
    pub client_info: ::core::option::Option<super::super::unity::ClientInfo>,
}
/// A response for the
/// [LogPlayerReports][google.maps.playablelocations.v3.PlayableLocations.LogPlayerReports]
/// method.
///
/// This method returns no data upon success.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogPlayerReportsResponse {}
/// A request for logging impressions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogImpressionsRequest {
    /// Required. Impression event details. The maximum number of impression
    /// reports that you can log at once is 50.
    #[prost(message, repeated, tag = "1")]
    pub impressions: ::prost::alloc::vec::Vec<Impression>,
    /// Required. A string that uniquely identifies the log impressions request.
    /// This allows you to detect duplicate requests. We recommend that you use
    /// UUIDs for this value. The value must not exceed 50 characters.
    ///
    /// You should reuse the `request_id` only when retrying a request in case of
    /// failure. In this case, the request must be identical to the one that
    /// failed.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. Information about the client device. For example, device model
    /// and operating system.
    #[prost(message, optional, tag = "3")]
    pub client_info: ::core::option::Option<super::super::unity::ClientInfo>,
}
/// A response for the
/// [LogImpressions][google.maps.playablelocations.v3.PlayableLocations.LogImpressions]
/// method. This method returns no data upon success.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogImpressionsResponse {}
/// Generated client implementations.
pub mod playable_locations_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Playable Locations API for v3.
    #[derive(Debug, Clone)]
    pub struct PlayableLocationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayableLocationsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlayableLocationsClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PlayableLocationsClient<InterceptedService<T, F>>
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
            PlayableLocationsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Returns a set of playable locations that lie within a specified area,
        /// that satisfy optional filter criteria.
        ///
        /// Note: Identical `SamplePlayableLocations` requests can return different
        /// results as the state of the world changes over time.
        pub async fn sample_playable_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::SamplePlayableLocationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SamplePlayableLocationsResponse>,
            tonic::Status,
        > {
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
                "/google.maps.playablelocations.v3.PlayableLocations/SamplePlayableLocations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.playablelocations.v3.PlayableLocations",
                        "SamplePlayableLocations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Logs bad playable location reports submitted by players.
        ///
        /// Reports are not partially saved; either all reports are saved and this
        /// request succeeds, or no reports are saved, and this request fails.
        pub async fn log_player_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::LogPlayerReportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LogPlayerReportsResponse>,
            tonic::Status,
        > {
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
                "/google.maps.playablelocations.v3.PlayableLocations/LogPlayerReports",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.playablelocations.v3.PlayableLocations",
                        "LogPlayerReports",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Logs new events when playable locations are displayed, and when they are
        /// interacted with.
        ///
        /// Impressions are not partially saved; either all impressions are saved and
        /// this request succeeds, or no impressions are saved, and this request fails.
        pub async fn log_impressions(
            &mut self,
            request: impl tonic::IntoRequest<super::LogImpressionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LogImpressionsResponse>,
            tonic::Status,
        > {
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
                "/google.maps.playablelocations.v3.PlayableLocations/LogImpressions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.playablelocations.v3.PlayableLocations",
                        "LogImpressions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
