use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityPolicyUserDefinedField {
    /// The base relative to which 'offset' is measured. Possible values are: - IPV4: Points to the beginning of the IPv4 header. - IPV6: Points to the beginning of the IPv6 header. - TCP: Points to the beginning of the TCP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments. - UDP: Points to the beginning of the UDP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments. required
    #[serde(rename = "base", skip_serializing_if = "Option::is_none")]
    pub base: Option<Base>,
    /// If specified, apply this mask (bitwise AND) to the field to ignore bits before matching. Encoded as a hexadecimal number (starting with \"0x\"). The last byte of the field (in network byte order) corresponds to the least significant byte of the mask.
    #[serde(rename = "mask", skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    /// The name of this field. Must be unique within the policy.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Offset of the first byte of the field (in network byte order) relative to 'base'.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Size of the field in bytes. Valid values: 1-4.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl SecurityPolicyUserDefinedField {
    pub fn new() -> SecurityPolicyUserDefinedField {
        SecurityPolicyUserDefinedField {
            base: None,
            mask: None,
            name: None,
            offset: None,
            size: None,
        }
    }
}

/// The base relative to which 'offset' is measured. Possible values are: - IPV4: Points to the beginning of the IPv4 header. - IPV6: Points to the beginning of the IPv6 header. - TCP: Points to the beginning of the TCP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments. - UDP: Points to the beginning of the UDP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments. required
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Base {
    #[serde(rename = "IPV4")]
    Ipv4,
    #[serde(rename = "IPV6")]
    Ipv6,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

impl Default for Base {
    fn default() -> Base {
        Self::Ipv4
    }
}
