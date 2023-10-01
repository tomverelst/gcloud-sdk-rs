/// Metadata for any related URL information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedUrl {
    /// Specific URL associated with the resource.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// Label to describe usage of the URL.
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
}
/// Verifiers (e.g. Kritis implementations) MUST verify signatures
/// with respect to the trust anchors defined in policy (e.g. a Kritis policy).
/// Typically this means that the verifier has been configured with a map from
/// `public_key_id` to public key material (and any required parameters, e.g.
/// signing algorithm).
///
/// In particular, verification implementations MUST NOT treat the signature
/// `public_key_id` as anything more than a key lookup hint. The `public_key_id`
/// DOES NOT validate or authenticate a public key; it only provides a mechanism
/// for quickly selecting a public key ALREADY CONFIGURED on the verifier through
/// a trusted channel. Verification implementations MUST reject signatures in any
/// of the following circumstances:
///    * The `public_key_id` is not recognized by the verifier.
///    * The public key that `public_key_id` refers to does not verify the
///      signature with respect to the payload.
///
/// The `signature` contents SHOULD NOT be "attached" (where the payload is
/// included with the serialized `signature` bytes). Verifiers MUST ignore any
/// "attached" payload and only verify signatures with respect to explicitly
/// provided payload (e.g. a `payload` field on the proto message that holds
/// this Signature, or the canonical serialization of the proto message that
/// holds this signature).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    /// The content of the signature, an opaque bytestring.
    /// The payload that this signature verifies MUST be unambiguously provided
    /// with the Signature during verification. A wrapper message might provide
    /// the payload explicitly. Alternatively, a message might have a canonical
    /// serialization that can always be unambiguously computed to derive the
    /// payload.
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// The identifier for the public key that verifies this signature.
    ///    * The `public_key_id` is required.
    ///    * The `public_key_id` MUST be an RFC3986 conformant URI.
    ///    * When possible, the `public_key_id` SHOULD be an immutable reference,
    ///      such as a cryptographic digest.
    ///
    /// Examples of valid `public_key_id`s:
    ///
    /// OpenPGP V4 public key fingerprint:
    ///    * "openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA"
    /// See <https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr> for more
    /// details on this scheme.
    ///
    /// RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER
    /// serialization):
    ///    * "ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU"
    ///    * "nih:///sha-256;703f68f42aba2c6de30f488a5ea122fef76324679c9bf89791ba95a1271589a5"
    #[prost(string, tag = "2")]
    pub public_key_id: ::prost::alloc::string::String,
}
/// Kind represents the kinds of notes supported.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NoteKind {
    /// Unknown.
    Unspecified = 0,
    /// The note and occurrence represent a package vulnerability.
    Vulnerability = 1,
    /// The note and occurrence assert build provenance.
    Build = 2,
    /// This represents an image basis relationship.
    Image = 3,
    /// This represents a package installed via a package manager.
    Package = 4,
    /// The note and occurrence track deployment events.
    Deployment = 5,
    /// The note and occurrence track the initial discovery status of a resource.
    Discovery = 6,
    /// This represents a logical "role" that can attest to artifacts.
    Attestation = 7,
}
impl NoteKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NoteKind::Unspecified => "NOTE_KIND_UNSPECIFIED",
            NoteKind::Vulnerability => "VULNERABILITY",
            NoteKind::Build => "BUILD",
            NoteKind::Image => "IMAGE",
            NoteKind::Package => "PACKAGE",
            NoteKind::Deployment => "DEPLOYMENT",
            NoteKind::Discovery => "DISCOVERY",
            NoteKind::Attestation => "ATTESTATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOTE_KIND_UNSPECIFIED" => Some(Self::Unspecified),
            "VULNERABILITY" => Some(Self::Vulnerability),
            "BUILD" => Some(Self::Build),
            "IMAGE" => Some(Self::Image),
            "PACKAGE" => Some(Self::Package),
            "DEPLOYMENT" => Some(Self::Deployment),
            "DISCOVERY" => Some(Self::Discovery),
            "ATTESTATION" => Some(Self::Attestation),
            _ => None,
        }
    }
}
/// An instance of an analysis type that has been found on a resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Occurrence {
    /// Output only. The name of the occurrence in the form of
    /// `projects/\[PROJECT_ID\]/occurrences/\[OCCURRENCE_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The resource for which the occurrence applies.
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<Resource>,
    /// Required. Immutable. The analysis note associated with this occurrence, in
    /// the form of `projects/\[PROVIDER_ID\]/notes/\[NOTE_ID\]`. This field can be
    /// used as a filter in list requests.
    #[prost(string, tag = "3")]
    pub note_name: ::prost::alloc::string::String,
    /// Output only. This explicitly denotes which of the occurrence details are
    /// specified. This field can be used as a filter in list requests.
    #[prost(enumeration = "NoteKind", tag = "4")]
    pub kind: i32,
    /// A description of actions that can be taken to remedy the note.
    #[prost(string, tag = "5")]
    pub remediation: ::prost::alloc::string::String,
    /// Output only. The time this occurrence was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this occurrence was last updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Immutable. Describes the details of the note kind found on this
    /// resource.
    #[prost(oneof = "occurrence::Details", tags = "8, 9, 10, 11, 12, 13, 14")]
    pub details: ::core::option::Option<occurrence::Details>,
}
/// Nested message and enum types in `Occurrence`.
pub mod occurrence {
    /// Required. Immutable. Describes the details of the note kind found on this
    /// resource.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Describes a security vulnerability.
        #[prost(message, tag = "8")]
        Vulnerability(super::vulnerability::Details),
        /// Describes a verifiable build.
        #[prost(message, tag = "9")]
        Build(super::build::Details),
        /// Describes how this resource derives from the basis in the associated
        /// note.
        #[prost(message, tag = "10")]
        DerivedImage(super::image::Details),
        /// Describes the installation of a package on the linked resource.
        #[prost(message, tag = "11")]
        Installation(super::package::Details),
        /// Describes the deployment of an artifact on a runtime.
        #[prost(message, tag = "12")]
        Deployment(super::deployment::Details),
        /// Describes when a resource was discovered.
        #[prost(message, tag = "13")]
        Discovered(super::discovery::Details),
        /// Describes an attestation of an artifact.
        #[prost(message, tag = "14")]
        Attestation(super::attestation::Details),
    }
}
/// An entity that can have metadata. For example, a Docker image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The name of the resource. For example, the name of a Docker image -
    /// "Debian".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The unique URI of the resource. For example,
    /// `<https://gcr.io/project/image@sha256:foo`> for a Docker image.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// The hash of the resource content. For example, the Docker digest.
    #[prost(message, optional, tag = "3")]
    pub content_hash: ::core::option::Option<provenance::Hash>,
}
/// A type of analysis that can be done for a resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Note {
    /// Output only. The name of the note in the form of
    /// `projects/\[PROVIDER_ID\]/notes/\[NOTE_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A one sentence description of this note.
    #[prost(string, tag = "2")]
    pub short_description: ::prost::alloc::string::String,
    /// A detailed description of this note.
    #[prost(string, tag = "3")]
    pub long_description: ::prost::alloc::string::String,
    /// Output only. The type of analysis. This field can be used as a filter in
    /// list requests.
    #[prost(enumeration = "NoteKind", tag = "4")]
    pub kind: i32,
    /// URLs associated with this note.
    #[prost(message, repeated, tag = "5")]
    pub related_url: ::prost::alloc::vec::Vec<RelatedUrl>,
    /// Time of expiration for this note. Empty if note does not expire.
    #[prost(message, optional, tag = "6")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this note was created. This field can be used as a
    /// filter in list requests.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this note was last updated. This field can be used as
    /// a filter in list requests.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Other notes related to this note.
    #[prost(string, repeated, tag = "9")]
    pub related_note_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Immutable. The type of analysis this note represents.
    #[prost(oneof = "note::Type", tags = "10, 11, 12, 13, 14, 15, 16")]
    pub r#type: ::core::option::Option<note::Type>,
}
/// Nested message and enum types in `Note`.
pub mod note {
    /// Required. Immutable. The type of analysis this note represents.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// A note describing a package vulnerability.
        #[prost(message, tag = "10")]
        Vulnerability(super::vulnerability::Vulnerability),
        /// A note describing build provenance for a verifiable build.
        #[prost(message, tag = "11")]
        Build(super::build::Build),
        /// A note describing a base image.
        #[prost(message, tag = "12")]
        BaseImage(super::image::Basis),
        /// A note describing a package hosted by various package managers.
        #[prost(message, tag = "13")]
        Package(super::package::Package),
        /// A note describing something that can be deployed.
        #[prost(message, tag = "14")]
        Deployable(super::deployment::Deployable),
        /// A note describing the initial analysis of a resource.
        #[prost(message, tag = "15")]
        Discovery(super::discovery::Discovery),
        /// A note describing an attestation role.
        #[prost(message, tag = "16")]
        AttestationAuthority(super::attestation::Authority),
    }
}
/// Request to get an occurrence.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOccurrenceRequest {
    /// The name of the occurrence in the form of
    /// `projects/\[PROJECT_ID\]/occurrences/\[OCCURRENCE_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to list occurrences.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOccurrencesRequest {
    /// The name of the project to list occurrences for in the form of
    /// `projects/\[PROJECT_ID\]`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter expression.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Number of occurrences to return in the list.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing occurrences.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOccurrencesResponse {
    /// The occurrences requested.
    #[prost(message, repeated, tag = "1")]
    pub occurrences: ::prost::alloc::vec::Vec<Occurrence>,
    /// The next pagination token in the list response. It should be used as
    /// `page_token` for the following request. An empty value means no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to delete a occurrence.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOccurrenceRequest {
    /// The name of the occurrence in the form of
    /// `projects/\[PROJECT_ID\]/occurrences/\[OCCURRENCE_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to create a new occurrence.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOccurrenceRequest {
    /// The name of the project in the form of `projects/\[PROJECT_ID\]`, under which
    /// the occurrence is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The occurrence to create.
    #[prost(message, optional, tag = "2")]
    pub occurrence: ::core::option::Option<Occurrence>,
}
/// Request to update an occurrence.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOccurrenceRequest {
    /// The name of the occurrence in the form of
    /// `projects/\[PROJECT_ID\]/occurrences/\[OCCURRENCE_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The updated occurrence.
    #[prost(message, optional, tag = "2")]
    pub occurrence: ::core::option::Option<Occurrence>,
    /// The fields to update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to get a note.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNoteRequest {
    /// The name of the note in the form of
    /// `projects/\[PROVIDER_ID\]/notes/\[NOTE_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to get the note to which the specified occurrence is attached.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOccurrenceNoteRequest {
    /// The name of the occurrence in the form of
    /// `projects/\[PROJECT_ID\]/occurrences/\[OCCURRENCE_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to list notes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotesRequest {
    /// The name of the project to list notes for in the form of
    /// `projects/\[PROJECT_ID\]`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter expression.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Number of notes to return in the list.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing notes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotesResponse {
    /// The notes requested.
    #[prost(message, repeated, tag = "1")]
    pub notes: ::prost::alloc::vec::Vec<Note>,
    /// The next pagination token in the list response. It should be used as
    /// `page_token` for the following request. An empty value means no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to delete a note.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNoteRequest {
    /// The name of the note in the form of
    /// `projects/\[PROVIDER_ID\]/notes/\[NOTE_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to create a new note.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNoteRequest {
    /// The name of the project in the form of `projects/\[PROJECT_ID\]`, under which
    /// the note is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The ID to use for this note.
    #[prost(string, tag = "2")]
    pub note_id: ::prost::alloc::string::String,
    /// The note to create.
    #[prost(message, optional, tag = "3")]
    pub note: ::core::option::Option<Note>,
}
/// Request to update a note.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNoteRequest {
    /// The name of the note in the form of
    /// `projects/\[PROVIDER_ID\]/notes/\[NOTE_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The updated note.
    #[prost(message, optional, tag = "2")]
    pub note: ::core::option::Option<Note>,
    /// The fields to update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to list occurrences for a note.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNoteOccurrencesRequest {
    /// The name of the note to list occurrences for in the form of
    /// `projects/\[PROVIDER_ID\]/notes/\[NOTE_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The filter expression.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Number of occurrences to return in the list.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing occurrences for a note.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNoteOccurrencesResponse {
    /// The occurrences attached to the specified note.
    #[prost(message, repeated, tag = "1")]
    pub occurrences: ::prost::alloc::vec::Vec<Occurrence>,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to create notes in batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateNotesRequest {
    /// The name of the project in the form of `projects/\[PROJECT_ID\]`, under which
    /// the notes are to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The notes to create.
    #[prost(map = "string, message", tag = "2")]
    pub notes: ::std::collections::HashMap<::prost::alloc::string::String, Note>,
}
/// Response for creating notes in batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateNotesResponse {
    /// The notes that were created.
    #[prost(message, repeated, tag = "1")]
    pub notes: ::prost::alloc::vec::Vec<Note>,
}
/// Request to create occurrences in batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateOccurrencesRequest {
    /// The name of the project in the form of `projects/\[PROJECT_ID\]`, under which
    /// the occurrences are to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The occurrences to create.
    #[prost(message, repeated, tag = "2")]
    pub occurrences: ::prost::alloc::vec::Vec<Occurrence>,
}
/// Response for creating occurrences in batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateOccurrencesResponse {
    /// The occurrences that were created.
    #[prost(message, repeated, tag = "1")]
    pub occurrences: ::prost::alloc::vec::Vec<Occurrence>,
}
/// Request to get a vulnerability summary for some set of occurrences.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVulnerabilityOccurrencesSummaryRequest {
    /// The name of the project to get a vulnerability summary for in the form of
    /// `projects/\[PROJECT_ID\]`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter expression.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
}
/// A summary of how many vulnerability occurrences there are per resource and
/// severity type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerabilityOccurrencesSummary {
    /// A listing by resource of the number of fixable and total vulnerabilities.
    #[prost(message, repeated, tag = "1")]
    pub counts: ::prost::alloc::vec::Vec<
        vulnerability_occurrences_summary::FixableTotalByDigest,
    >,
}
/// Nested message and enum types in `VulnerabilityOccurrencesSummary`.
pub mod vulnerability_occurrences_summary {
    /// Per resource and severity counts of fixable and total vulnerabilities.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FixableTotalByDigest {
        /// The affected resource.
        #[prost(message, optional, tag = "1")]
        pub resource: ::core::option::Option<super::Resource>,
        /// The severity for this count. SEVERITY_UNSPECIFIED indicates total across
        /// all severities.
        #[prost(enumeration = "super::vulnerability::Severity", tag = "2")]
        pub severity: i32,
        /// The number of fixable vulnerabilities associated with this resource.
        #[prost(int64, tag = "3")]
        pub fixable_count: i64,
        /// The total number of vulnerabilities associated with this resource.
        #[prost(int64, tag = "4")]
        pub total_count: i64,
    }
}
/// Generated client implementations.
pub mod grafeas_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// [Grafeas](grafeas.io) API.
    ///
    /// Retrieves analysis results of Cloud components such as Docker container
    /// images.
    ///
    /// Analysis results are stored as a series of occurrences. An `Occurrence`
    /// contains information about a specific analysis instance on a resource. An
    /// occurrence refers to a `Note`. A note contains details describing the
    /// analysis and is generally stored in a separate project, called a `Provider`.
    /// Multiple occurrences can refer to the same note.
    ///
    /// For example, an SSL vulnerability could affect multiple images. In this case,
    /// there would be one note for the vulnerability and an occurrence for each
    /// image with the vulnerability referring to that note.
    #[derive(Debug, Clone)]
    pub struct GrafeasV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GrafeasV1Beta1Client<tonic::transport::Channel> {
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
    impl<T> GrafeasV1Beta1Client<T>
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
        ) -> GrafeasV1Beta1Client<InterceptedService<T, F>>
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
            GrafeasV1Beta1Client::new(InterceptedService::new(inner, interceptor))
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
        /// Gets the specified occurrence.
        pub async fn get_occurrence(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOccurrenceRequest>,
        ) -> std::result::Result<tonic::Response<super::Occurrence>, tonic::Status> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/GetOccurrence",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "GetOccurrence"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists occurrences for the specified project.
        pub async fn list_occurrences(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOccurrencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOccurrencesResponse>,
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
                "/grafeas.v1beta1.GrafeasV1Beta1/ListOccurrences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "ListOccurrences"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified occurrence. For example, use this method to delete an
        /// occurrence when the occurrence is no longer applicable for the given
        /// resource.
        pub async fn delete_occurrence(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOccurrenceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/DeleteOccurrence",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "DeleteOccurrence"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new occurrence.
        pub async fn create_occurrence(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOccurrenceRequest>,
        ) -> std::result::Result<tonic::Response<super::Occurrence>, tonic::Status> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/CreateOccurrence",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "CreateOccurrence"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates new occurrences in batch.
        pub async fn batch_create_occurrences(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateOccurrencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateOccurrencesResponse>,
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
                "/grafeas.v1beta1.GrafeasV1Beta1/BatchCreateOccurrences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "grafeas.v1beta1.GrafeasV1Beta1",
                        "BatchCreateOccurrences",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified occurrence.
        pub async fn update_occurrence(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOccurrenceRequest>,
        ) -> std::result::Result<tonic::Response<super::Occurrence>, tonic::Status> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/UpdateOccurrence",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "UpdateOccurrence"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the note attached to the specified occurrence. Consumer projects can
        /// use this method to get a note that belongs to a provider project.
        pub async fn get_occurrence_note(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOccurrenceNoteRequest>,
        ) -> std::result::Result<tonic::Response<super::Note>, tonic::Status> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/GetOccurrenceNote",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "grafeas.v1beta1.GrafeasV1Beta1",
                        "GetOccurrenceNote",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the specified note.
        pub async fn get_note(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNoteRequest>,
        ) -> std::result::Result<tonic::Response<super::Note>, tonic::Status> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/GetNote",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "GetNote"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists notes for the specified project.
        pub async fn list_notes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNotesResponse>,
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
                "/grafeas.v1beta1.GrafeasV1Beta1/ListNotes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "ListNotes"));
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified note.
        pub async fn delete_note(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNoteRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/DeleteNote",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "DeleteNote"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new note.
        pub async fn create_note(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNoteRequest>,
        ) -> std::result::Result<tonic::Response<super::Note>, tonic::Status> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/CreateNote",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "CreateNote"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates new notes in batch.
        pub async fn batch_create_notes(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateNotesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateNotesResponse>,
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
                "/grafeas.v1beta1.GrafeasV1Beta1/BatchCreateNotes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "BatchCreateNotes"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified note.
        pub async fn update_note(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNoteRequest>,
        ) -> std::result::Result<tonic::Response<super::Note>, tonic::Status> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/UpdateNote",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("grafeas.v1beta1.GrafeasV1Beta1", "UpdateNote"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists occurrences referencing the specified note. Provider projects can use
        /// this method to get all occurrences across consumer projects referencing the
        /// specified note.
        pub async fn list_note_occurrences(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNoteOccurrencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNoteOccurrencesResponse>,
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
                "/grafeas.v1beta1.GrafeasV1Beta1/ListNoteOccurrences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "grafeas.v1beta1.GrafeasV1Beta1",
                        "ListNoteOccurrences",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a summary of the number and severity of occurrences.
        pub async fn get_vulnerability_occurrences_summary(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetVulnerabilityOccurrencesSummaryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::VulnerabilityOccurrencesSummary>,
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
                "/grafeas.v1beta1.GrafeasV1Beta1/GetVulnerabilityOccurrencesSummary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "grafeas.v1beta1.GrafeasV1Beta1",
                        "GetVulnerabilityOccurrencesSummary",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
