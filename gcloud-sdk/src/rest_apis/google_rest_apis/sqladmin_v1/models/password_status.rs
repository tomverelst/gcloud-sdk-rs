use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// PasswordStatus : Read-only password status.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PasswordStatus {
    /// If true, user does not have login privileges.
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// The expiration time of the current password.
    #[serde(
        rename = "passwordExpirationTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub password_expiration_time: Option<String>,
}

impl PasswordStatus {
    /// Read-only password status.
    pub fn new() -> PasswordStatus {
        PasswordStatus {
            locked: None,
            password_expiration_time: None,
        }
    }
}
