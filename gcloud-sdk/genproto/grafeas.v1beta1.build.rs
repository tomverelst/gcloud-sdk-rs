/// Note holding the version of the provider's builder and the signature of the
/// provenance message in the build details occurrence.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Build {
    /// Required. Immutable. Version of the builder which produced this build.
    #[prost(string, tag = "1")]
    pub builder_version: ::prost::alloc::string::String,
    /// Signature of the build in occurrences pointing to this build note
    /// containing build details.
    #[prost(message, optional, tag = "2")]
    pub signature: ::core::option::Option<BuildSignature>,
}
/// Message encapsulating the signature of the verified build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildSignature {
    /// Public key of the builder which can be used to verify that the related
    /// findings are valid and unchanged. If `key_type` is empty, this defaults
    /// to PEM encoded public keys.
    ///
    /// This field may be empty if `key_id` references an external key.
    ///
    /// For Cloud Build based signatures, this is a PEM encoded public
    /// key. To verify the Cloud Build signature, place the contents of
    /// this field into a file (public.pem). The signature field is base64-decoded
    /// into its binary representation in signature.bin, and the provenance bytes
    /// from `BuildDetails` are base64-decoded into a binary representation in
    /// signed.bin. OpenSSL can then verify the signature:
    /// `openssl sha256 -verify public.pem -signature signature.bin signed.bin`
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    /// Required. Signature of the related `BuildProvenance`. In JSON, this is
    /// base-64 encoded.
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// An ID for the key used to sign. This could be either an ID for the key
    /// stored in `public_key` (such as the ID or fingerprint for a PGP key, or the
    /// CN for a cert), or a reference to an external key (such as a reference to a
    /// key in Cloud Key Management Service).
    #[prost(string, tag = "3")]
    pub key_id: ::prost::alloc::string::String,
    /// The type of the key, either stored in `public_key` or referenced in
    /// `key_id`.
    #[prost(enumeration = "build_signature::KeyType", tag = "4")]
    pub key_type: i32,
}
/// Nested message and enum types in `BuildSignature`.
pub mod build_signature {
    /// Public key formats.
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
    pub enum KeyType {
        /// `KeyType` is not set.
        Unspecified = 0,
        /// `PGP ASCII Armored` public key.
        PgpAsciiArmored = 1,
        /// `PKIX PEM` public key.
        PkixPem = 2,
    }
    impl KeyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeyType::Unspecified => "KEY_TYPE_UNSPECIFIED",
                KeyType::PgpAsciiArmored => "PGP_ASCII_ARMORED",
                KeyType::PkixPem => "PKIX_PEM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KEY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PGP_ASCII_ARMORED" => Some(Self::PgpAsciiArmored),
                "PKIX_PEM" => Some(Self::PkixPem),
                _ => None,
            }
        }
    }
}
/// Details of a build occurrence.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Details {
    /// Required. The actual provenance for the build.
    #[prost(message, optional, tag = "1")]
    pub provenance: ::core::option::Option<super::provenance::BuildProvenance>,
    /// Serialized JSON representation of the provenance, used in generating the
    /// build signature in the corresponding build note. After verifying the
    /// signature, `provenance_bytes` can be unmarshalled and compared to the
    /// provenance to confirm that it is unchanged. A base64-encoded string
    /// representation of the provenance bytes is used for the signature in order
    /// to interoperate with openssl which expects this format for signature
    /// verification.
    ///
    /// The serialized form is captured both to avoid ambiguity in how the
    /// provenance is marshalled to json as well to prevent incompatibilities with
    /// future changes.
    #[prost(string, tag = "2")]
    pub provenance_bytes: ::prost::alloc::string::String,
}
