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

/// GlobalExplanation : Global explanations containing the top most important features after training.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalExplanation {
    /// Class label for this set of global explanations. Will be empty/null for binary logistic and linear regression models. Sorted alphabetically in descending order.
    #[serde(rename = "classLabel", skip_serializing_if = "Option::is_none")]
    pub class_label: Option<String>,
    /// A list of the top global explanations. Sorted by absolute value of attribution in descending order.
    #[serde(rename = "explanations", skip_serializing_if = "Option::is_none")]
    pub explanations: Option<Vec<models::Explanation>>,
}

impl GlobalExplanation {
    /// Global explanations containing the top most important features after training.
    pub fn new() -> GlobalExplanation {
        GlobalExplanation {
            class_label: None,
            explanations: None,
        }
    }
}
