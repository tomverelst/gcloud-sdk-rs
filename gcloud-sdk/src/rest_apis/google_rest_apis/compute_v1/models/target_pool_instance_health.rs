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
pub struct TargetPoolInstanceHealth {
    #[serde(rename = "healthStatus", skip_serializing_if = "Option::is_none")]
    pub health_status: Option<Vec<models::HealthStatus>>,
    /// [Output Only] Type of resource. Always compute#targetPoolInstanceHealth when checking the health of an instance.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl TargetPoolInstanceHealth {
    pub fn new() -> TargetPoolInstanceHealth {
        TargetPoolInstanceHealth {
            health_status: None,
            kind: None,
        }
    }
}
