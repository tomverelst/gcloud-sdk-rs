/*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::compute_v1::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketMirroringFilter {
    /// Protocols that apply as filter on mirrored traffic. If no protocols are specified, all traffic that matches the specified CIDR ranges is mirrored. If neither cidrRanges nor IPProtocols is specified, all IPv4 traffic is mirrored.
    #[serde(rename = "IPProtocols", skip_serializing_if = "Option::is_none")]
    pub ip_protocols: Option<Vec<String>>,
    /// One or more IPv4 or IPv6 CIDR ranges that apply as filter on the source (ingress) or destination (egress) IP in the IP header. If no ranges are specified, all IPv4 traffic that matches the specified IPProtocols is mirrored. If neither cidrRanges nor IPProtocols is specified, all IPv4 traffic is mirrored. To mirror all IPv4 and IPv6 traffic, use \"0.0.0.0/0,::/0\". Note: Support for IPv6 traffic is in preview.
    #[serde(rename = "cidrRanges", skip_serializing_if = "Option::is_none")]
    pub cidr_ranges: Option<Vec<String>>,
    /// Direction of traffic to mirror, either INGRESS, EGRESS, or BOTH. The default is BOTH.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
}

impl PacketMirroringFilter {
    pub fn new() -> PacketMirroringFilter {
        PacketMirroringFilter {
            ip_protocols: None,
            cidr_ranges: None,
            direction: None,
        }
    }
}
/// Direction of traffic to mirror, either INGRESS, EGRESS, or BOTH. The default is BOTH.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "EGRESS")]
    Egress,
    #[serde(rename = "INGRESS")]
    Ingress,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Both
    }
}
