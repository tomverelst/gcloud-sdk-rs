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

/// BucketEncryption : Encryption configuration for a bucket.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BucketEncryption {
    /// A Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified.
    #[serde(rename = "defaultKmsKeyName", skip_serializing_if = "Option::is_none")]
    pub default_kms_key_name: Option<String>,
}

impl BucketEncryption {
    /// Encryption configuration for a bucket.
    pub fn new() -> BucketEncryption {
        BucketEncryption {
            default_kms_key_name: None,
        }
    }
}
