use serde::{Deserialize, Serialize}; /*
                                      * Workflow Executions API
                                      *
                                      * Execute workflows created with Workflows API.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Position : Position contains source position information about the stack trace element such as line number, column number and length of the code block in bytes.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Position {
    /// The source code column position (of the line) the current instruction was generated from.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    /// The number of bytes of source code making up this stack trace element.
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<String>,
    /// The source code line number the current instruction was generated from.
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
}

impl Position {
    /// Position contains source position information about the stack trace element such as line number, column number and length of the code block in bytes.
    pub fn new() -> Position {
        Position {
            column: None,
            length: None,
            line: None,
        }
    }
}
