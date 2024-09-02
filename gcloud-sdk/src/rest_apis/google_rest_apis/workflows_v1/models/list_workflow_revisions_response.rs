use serde::{Deserialize, Serialize}; /*
                                      * Workflows API
                                      *
                                      * Manage workflow definitions. To execute workflows and manage executions, see the Workflows Executions API.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ListWorkflowRevisionsResponse : Response for the ListWorkflowRevisions method.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListWorkflowRevisionsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The revisions of the workflow, ordered in reverse chronological order.
    #[serde(rename = "workflows", skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<crate::google_rest_apis::workflows_v1::models::Workflow>>,
}

impl ListWorkflowRevisionsResponse {
    /// Response for the ListWorkflowRevisions method.
    pub fn new() -> ListWorkflowRevisionsResponse {
        ListWorkflowRevisionsResponse {
            next_page_token: None,
            workflows: None,
        }
    }
}
