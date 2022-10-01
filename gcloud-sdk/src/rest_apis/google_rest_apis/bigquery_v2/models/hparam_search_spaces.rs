use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// HparamSearchSpaces : Hyperparameter search spaces. These should be a subset of training_options.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HparamSearchSpaces {
    #[serde(rename = "activationFn", skip_serializing_if = "Option::is_none")]
    pub activation_fn:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::StringHparamSearchSpace>>,
    #[serde(rename = "batchSize", skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<Box<crate::google_rest_apis::bigquery_v2::models::IntHparamSearchSpace>>,
    #[serde(rename = "boosterType", skip_serializing_if = "Option::is_none")]
    pub booster_type:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::StringHparamSearchSpace>>,
    #[serde(rename = "colsampleBylevel", skip_serializing_if = "Option::is_none")]
    pub colsample_bylevel:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleHparamSearchSpace>>,
    #[serde(rename = "colsampleBynode", skip_serializing_if = "Option::is_none")]
    pub colsample_bynode:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleHparamSearchSpace>>,
    #[serde(rename = "colsampleBytree", skip_serializing_if = "Option::is_none")]
    pub colsample_bytree:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleHparamSearchSpace>>,
    #[serde(rename = "dartNormalizeType", skip_serializing_if = "Option::is_none")]
    pub dart_normalize_type:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::StringHparamSearchSpace>>,
    #[serde(rename = "dropout", skip_serializing_if = "Option::is_none")]
    pub dropout: Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleHparamSearchSpace>>,
    #[serde(rename = "hiddenUnits", skip_serializing_if = "Option::is_none")]
    pub hidden_units:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::IntArrayHparamSearchSpace>>,
    #[serde(rename = "l1Reg", skip_serializing_if = "Option::is_none")]
    pub l1_reg: Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleHparamSearchSpace>>,
    #[serde(rename = "l2Reg", skip_serializing_if = "Option::is_none")]
    pub l2_reg: Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleHparamSearchSpace>>,
    #[serde(rename = "learnRate", skip_serializing_if = "Option::is_none")]
    pub learn_rate:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleHparamSearchSpace>>,
    #[serde(rename = "maxTreeDepth", skip_serializing_if = "Option::is_none")]
    pub max_tree_depth:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::IntHparamSearchSpace>>,
    #[serde(rename = "minSplitLoss", skip_serializing_if = "Option::is_none")]
    pub min_split_loss:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleHparamSearchSpace>>,
    #[serde(rename = "minTreeChildWeight", skip_serializing_if = "Option::is_none")]
    pub min_tree_child_weight:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::IntHparamSearchSpace>>,
    #[serde(rename = "numClusters", skip_serializing_if = "Option::is_none")]
    pub num_clusters:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::IntHparamSearchSpace>>,
    #[serde(rename = "numFactors", skip_serializing_if = "Option::is_none")]
    pub num_factors:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::IntHparamSearchSpace>>,
    #[serde(rename = "numParallelTree", skip_serializing_if = "Option::is_none")]
    pub num_parallel_tree:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::IntHparamSearchSpace>>,
    #[serde(rename = "optimizer", skip_serializing_if = "Option::is_none")]
    pub optimizer:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::StringHparamSearchSpace>>,
    #[serde(rename = "subsample", skip_serializing_if = "Option::is_none")]
    pub subsample:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleHparamSearchSpace>>,
    #[serde(rename = "treeMethod", skip_serializing_if = "Option::is_none")]
    pub tree_method:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::StringHparamSearchSpace>>,
    #[serde(rename = "walsAlpha", skip_serializing_if = "Option::is_none")]
    pub wals_alpha:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleHparamSearchSpace>>,
}

impl HparamSearchSpaces {
    /// Hyperparameter search spaces. These should be a subset of training_options.
    pub fn new() -> HparamSearchSpaces {
        HparamSearchSpaces {
            activation_fn: None,
            batch_size: None,
            booster_type: None,
            colsample_bylevel: None,
            colsample_bynode: None,
            colsample_bytree: None,
            dart_normalize_type: None,
            dropout: None,
            hidden_units: None,
            l1_reg: None,
            l2_reg: None,
            learn_rate: None,
            max_tree_depth: None,
            min_split_loss: None,
            min_tree_child_weight: None,
            num_clusters: None,
            num_factors: None,
            num_parallel_tree: None,
            optimizer: None,
            subsample: None,
            tree_method: None,
            wals_alpha: None,
        }
    }
}
