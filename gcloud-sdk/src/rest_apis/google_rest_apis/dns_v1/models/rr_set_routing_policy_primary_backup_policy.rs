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

/// RrSetRoutingPolicyPrimaryBackupPolicy : Configures a RRSetRoutingPolicy such that all queries are responded with the primary_targets if they are healthy. And if all of them are unhealthy, then we fallback to a geo localized policy.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RrSetRoutingPolicyPrimaryBackupPolicy {
    #[serde(rename = "backupGeoTargets", skip_serializing_if = "Option::is_none")]
    pub backup_geo_targets: Option<Box<models::RrSetRoutingPolicyGeoPolicy>>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "primaryTargets", skip_serializing_if = "Option::is_none")]
    pub primary_targets: Option<Box<models::RrSetRoutingPolicyHealthCheckTargets>>,
    /// When serving state is PRIMARY, this field provides the option of sending a small percentage of the traffic to the backup targets.
    #[serde(rename = "trickleTraffic", skip_serializing_if = "Option::is_none")]
    pub trickle_traffic: Option<f64>,
}

impl RrSetRoutingPolicyPrimaryBackupPolicy {
    /// Configures a RRSetRoutingPolicy such that all queries are responded with the primary_targets if they are healthy. And if all of them are unhealthy, then we fallback to a geo localized policy.
    pub fn new() -> RrSetRoutingPolicyPrimaryBackupPolicy {
        RrSetRoutingPolicyPrimaryBackupPolicy {
            backup_geo_targets: None,
            kind: None,
            primary_targets: None,
            trickle_traffic: None,
        }
    }
}
