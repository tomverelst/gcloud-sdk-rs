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
pub struct BackendServicesScopedList {
    /// A list of BackendServices contained in this scope.
    #[serde(rename = "backendServices", skip_serializing_if = "Option::is_none")]
    pub backend_services: Option<Vec<models::BackendService>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::BackendServicesScopedListWarning>>,
}

impl BackendServicesScopedList {
    pub fn new() -> BackendServicesScopedList {
        BackendServicesScopedList {
            backend_services: None,
            warning: None,
        }
    }
}
