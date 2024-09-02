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

/// ObjectCustomerEncryption : Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectCustomerEncryption {
    /// The encryption algorithm.
    #[serde(
        rename = "encryptionAlgorithm",
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_algorithm: Option<String>,
    /// SHA256 hash value of the encryption key.
    #[serde(rename = "keySha256", skip_serializing_if = "Option::is_none")]
    pub key_sha256: Option<String>,
}

impl ObjectCustomerEncryption {
    /// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
    pub fn new() -> ObjectCustomerEncryption {
        ObjectCustomerEncryption {
            encryption_algorithm: None,
            key_sha256: None,
        }
    }
}
