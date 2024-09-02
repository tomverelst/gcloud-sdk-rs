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

/// ShieldedInstanceIdentityEntry : A Shielded Instance Identity Entry.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShieldedInstanceIdentityEntry {
    /// A PEM-encoded X.509 certificate. This field can be empty.
    #[serde(rename = "ekCert", skip_serializing_if = "Option::is_none")]
    pub ek_cert: Option<String>,
    /// A PEM-encoded public key.
    #[serde(rename = "ekPub", skip_serializing_if = "Option::is_none")]
    pub ek_pub: Option<String>,
}

impl ShieldedInstanceIdentityEntry {
    /// A Shielded Instance Identity Entry.
    pub fn new() -> ShieldedInstanceIdentityEntry {
        ShieldedInstanceIdentityEntry {
            ek_cert: None,
            ek_pub: None,
        }
    }
}
