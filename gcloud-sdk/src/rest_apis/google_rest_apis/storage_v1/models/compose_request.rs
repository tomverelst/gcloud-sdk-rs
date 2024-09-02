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

/// ComposeRequest : A Compose request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposeRequest {
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<models::Object>>,
    /// The kind of item this is.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The list of source objects that will be concatenated into a single object.
    #[serde(rename = "sourceObjects", skip_serializing_if = "Option::is_none")]
    pub source_objects: Option<Vec<models::ComposeRequestSourceObjectsInner>>,
}

impl ComposeRequest {
    /// A Compose request.
    pub fn new() -> ComposeRequest {
        ComposeRequest {
            destination: None,
            kind: None,
            source_objects: None,
        }
    }
}
