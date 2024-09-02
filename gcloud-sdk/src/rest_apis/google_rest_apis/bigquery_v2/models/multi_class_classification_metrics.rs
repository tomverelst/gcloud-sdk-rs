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

/// MultiClassClassificationMetrics : Evaluation metrics for multi-class classification/classifier models.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiClassClassificationMetrics {
    #[serde(
        rename = "aggregateClassificationMetrics",
        skip_serializing_if = "Option::is_none"
    )]
    pub aggregate_classification_metrics: Option<Box<models::AggregateClassificationMetrics>>,
    /// Confusion matrix at different thresholds.
    #[serde(
        rename = "confusionMatrixList",
        skip_serializing_if = "Option::is_none"
    )]
    pub confusion_matrix_list: Option<Vec<models::ConfusionMatrix>>,
}

impl MultiClassClassificationMetrics {
    /// Evaluation metrics for multi-class classification/classifier models.
    pub fn new() -> MultiClassClassificationMetrics {
        MultiClassClassificationMetrics {
            aggregate_classification_metrics: None,
            confusion_matrix_list: None,
        }
    }
}
