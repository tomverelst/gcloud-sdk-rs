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
pub struct MachineTypesScopedList {
    /// [Output Only] A list of machine types contained in this scope.
    #[serde(rename = "machineTypes", skip_serializing_if = "Option::is_none")]
    pub machine_types: Option<Vec<models::MachineType>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::MachineTypesScopedListWarning>>,
}

impl MachineTypesScopedList {
    pub fn new() -> MachineTypesScopedList {
        MachineTypesScopedList {
            machine_types: None,
            warning: None,
        }
    }
}
