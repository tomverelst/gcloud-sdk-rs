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
pub struct JobConfigurationExtract {
    /// [Optional] The compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE. The default value is NONE. DEFLATE and SNAPPY are only supported for Avro. Not applicable when extracting models.
    #[serde(rename = "compression", skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    /// [Optional] The exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON, PARQUET or AVRO for tables and ML_TF_SAVED_MODEL or ML_XGBOOST_BOOSTER for models. The default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV. The default value for models is ML_TF_SAVED_MODEL.
    #[serde(rename = "destinationFormat", skip_serializing_if = "Option::is_none")]
    pub destination_format: Option<String>,
    /// [Pick one] DEPRECATED: Use destinationUris instead, passing only one URI as necessary. The fully-qualified Google Cloud Storage URI where the extracted table should be written.
    #[serde(rename = "destinationUri", skip_serializing_if = "Option::is_none")]
    pub destination_uri: Option<String>,
    /// [Pick one] A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written.
    #[serde(rename = "destinationUris", skip_serializing_if = "Option::is_none")]
    pub destination_uris: Option<Vec<String>>,
    /// [Optional] Delimiter to use between fields in the exported data. Default is ','. Not applicable when extracting models.
    #[serde(rename = "fieldDelimiter", skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    /// [Optional] Whether to print out a header row in the results. Default is true. Not applicable when extracting models.
    #[serde(rename = "printHeader", skip_serializing_if = "Option::is_none")]
    pub print_header: Option<bool>,
    #[serde(rename = "sourceModel", skip_serializing_if = "Option::is_none")]
    pub source_model: Option<Box<models::ModelReference>>,
    #[serde(rename = "sourceTable", skip_serializing_if = "Option::is_none")]
    pub source_table: Option<Box<models::TableReference>>,
    /// [Optional] If destinationFormat is set to \"AVRO\", this flag indicates whether to enable extracting applicable column types (such as TIMESTAMP) to their corresponding AVRO logical types (timestamp-micros), instead of only using their raw types (avro-long). Not applicable when extracting models.
    #[serde(
        rename = "useAvroLogicalTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_avro_logical_types: Option<bool>,
}

impl JobConfigurationExtract {
    pub fn new() -> JobConfigurationExtract {
        JobConfigurationExtract {
            compression: None,
            destination_format: None,
            destination_uri: None,
            destination_uris: None,
            field_delimiter: None,
            print_header: None,
            source_model: None,
            source_table: None,
            use_avro_logical_types: None,
        }
    }
}
