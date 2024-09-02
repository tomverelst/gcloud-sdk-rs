/*
                                      * Workflow Executions API
                                      *
                                      * Execute workflows created with Workflows API.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::workflowexecutions_v1::models;
use serde::{Deserialize, Serialize};

/// ListExecutionsResponse : Response for the ListExecutions method.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListExecutionsResponse {
    /// The executions which match the request.
    #[serde(rename = "executions", skip_serializing_if = "Option::is_none")]
    pub executions: Option<Vec<models::Execution>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListExecutionsResponse {
    /// Response for the ListExecutions method.
    pub fn new() -> ListExecutionsResponse {
        ListExecutionsResponse {
            executions: None,
            next_page_token: None,
        }
    }
}
