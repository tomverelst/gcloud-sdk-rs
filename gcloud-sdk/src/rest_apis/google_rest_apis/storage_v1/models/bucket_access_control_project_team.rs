/*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::storage_v1::models;
use serde::{Deserialize, Serialize};

/// BucketAccessControlProjectTeam : The project team associated with the entity, if any.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BucketAccessControlProjectTeam {
    /// The project number.
    #[serde(rename = "projectNumber", skip_serializing_if = "Option::is_none")]
    pub project_number: Option<String>,
    /// The team.
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}

impl BucketAccessControlProjectTeam {
    /// The project team associated with the entity, if any.
    pub fn new() -> BucketAccessControlProjectTeam {
        BucketAccessControlProjectTeam {
            project_number: None,
            team: None,
        }
    }
}
