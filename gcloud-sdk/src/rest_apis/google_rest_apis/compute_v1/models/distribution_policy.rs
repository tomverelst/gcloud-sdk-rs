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
pub struct DistributionPolicy {
    /// The distribution shape to which the group converges either proactively or on resize events (depending on the value set in updatePolicy.instanceRedistributionType).
    #[serde(rename = "targetShape", skip_serializing_if = "Option::is_none")]
    pub target_shape: Option<TargetShape>,
    /// Zones where the regional managed instance group will create and manage its instances.
    #[serde(rename = "zones", skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<models::DistributionPolicyZoneConfiguration>>,
}

impl DistributionPolicy {
    pub fn new() -> DistributionPolicy {
        DistributionPolicy {
            target_shape: None,
            zones: None,
        }
    }
}
/// The distribution shape to which the group converges either proactively or on resize events (depending on the value set in updatePolicy.instanceRedistributionType).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TargetShape {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "ANY_SINGLE_ZONE")]
    AnySingleZone,
    #[serde(rename = "BALANCED")]
    Balanced,
    #[serde(rename = "EVEN")]
    Even,
}

impl Default for TargetShape {
    fn default() -> TargetShape {
        Self::Any
    }
}
