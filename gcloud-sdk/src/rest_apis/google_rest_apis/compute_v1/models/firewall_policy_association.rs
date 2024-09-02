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
pub struct FirewallPolicyAssociation {
    /// The target that the firewall policy is attached to.
    #[serde(rename = "attachmentTarget", skip_serializing_if = "Option::is_none")]
    pub attachment_target: Option<String>,
    /// [Output Only] Deprecated, please use short name instead. The display name of the firewall policy of the association.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// [Output Only] The firewall policy ID of the association.
    #[serde(rename = "firewallPolicyId", skip_serializing_if = "Option::is_none")]
    pub firewall_policy_id: Option<String>,
    /// The name for an association.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] The short name of the firewall policy of the association.
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
}

impl FirewallPolicyAssociation {
    pub fn new() -> FirewallPolicyAssociation {
        FirewallPolicyAssociation {
            attachment_target: None,
            display_name: None,
            firewall_policy_id: None,
            name: None,
            short_name: None,
        }
    }
}
