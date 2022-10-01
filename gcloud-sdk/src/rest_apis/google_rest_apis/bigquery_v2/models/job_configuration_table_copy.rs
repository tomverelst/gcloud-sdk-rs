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
pub struct JobConfigurationTableCopy {
    /// [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename = "createDisposition", skip_serializing_if = "Option::is_none")]
    pub create_disposition: Option<String>,
    #[serde(
        rename = "destinationEncryptionConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_encryption_configuration:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::EncryptionConfiguration>>,
    /// [Optional] The time when the destination table expires. Expired tables will be deleted and their storage reclaimed.
    #[serde(
        rename = "destinationExpirationTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_expiration_time: Option<serde_json::Value>,
    #[serde(rename = "destinationTable", skip_serializing_if = "Option::is_none")]
    pub destination_table:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::TableReference>>,
    /// [Optional] Supported operation types in table copy job.
    #[serde(rename = "operationType", skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[serde(rename = "sourceTable", skip_serializing_if = "Option::is_none")]
    pub source_table: Option<Box<crate::google_rest_apis::bigquery_v2::models::TableReference>>,
    /// [Pick one] Source tables to copy.
    #[serde(rename = "sourceTables", skip_serializing_if = "Option::is_none")]
    pub source_tables: Option<Vec<crate::google_rest_apis::bigquery_v2::models::TableReference>>,
    /// [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename = "writeDisposition", skip_serializing_if = "Option::is_none")]
    pub write_disposition: Option<String>,
}

impl JobConfigurationTableCopy {
    pub fn new() -> JobConfigurationTableCopy {
        JobConfigurationTableCopy {
            create_disposition: None,
            destination_encryption_configuration: None,
            destination_expiration_time: None,
            destination_table: None,
            operation_type: None,
            source_table: None,
            source_tables: None,
            write_disposition: None,
        }
    }
}
