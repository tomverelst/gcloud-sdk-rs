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

/// UsersListResponse : User list response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsersListResponse {
    /// List of user resources in the instance.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::User>>,
    /// This is always `sql#usersList`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// An identifier that uniquely identifies the operation. You can use this identifier to retrieve the Operations resource that has information about the operation.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl UsersListResponse {
    /// User list response.
    pub fn new() -> UsersListResponse {
        UsersListResponse {
            items: None,
            kind: None,
            next_page_token: None,
        }
    }
}
