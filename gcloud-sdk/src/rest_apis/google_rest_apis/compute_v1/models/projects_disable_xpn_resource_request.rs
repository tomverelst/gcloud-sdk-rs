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
pub struct ProjectsDisableXpnResourceRequest {
    #[serde(rename = "xpnResource", skip_serializing_if = "Option::is_none")]
    pub xpn_resource: Option<Box<models::XpnResourceId>>,
}

impl ProjectsDisableXpnResourceRequest {
    pub fn new() -> ProjectsDisableXpnResourceRequest {
        ProjectsDisableXpnResourceRequest { xpn_resource: None }
    }
}
