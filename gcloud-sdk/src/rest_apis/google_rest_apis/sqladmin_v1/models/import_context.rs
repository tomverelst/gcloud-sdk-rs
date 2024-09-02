/*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::sqladmin_v1::models;
use serde::{Deserialize, Serialize};

/// ImportContext : Database instance import context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportContext {
    #[serde(rename = "bakImportOptions", skip_serializing_if = "Option::is_none")]
    pub bak_import_options: Option<Box<models::ImportContextBakImportOptions>>,
    #[serde(rename = "csvImportOptions", skip_serializing_if = "Option::is_none")]
    pub csv_import_options: Option<Box<models::ImportContextCsvImportOptions>>,
    /// The target database for the import. If `fileType` is `SQL`, this field is required only if the import file does not specify a database, and is overridden by any database specification in the import file. If `fileType` is `CSV`, one database must be specified.
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// The file type for the specified uri.\\`SQL`: The file contains SQL statements. \\`CSV`: The file contains CSV data.
    #[serde(rename = "fileType", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<FileType>,
    /// The PostgreSQL user for this import operation. PostgreSQL instances only.
    #[serde(rename = "importUser", skip_serializing_if = "Option::is_none")]
    pub import_user: Option<String>,
    /// This is always `sql#importContext`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Path to the import file in Cloud Storage, in the form `gs://bucketName/fileName`. Compressed gzip files (.gz) are supported when `fileType` is `SQL`. The instance must have write permissions to the bucket and read access to the file.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ImportContext {
    /// Database instance import context.
    pub fn new() -> ImportContext {
        ImportContext {
            bak_import_options: None,
            csv_import_options: None,
            database: None,
            file_type: None,
            import_user: None,
            kind: None,
            uri: None,
        }
    }
}
/// The file type for the specified uri.\\`SQL`: The file contains SQL statements. \\`CSV`: The file contains CSV data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileType {
    #[serde(rename = "SQL_FILE_TYPE_UNSPECIFIED")]
    SqlFileTypeUnspecified,
    #[serde(rename = "SQL")]
    Sql,
    #[serde(rename = "CSV")]
    Csv,
    #[serde(rename = "BAK")]
    Bak,
}

impl Default for FileType {
    fn default() -> FileType {
        Self::SqlFileTypeUnspecified
    }
}
