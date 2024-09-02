/*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::sqladmin_v1::models;
use serde::{Deserialize, Serialize};

/// SyncFlags : Initial sync flags for certain Cloud SQL APIs. Currently used for the MySQL external server initial dump.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncFlags {
    /// The name of the flag.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value of the flag. This field must be omitted if the flag doesn't take a value.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SyncFlags {
    /// Initial sync flags for certain Cloud SQL APIs. Currently used for the MySQL external server initial dump.
    pub fn new() -> SyncFlags {
        SyncFlags {
            name: None,
            value: None,
        }
    }
}
