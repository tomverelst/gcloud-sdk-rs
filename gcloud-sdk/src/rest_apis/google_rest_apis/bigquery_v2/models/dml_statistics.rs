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
pub struct DmlStatistics {
    /// Number of deleted Rows. populated by DML DELETE, MERGE and TRUNCATE statements.
    #[serde(rename = "deletedRowCount", skip_serializing_if = "Option::is_none")]
    pub deleted_row_count: Option<String>,
    /// Number of inserted Rows. Populated by DML INSERT and MERGE statements.
    #[serde(rename = "insertedRowCount", skip_serializing_if = "Option::is_none")]
    pub inserted_row_count: Option<String>,
    /// Number of updated Rows. Populated by DML UPDATE and MERGE statements.
    #[serde(rename = "updatedRowCount", skip_serializing_if = "Option::is_none")]
    pub updated_row_count: Option<String>,
}

impl DmlStatistics {
    pub fn new() -> DmlStatistics {
        DmlStatistics {
            deleted_row_count: None,
            inserted_row_count: None,
            updated_row_count: None,
        }
    }
}
