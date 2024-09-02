/*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::dns_v1::models;
use serde::{Deserialize, Serialize};

/// ManagedZoneServiceDirectoryConfig : Contains information about Service Directory-backed zones.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedZoneServiceDirectoryConfig {
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Box<models::ManagedZoneServiceDirectoryConfigNamespace>>,
}

impl ManagedZoneServiceDirectoryConfig {
    /// Contains information about Service Directory-backed zones.
    pub fn new() -> ManagedZoneServiceDirectoryConfig {
        ManagedZoneServiceDirectoryConfig {
            kind: None,
            namespace: None,
        }
    }
}
