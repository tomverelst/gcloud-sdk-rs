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

/// SslCertsInsertRequest : SslCerts insert request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SslCertsInsertRequest {
    /// User supplied name. Must be a distinct name from the other certificates for this instance.
    #[serde(rename = "commonName", skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
}

impl SslCertsInsertRequest {
    /// SslCerts insert request.
    pub fn new() -> SslCertsInsertRequest {
        SslCertsInsertRequest { common_name: None }
    }
}
