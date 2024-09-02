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

/// XpnResourceId : Service resource (a.k.a service project) ID.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct XpnResourceId {
    /// The ID of the service resource. In the case of projects, this field supports project id (e.g., my-project-123) and project number (e.g. 12345678).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the service resource.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl XpnResourceId {
    /// Service resource (a.k.a service project) ID.
    pub fn new() -> XpnResourceId {
        XpnResourceId {
            id: None,
            r#type: None,
        }
    }
}
/// The type of the service resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "XPN_RESOURCE_TYPE_UNSPECIFIED")]
    XpnResourceTypeUnspecified,
}

impl Default for Type {
    fn default() -> Type {
        Self::Project
    }
}
