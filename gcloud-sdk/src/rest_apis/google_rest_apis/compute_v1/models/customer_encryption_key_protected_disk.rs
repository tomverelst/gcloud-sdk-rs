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
pub struct CustomerEncryptionKeyProtectedDisk {
    #[serde(rename = "diskEncryptionKey", skip_serializing_if = "Option::is_none")]
    pub disk_encryption_key: Option<Box<models::CustomerEncryptionKey>>,
    /// Specifies a valid partial or full URL to an existing Persistent Disk resource. This field is only applicable for persistent disks. For example: \"source\": \"/compute/v1/projects/project_id/zones/zone/disks/ disk_name
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl CustomerEncryptionKeyProtectedDisk {
    pub fn new() -> CustomerEncryptionKeyProtectedDisk {
        CustomerEncryptionKeyProtectedDisk {
            disk_encryption_key: None,
            source: None,
        }
    }
}
