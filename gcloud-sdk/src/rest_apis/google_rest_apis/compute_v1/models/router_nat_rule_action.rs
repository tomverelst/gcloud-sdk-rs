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
pub struct RouterNatRuleAction {
    /// A list of URLs of the IP resources used for this NAT rule. These IP addresses must be valid static external IP addresses assigned to the project. This field is used for public NAT.
    #[serde(rename = "sourceNatActiveIps", skip_serializing_if = "Option::is_none")]
    pub source_nat_active_ips: Option<Vec<String>>,
    /// A list of URLs of the subnetworks used as source ranges for this NAT Rule. These subnetworks must have purpose set to PRIVATE_NAT. This field is used for private NAT.
    #[serde(
        rename = "sourceNatActiveRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_nat_active_ranges: Option<Vec<String>>,
    /// A list of URLs of the IP resources to be drained. These IPs must be valid static external IPs that have been assigned to the NAT. These IPs should be used for updating/patching a NAT rule only. This field is used for public NAT.
    #[serde(rename = "sourceNatDrainIps", skip_serializing_if = "Option::is_none")]
    pub source_nat_drain_ips: Option<Vec<String>>,
    /// A list of URLs of subnetworks representing source ranges to be drained. This is only supported on patch/update, and these subnetworks must have previously been used as active ranges in this NAT Rule. This field is used for private NAT.
    #[serde(
        rename = "sourceNatDrainRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_nat_drain_ranges: Option<Vec<String>>,
}

impl RouterNatRuleAction {
    pub fn new() -> RouterNatRuleAction {
        RouterNatRuleAction {
            source_nat_active_ips: None,
            source_nat_active_ranges: None,
            source_nat_drain_ips: None,
            source_nat_drain_ranges: None,
        }
    }
}
