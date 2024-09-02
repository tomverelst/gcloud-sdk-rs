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

/// VmEndpointNatMappingsInterfaceNatMappingsNatRuleMappings : Contains information of NAT Mappings provided by a NAT Rule.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmEndpointNatMappingsInterfaceNatMappingsNatRuleMappings {
    /// List of all drain IP:port-range mappings assigned to this interface by this rule. These ranges are inclusive, that is, both the first and the last ports can be used for NAT. Example: [\"2.2.2.2:12345-12355\", \"1.1.1.1:2234-2234\"].
    #[serde(
        rename = "drainNatIpPortRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub drain_nat_ip_port_ranges: Option<Vec<String>>,
    /// A list of all IP:port-range mappings assigned to this interface by this rule. These ranges are inclusive, that is, both the first and the last ports can be used for NAT. Example: [\"2.2.2.2:12345-12355\", \"1.1.1.1:2234-2234\"].
    #[serde(rename = "natIpPortRanges", skip_serializing_if = "Option::is_none")]
    pub nat_ip_port_ranges: Option<Vec<String>>,
    /// Total number of drain ports across all NAT IPs allocated to this interface by this rule. It equals the aggregated port number in the field drain_nat_ip_port_ranges.
    #[serde(
        rename = "numTotalDrainNatPorts",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_total_drain_nat_ports: Option<i32>,
    /// Total number of ports across all NAT IPs allocated to this interface by this rule. It equals the aggregated port number in the field nat_ip_port_ranges.
    #[serde(rename = "numTotalNatPorts", skip_serializing_if = "Option::is_none")]
    pub num_total_nat_ports: Option<i32>,
    /// Rule number of the NAT Rule.
    #[serde(rename = "ruleNumber", skip_serializing_if = "Option::is_none")]
    pub rule_number: Option<i32>,
}

impl VmEndpointNatMappingsInterfaceNatMappingsNatRuleMappings {
    /// Contains information of NAT Mappings provided by a NAT Rule.
    pub fn new() -> VmEndpointNatMappingsInterfaceNatMappingsNatRuleMappings {
        VmEndpointNatMappingsInterfaceNatMappingsNatRuleMappings {
            drain_nat_ip_port_ranges: None,
            nat_ip_port_ranges: None,
            num_total_drain_nat_ports: None,
            num_total_nat_ports: None,
            rule_number: None,
        }
    }
}
