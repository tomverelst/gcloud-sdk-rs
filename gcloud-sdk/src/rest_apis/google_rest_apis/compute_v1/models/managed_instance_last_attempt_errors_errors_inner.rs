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
pub struct ManagedInstanceLastAttemptErrorsErrorsInner {
    /// [Output Only] The error type identifier for this error.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// [Output Only] An optional list of messages that contain the error details. There is a set of defined message types to use for providing details.The syntax depends on the error code. For example, QuotaExceededInfo will have details when the error code is QUOTA_EXCEEDED.
    #[serde(rename = "errorDetails", skip_serializing_if = "Option::is_none")]
    pub error_details:
        Option<Vec<models::ManagedInstanceLastAttemptErrorsErrorsInnerErrorDetailsInner>>,
    /// [Output Only] Indicates the field in the request that caused the error. This property is optional.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// [Output Only] An optional, human-readable error message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ManagedInstanceLastAttemptErrorsErrorsInner {
    pub fn new() -> ManagedInstanceLastAttemptErrorsErrorsInner {
        ManagedInstanceLastAttemptErrorsErrorsInner {
            code: None,
            error_details: None,
            location: None,
            message: None,
        }
    }
}
