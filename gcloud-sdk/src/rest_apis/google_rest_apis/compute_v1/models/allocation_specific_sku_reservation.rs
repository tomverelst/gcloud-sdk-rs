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

/// AllocationSpecificSkuReservation : This reservation type allows to pre allocate specific instance configuration. Next ID: 6
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocationSpecificSkuReservation {
    /// [Output Only] Indicates how many instances are actually usable currently.
    #[serde(rename = "assuredCount", skip_serializing_if = "Option::is_none")]
    pub assured_count: Option<String>,
    /// Specifies the number of resources that are allocated.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /// [Output Only] Indicates how many instances are in use.
    #[serde(rename = "inUseCount", skip_serializing_if = "Option::is_none")]
    pub in_use_count: Option<String>,
    #[serde(rename = "instanceProperties", skip_serializing_if = "Option::is_none")]
    pub instance_properties:
        Option<Box<models::AllocationSpecificSkuAllocationReservedInstanceProperties>>,
    /// Specifies the instance template to create the reservation. If you use this field, you must exclude the instanceProperties field. This field is optional, and it can be a full or partial URL. For example, the following are all valid URLs to an instance template: - https://www.googleapis.com/compute/v1/projects/project /global/instanceTemplates/instanceTemplate - projects/project/global/instanceTemplates/instanceTemplate - global/instanceTemplates/instanceTemplate
    #[serde(
        rename = "sourceInstanceTemplate",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_instance_template: Option<String>,
}

impl AllocationSpecificSkuReservation {
    /// This reservation type allows to pre allocate specific instance configuration. Next ID: 6
    pub fn new() -> AllocationSpecificSkuReservation {
        AllocationSpecificSkuReservation {
            assured_count: None,
            count: None,
            in_use_count: None,
            instance_properties: None,
            source_instance_template: None,
        }
    }
}
