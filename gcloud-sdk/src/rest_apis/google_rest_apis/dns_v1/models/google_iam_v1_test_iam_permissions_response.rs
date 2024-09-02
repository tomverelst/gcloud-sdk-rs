/*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::dns_v1::models;
use serde::{Deserialize, Serialize};

/// GoogleIamV1TestIamPermissionsResponse : Response message for `TestIamPermissions` method.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoogleIamV1TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl GoogleIamV1TestIamPermissionsResponse {
    /// Response message for `TestIamPermissions` method.
    pub fn new() -> GoogleIamV1TestIamPermissionsResponse {
        GoogleIamV1TestIamPermissionsResponse { permissions: None }
    }
}
