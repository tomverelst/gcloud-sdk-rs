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
pub struct UrlMapsScopedList {
    /// A list of UrlMaps contained in this scope.
    #[serde(rename = "urlMaps", skip_serializing_if = "Option::is_none")]
    pub url_maps: Option<Vec<models::UrlMap>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::BackendServicesScopedListWarning>>,
}

impl UrlMapsScopedList {
    pub fn new() -> UrlMapsScopedList {
        UrlMapsScopedList {
            url_maps: None,
            warning: None,
        }
    }
}
