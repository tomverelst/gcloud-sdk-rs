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
pub struct SnapshotDefinition {
    #[serde(rename = "baseTableReference", skip_serializing_if = "Option::is_none")]
    pub base_table_reference:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::TableReference>>,
    /// [Required] The time at which the base table was snapshot. This value is reported in the JSON response using RFC3339 format.
    #[serde(rename = "snapshotTime", skip_serializing_if = "Option::is_none")]
    pub snapshot_time: Option<String>,
}

impl SnapshotDefinition {
    pub fn new() -> SnapshotDefinition {
        SnapshotDefinition {
            base_table_reference: None,
            snapshot_time: None,
        }
    }
}
