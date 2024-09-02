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
pub struct ListRoutinesResponse {
    /// A token to request the next page of results.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// Routines in the requested dataset. Unless read_mask is set in the request, only the following fields are populated: etag, project_id, dataset_id, routine_id, routine_type, creation_time, last_modified_time, and language.
    #[serde(rename = "routines", skip_serializing_if = "Option::is_none")]
    pub routines: Option<Vec<models::Routine>>,
}

impl ListRoutinesResponse {
    pub fn new() -> ListRoutinesResponse {
        ListRoutinesResponse {
            next_page_token: None,
            routines: None,
        }
    }
}
