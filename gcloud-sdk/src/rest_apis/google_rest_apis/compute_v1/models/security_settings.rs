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

/// SecuritySettings : The authentication and authorization settings for a BackendService.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySettings {
    #[serde(
        rename = "awsV4Authentication",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_v4_authentication: Option<Box<models::Awsv4Signature>>,
    /// Optional. A URL referring to a networksecurity.ClientTlsPolicy resource that describes how clients should authenticate with this service's backends. clientTlsPolicy only applies to a global BackendService with the loadBalancingScheme set to INTERNAL_SELF_MANAGED. If left blank, communications are not encrypted.
    #[serde(rename = "clientTlsPolicy", skip_serializing_if = "Option::is_none")]
    pub client_tls_policy: Option<String>,
    /// Optional. A list of Subject Alternative Names (SANs) that the client verifies during a mutual TLS handshake with an server/endpoint for this BackendService. When the server presents its X.509 certificate to the client, the client inspects the certificate's subjectAltName field. If the field contains one of the specified values, the communication continues. Otherwise, it fails. This additional check enables the client to verify that the server is authorized to run the requested service. Note that the contents of the server certificate's subjectAltName field are configured by the Public Key Infrastructure which provisions server identities. Only applies to a global BackendService with loadBalancingScheme set to INTERNAL_SELF_MANAGED. Only applies when BackendService has an attached clientTlsPolicy with clientCertificate (mTLS mode).
    #[serde(rename = "subjectAltNames", skip_serializing_if = "Option::is_none")]
    pub subject_alt_names: Option<Vec<String>>,
}

impl SecuritySettings {
    /// The authentication and authorization settings for a BackendService.
    pub fn new() -> SecuritySettings {
        SecuritySettings {
            aws_v4_authentication: None,
            client_tls_policy: None,
            subject_alt_names: None,
        }
    }
}
