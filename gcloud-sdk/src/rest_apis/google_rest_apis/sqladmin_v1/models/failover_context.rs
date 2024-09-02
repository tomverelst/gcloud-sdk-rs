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

/// FailoverContext : Database instance failover context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverContext {
    /// This is always `sql#failoverContext`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The current settings version of this instance. Request will be rejected if this version doesn't match the current settings version.
    #[serde(rename = "settingsVersion", skip_serializing_if = "Option::is_none")]
    pub settings_version: Option<String>,
}

impl FailoverContext {
    /// Database instance failover context.
    pub fn new() -> FailoverContext {
        FailoverContext {
            kind: None,
            settings_version: None,
        }
    }
}
