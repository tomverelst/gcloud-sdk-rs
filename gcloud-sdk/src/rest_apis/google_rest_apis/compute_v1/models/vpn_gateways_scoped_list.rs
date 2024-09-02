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
pub struct VpnGatewaysScopedList {
    /// [Output Only] A list of VPN gateways contained in this scope.
    #[serde(rename = "vpnGateways", skip_serializing_if = "Option::is_none")]
    pub vpn_gateways: Option<Vec<models::VpnGateway>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::AddressesScopedListWarning>>,
}

impl VpnGatewaysScopedList {
    pub fn new() -> VpnGatewaysScopedList {
        VpnGatewaysScopedList {
            vpn_gateways: None,
            warning: None,
        }
    }
}
