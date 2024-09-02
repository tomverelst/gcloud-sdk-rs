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

/// AggregateClassificationMetrics : Aggregate metrics for classification/classifier models. For multi-class models, the metrics are either macro-averaged or micro-averaged. When macro-averaged, the metrics are calculated for each label and then an unweighted average is taken of those values. When micro-averaged, the metric is calculated globally by counting the total number of correctly predicted rows.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateClassificationMetrics {
    /// Accuracy is the fraction of predictions given the correct label. For multiclass this is a micro-averaged metric.
    #[serde(rename = "accuracy", skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
    /// The F1 score is an average of recall and precision. For multiclass this is a macro-averaged metric.
    #[serde(rename = "f1Score", skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f64>,
    /// Logarithmic Loss. For multiclass this is a macro-averaged metric.
    #[serde(rename = "logLoss", skip_serializing_if = "Option::is_none")]
    pub log_loss: Option<f64>,
    /// Precision is the fraction of actual positive predictions that had positive actual labels. For multiclass this is a macro-averaged metric treating each class as a binary classifier.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    /// Recall is the fraction of actual positive labels that were given a positive prediction. For multiclass this is a macro-averaged metric.
    #[serde(rename = "recall", skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
    /// Area Under a ROC Curve. For multiclass this is a macro-averaged metric.
    #[serde(rename = "rocAuc", skip_serializing_if = "Option::is_none")]
    pub roc_auc: Option<f64>,
    /// Threshold at which the metrics are computed. For binary classification models this is the positive class threshold. For multi-class classfication models this is the confidence threshold.
    #[serde(rename = "threshold", skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

impl AggregateClassificationMetrics {
    /// Aggregate metrics for classification/classifier models. For multi-class models, the metrics are either macro-averaged or micro-averaged. When macro-averaged, the metrics are calculated for each label and then an unweighted average is taken of those values. When micro-averaged, the metric is calculated globally by counting the total number of correctly predicted rows.
    pub fn new() -> AggregateClassificationMetrics {
        AggregateClassificationMetrics {
            accuracy: None,
            f1_score: None,
            log_loss: None,
            precision: None,
            recall: None,
            roc_auc: None,
            threshold: None,
        }
    }
}
