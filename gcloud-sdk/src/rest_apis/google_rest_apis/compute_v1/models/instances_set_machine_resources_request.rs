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
pub struct InstancesSetMachineResourcesRequest {
    /// A list of the type and count of accelerator cards attached to the instance.
    #[serde(rename = "guestAccelerators", skip_serializing_if = "Option::is_none")]
    pub guest_accelerators: Option<Vec<models::AcceleratorConfig>>,
}

impl InstancesSetMachineResourcesRequest {
    pub fn new() -> InstancesSetMachineResourcesRequest {
        InstancesSetMachineResourcesRequest {
            guest_accelerators: None,
        }
    }
}
