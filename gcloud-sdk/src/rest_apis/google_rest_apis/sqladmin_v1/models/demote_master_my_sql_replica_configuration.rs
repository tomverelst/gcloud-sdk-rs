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

/// DemoteMasterMySqlReplicaConfiguration : Read-replica configuration specific to MySQL databases.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DemoteMasterMySqlReplicaConfiguration {
    /// PEM representation of the trusted CA's x509 certificate.
    #[serde(rename = "caCertificate", skip_serializing_if = "Option::is_none")]
    pub ca_certificate: Option<String>,
    /// PEM representation of the replica's x509 certificate.
    #[serde(rename = "clientCertificate", skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<String>,
    /// PEM representation of the replica's private key. The corresponsing public key is encoded in the client's certificate. The format of the replica's private key can be either PKCS #1 or PKCS #8.
    #[serde(rename = "clientKey", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,
    /// This is always `sql#demoteMasterMysqlReplicaConfiguration`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The password for the replication connection.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// The username for the replication connection.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl DemoteMasterMySqlReplicaConfiguration {
    /// Read-replica configuration specific to MySQL databases.
    pub fn new() -> DemoteMasterMySqlReplicaConfiguration {
        DemoteMasterMySqlReplicaConfiguration {
            ca_certificate: None,
            client_certificate: None,
            client_key: None,
            kind: None,
            password: None,
            username: None,
        }
    }
}
