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
pub struct InstancesSetMinCpuPlatformRequest {
    /// Minimum cpu/platform this instance should be started at.
    #[serde(rename = "minCpuPlatform", skip_serializing_if = "Option::is_none")]
    pub min_cpu_platform: Option<String>,
}

impl InstancesSetMinCpuPlatformRequest {
    pub fn new() -> InstancesSetMinCpuPlatformRequest {
        InstancesSetMinCpuPlatformRequest {
            min_cpu_platform: None,
        }
    }
}
