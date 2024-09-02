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
pub struct SnapshotSettings {
    #[serde(rename = "storageLocation", skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<Box<models::SnapshotSettingsStorageLocationSettings>>,
}

impl SnapshotSettings {
    pub fn new() -> SnapshotSettings {
        SnapshotSettings {
            storage_location: None,
        }
    }
}
