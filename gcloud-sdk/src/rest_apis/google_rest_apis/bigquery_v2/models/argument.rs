use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Argument : Input/output argument of a function or a stored procedure.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Argument {
    /// Optional. Defaults to FIXED_TYPE.
    #[serde(rename = "argumentKind", skip_serializing_if = "Option::is_none")]
    pub argument_kind: Option<ArgumentKind>,
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<Box<crate::google_rest_apis::bigquery_v2::models::StandardSqlDataType>>,
    /// Optional. Specifies whether the argument is input or output. Can be set for procedures only.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// Optional. The name of this argument. Can be absent for function return argument.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Argument {
    /// Input/output argument of a function or a stored procedure.
    pub fn new() -> Argument {
        Argument {
            argument_kind: None,
            data_type: None,
            mode: None,
            name: None,
        }
    }
}

/// Optional. Defaults to FIXED_TYPE.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ArgumentKind {
    #[serde(rename = "ARGUMENT_KIND_UNSPECIFIED")]
    ArgumentKindUnspecified,
    #[serde(rename = "FIXED_TYPE")]
    FixedType,
    #[serde(rename = "ANY_TYPE")]
    AnyType,
}

impl Default for ArgumentKind {
    fn default() -> ArgumentKind {
        Self::ArgumentKindUnspecified
    }
}
/// Optional. Specifies whether the argument is input or output. Can be set for procedures only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "MODE_UNSPECIFIED")]
    ModeUnspecified,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "OUT")]
    Out,
    #[serde(rename = "INOUT")]
    Inout,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::ModeUnspecified
    }
}
