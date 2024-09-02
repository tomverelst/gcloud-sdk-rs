/*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::dns_v1::models;
use serde::{Deserialize, Serialize};

/// DnsKey : A DNSSEC key pair.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsKey {
    /// String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time.
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<Algorithm>,
    /// The time that this resource was created in the control plane. This is in RFC3339 text format. Output only.
    #[serde(rename = "creationTime", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the resource's function.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Cryptographic hashes of the DNSKEY resource record associated with this DnsKey. These digests are needed to construct a DS record that points at this DNS key. Output only.
    #[serde(rename = "digests", skip_serializing_if = "Option::is_none")]
    pub digests: Option<Vec<models::DnsKeyDigest>>,
    /// Unique identifier for the resource; defined by the server (output only).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Active keys are used to sign subsequent changes to the ManagedZone. Inactive keys are still present as DNSKEY Resource Records for the use of resolvers validating existing signatures.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Length of the key in bits. Specified at creation time, and then immutable.
    #[serde(rename = "keyLength", skip_serializing_if = "Option::is_none")]
    pub key_length: Option<i32>,
    /// The key tag is a non-cryptographic hash of the a DNSKEY resource record associated with this DnsKey. The key tag can be used to identify a DNSKEY more quickly (but it is not a unique identifier). In particular, the key tag is used in a parent zone's DS record to point at the DNSKEY in this child ManagedZone. The key tag is a number in the range [0, 65535] and the algorithm to calculate it is specified in RFC4034 Appendix B. Output only.
    #[serde(rename = "keyTag", skip_serializing_if = "Option::is_none")]
    pub key_tag: Option<i32>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Base64 encoded public half of this key. Output only.
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// One of \"KEY_SIGNING\" or \"ZONE_SIGNING\". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared, and this key is used to sign only resource record sets of other types. Immutable after creation time.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl DnsKey {
    /// A DNSSEC key pair.
    pub fn new() -> DnsKey {
        DnsKey {
            algorithm: None,
            creation_time: None,
            description: None,
            digests: None,
            id: None,
            is_active: None,
            key_length: None,
            key_tag: None,
            kind: None,
            public_key: None,
            r#type: None,
        }
    }
}
/// String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Algorithm {
    #[serde(rename = "rsasha1")]
    Rsasha1,
    #[serde(rename = "rsasha256")]
    Rsasha256,
    #[serde(rename = "rsasha512")]
    Rsasha512,
    #[serde(rename = "ecdsap256sha256")]
    Ecdsap256sha256,
    #[serde(rename = "ecdsap384sha384")]
    Ecdsap384sha384,
}

impl Default for Algorithm {
    fn default() -> Algorithm {
        Self::Rsasha1
    }
}
/// One of \"KEY_SIGNING\" or \"ZONE_SIGNING\". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared, and this key is used to sign only resource record sets of other types. Immutable after creation time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "keySigning")]
    KeySigning,
    #[serde(rename = "zoneSigning")]
    ZoneSigning,
}

impl Default for Type {
    fn default() -> Type {
        Self::KeySigning
    }
}
