use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Routine : A user-defined function or a stored procedure.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Routine {
    /// Optional.
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<crate::google_rest_apis::bigquery_v2::models::Argument>>,
    /// Output only. The time when this routine was created, in milliseconds since the epoch.
    #[serde(rename = "creationTime", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// Required. The body of the routine. For functions, this is the expression in the AS clause. If language=SQL, it is the substring inside (but excluding) the parentheses. For example, for the function created with the following statement: `CREATE FUNCTION JoinLines(x string, y string) as (concat(x, \"\\n\", y))` The definition_body is `concat(x, \"\\n\", y)` (\\n is not replaced with linebreak). If language=JAVASCRIPT, it is the evaluated string in the AS clause. For example, for the function created with the following statement: `CREATE FUNCTION f() RETURNS STRING LANGUAGE js AS 'return \"\\n\";\\n'` The definition_body is `return \"\\n\";\\n` Note that both \\n are replaced with linebreaks.
    #[serde(rename = "definitionBody", skip_serializing_if = "Option::is_none")]
    pub definition_body: Option<String>,
    /// Optional. The description of the routine, if defined.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. The determinism level of the JavaScript UDF, if defined.
    #[serde(rename = "determinismLevel", skip_serializing_if = "Option::is_none")]
    pub determinism_level: Option<DeterminismLevel>,
    /// Output only. A hash of this resource.
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// Optional. If language = \"JAVASCRIPT\", this field stores the path of the imported JAVASCRIPT libraries.
    #[serde(rename = "importedLibraries", skip_serializing_if = "Option::is_none")]
    pub imported_libraries: Option<Vec<String>>,
    /// Optional. Defaults to \"SQL\".
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// Output only. The time when this routine was last modified, in milliseconds since the epoch.
    #[serde(rename = "lastModifiedTime", skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(
        rename = "remoteFunctionOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_function_options:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::RemoteFunctionOptions>>,
    #[serde(rename = "returnTableType", skip_serializing_if = "Option::is_none")]
    pub return_table_type:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::StandardSqlTableType>>,
    #[serde(rename = "returnType", skip_serializing_if = "Option::is_none")]
    pub return_type: Option<Box<crate::google_rest_apis::bigquery_v2::models::StandardSqlDataType>>,
    #[serde(rename = "routineReference", skip_serializing_if = "Option::is_none")]
    pub routine_reference:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::RoutineReference>>,
    /// Required. The type of routine.
    #[serde(rename = "routineType", skip_serializing_if = "Option::is_none")]
    pub routine_type: Option<RoutineType>,
    #[serde(rename = "sparkOptions", skip_serializing_if = "Option::is_none")]
    pub spark_options: Option<Box<crate::google_rest_apis::bigquery_v2::models::SparkOptions>>,
    /// Optional. Can be set for procedures only. If true (default), the definition body will be validated in the creation and the updates of the procedure. For procedures with an argument of ANY TYPE, the definition body validtion is not supported at creation/update time, and thus this field must be set to false explicitly.
    #[serde(rename = "strictMode", skip_serializing_if = "Option::is_none")]
    pub strict_mode: Option<bool>,
}

impl Routine {
    /// A user-defined function or a stored procedure.
    pub fn new() -> Routine {
        Routine {
            arguments: None,
            creation_time: None,
            definition_body: None,
            description: None,
            determinism_level: None,
            etag: None,
            imported_libraries: None,
            language: None,
            last_modified_time: None,
            remote_function_options: None,
            return_table_type: None,
            return_type: None,
            routine_reference: None,
            routine_type: None,
            spark_options: None,
            strict_mode: None,
        }
    }
}

/// Optional. The determinism level of the JavaScript UDF, if defined.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeterminismLevel {
    #[serde(rename = "DETERMINISM_LEVEL_UNSPECIFIED")]
    DeterminismLevelUnspecified,
    #[serde(rename = "DETERMINISTIC")]
    Deterministic,
    #[serde(rename = "NOT_DETERMINISTIC")]
    NotDeterministic,
}

impl Default for DeterminismLevel {
    fn default() -> DeterminismLevel {
        Self::DeterminismLevelUnspecified
    }
}
/// Optional. Defaults to \"SQL\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "LANGUAGE_UNSPECIFIED")]
    LanguageUnspecified,
    #[serde(rename = "SQL")]
    Sql,
    #[serde(rename = "JAVASCRIPT")]
    Javascript,
    #[serde(rename = "PYTHON")]
    Python,
}

impl Default for Language {
    fn default() -> Language {
        Self::LanguageUnspecified
    }
}
/// Required. The type of routine.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoutineType {
    #[serde(rename = "ROUTINE_TYPE_UNSPECIFIED")]
    RoutineTypeUnspecified,
    #[serde(rename = "SCALAR_FUNCTION")]
    ScalarFunction,
    #[serde(rename = "PROCEDURE")]
    Procedure,
    #[serde(rename = "TABLE_VALUED_FUNCTION")]
    TableValuedFunction,
}

impl Default for RoutineType {
    fn default() -> RoutineType {
        Self::RoutineTypeUnspecified
    }
}
