use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ExportContextSqlExportOptionsMysqlExportOptions : Options for exporting from MySQL.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExportContextSqlExportOptionsMysqlExportOptions {
    /// Option to include SQL statement required to set up replication. If set to `1`, the dump file includes a CHANGE MASTER TO statement with the binary log coordinates, and --set-gtid-purged is set to ON. If set to `2`, the CHANGE MASTER TO statement is written as a SQL comment and has no effect. If set to any value other than `1`, --set-gtid-purged is set to OFF.
    #[serde(rename = "masterData", skip_serializing_if = "Option::is_none")]
    pub master_data: Option<i32>,
}

impl ExportContextSqlExportOptionsMysqlExportOptions {
    /// Options for exporting from MySQL.
    pub fn new() -> ExportContextSqlExportOptionsMysqlExportOptions {
        ExportContextSqlExportOptionsMysqlExportOptions { master_data: None }
    }
}
