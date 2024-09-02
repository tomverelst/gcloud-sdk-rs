use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AllocationAggregateReservationReservedResourceInfoAccelerator {
    /// Number of accelerators of specified type.
    #[serde(rename = "acceleratorCount", skip_serializing_if = "Option::is_none")]
    pub accelerator_count: Option<i32>,
    /// Full or partial URL to accelerator type. e.g. \"projects/{PROJECT}/zones/{ZONE}/acceleratorTypes/ct4l\"
    #[serde(rename = "acceleratorType", skip_serializing_if = "Option::is_none")]
    pub accelerator_type: Option<String>,
}

impl AllocationAggregateReservationReservedResourceInfoAccelerator {
    pub fn new() -> AllocationAggregateReservationReservedResourceInfoAccelerator {
        AllocationAggregateReservationReservedResourceInfoAccelerator {
            accelerator_count: None,
            accelerator_type: None,
        }
    }
}
