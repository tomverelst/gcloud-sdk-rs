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

/// GuestAttributesEntry : A guest attributes namespace/key/value entry.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestAttributesEntry {
    /// Key for the guest attribute entry.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Namespace for the guest attribute entry.
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Value for the guest attribute entry.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GuestAttributesEntry {
    /// A guest attributes namespace/key/value entry.
    pub fn new() -> GuestAttributesEntry {
        GuestAttributesEntry {
            key: None,
            namespace: None,
            value: None,
        }
    }
}
