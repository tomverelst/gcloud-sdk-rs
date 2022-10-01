use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JobCancelResponse {
    #[serde(rename = "job", skip_serializing_if = "Option::is_none")]
    pub job: Option<Box<crate::google_rest_apis::bigquery_v2::models::Job>>,
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
