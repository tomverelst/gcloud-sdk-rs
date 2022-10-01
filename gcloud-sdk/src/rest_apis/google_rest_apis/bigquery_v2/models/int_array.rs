use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// IntArray : An array of int.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IntArray {
    /// Elements in the int array.
    #[serde(rename = "elements", skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<String>>,
}

impl IntArray {
    /// An array of int.
    pub fn new() -> IntArray {
        IntArray { elements: None }
    }
}
