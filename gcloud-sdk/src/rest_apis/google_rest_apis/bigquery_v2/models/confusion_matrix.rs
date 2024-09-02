use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ConfusionMatrix : Confusion matrix for multi-class classification models.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfusionMatrix {
    /// Confidence threshold used when computing the entries of the confusion matrix.
    #[serde(
        rename = "confidenceThreshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub confidence_threshold: Option<f64>,
    /// One row per actual label.
    #[serde(rename = "rows", skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<crate::google_rest_apis::bigquery_v2::models::Row>>,
}

impl ConfusionMatrix {
    /// Confusion matrix for multi-class classification models.
    pub fn new() -> ConfusionMatrix {
        ConfusionMatrix {
            confidence_threshold: None,
            rows: None,
        }
    }
}
