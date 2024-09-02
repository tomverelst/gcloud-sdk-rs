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

/// VpnGatewayStatusHighAvailabilityRequirementState : Describes the high availability requirement state for the VPN connection between this Cloud VPN gateway and a peer gateway.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VpnGatewayStatusHighAvailabilityRequirementState {
    /// Indicates the high availability requirement state for the VPN connection. Valid values are CONNECTION_REDUNDANCY_MET, CONNECTION_REDUNDANCY_NOT_MET.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Indicates the reason why the VPN connection does not meet the high availability redundancy criteria/requirement. Valid values is INCOMPLETE_TUNNELS_COVERAGE.
    #[serde(rename = "unsatisfiedReason", skip_serializing_if = "Option::is_none")]
    pub unsatisfied_reason: Option<UnsatisfiedReason>,
}

impl VpnGatewayStatusHighAvailabilityRequirementState {
    /// Describes the high availability requirement state for the VPN connection between this Cloud VPN gateway and a peer gateway.
    pub fn new() -> VpnGatewayStatusHighAvailabilityRequirementState {
        VpnGatewayStatusHighAvailabilityRequirementState {
            state: None,
            unsatisfied_reason: None,
        }
    }
}
/// Indicates the high availability requirement state for the VPN connection. Valid values are CONNECTION_REDUNDANCY_MET, CONNECTION_REDUNDANCY_NOT_MET.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "CONNECTION_REDUNDANCY_MET")]
    Met,
    #[serde(rename = "CONNECTION_REDUNDANCY_NOT_MET")]
    NotMet,
}

impl Default for State {
    fn default() -> State {
        Self::Met
    }
}
/// Indicates the reason why the VPN connection does not meet the high availability redundancy criteria/requirement. Valid values is INCOMPLETE_TUNNELS_COVERAGE.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnsatisfiedReason {
    #[serde(rename = "INCOMPLETE_TUNNELS_COVERAGE")]
    IncompleteTunnelsCoverage,
}

impl Default for UnsatisfiedReason {
    fn default() -> UnsatisfiedReason {
        Self::IncompleteTunnelsCoverage
    }
}
