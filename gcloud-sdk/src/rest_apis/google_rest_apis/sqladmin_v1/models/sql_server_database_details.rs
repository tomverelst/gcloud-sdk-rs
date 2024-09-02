use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// SqlServerDatabaseDetails : Represents a Sql Server database on the Cloud SQL instance.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SqlServerDatabaseDetails {
    /// The version of SQL Server with which the database is to be made compatible
    #[serde(rename = "compatibilityLevel", skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<i32>,
    /// The recovery model of a SQL Server database
    #[serde(rename = "recoveryModel", skip_serializing_if = "Option::is_none")]
    pub recovery_model: Option<String>,
}

impl SqlServerDatabaseDetails {
    /// Represents a Sql Server database on the Cloud SQL instance.
    pub fn new() -> SqlServerDatabaseDetails {
        SqlServerDatabaseDetails {
            compatibility_level: None,
            recovery_model: None,
        }
    }
}
