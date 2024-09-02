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
pub struct RouterStatusResponse {
    /// Type of resource.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::RouterStatus>>,
}

impl RouterStatusResponse {
    pub fn new() -> RouterStatusResponse {
        RouterStatusResponse {
            kind: None,
            result: None,
        }
    }
}
