/*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::bigquery_v2::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCancelResponse {
    #[serde(rename = "job", skip_serializing_if = "Option::is_none")]
    pub job: Option<Box<models::Job>>,
    /// The resource type of the response.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl JobCancelResponse {
    pub fn new() -> JobCancelResponse {
        JobCancelResponse {
            job: None,
            kind: None,
        }
    }
}
