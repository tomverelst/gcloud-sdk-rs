use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// DoubleHparamSearchSpace : Search space for a double hyperparameter.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DoubleHparamSearchSpace {
    #[serde(rename = "candidates", skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleCandidates>>,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<crate::google_rest_apis::bigquery_v2::models::DoubleRange>>,
}

impl DoubleHparamSearchSpace {
    /// Search space for a double hyperparameter.
    pub fn new() -> DoubleHparamSearchSpace {
        DoubleHparamSearchSpace {
            candidates: None,
            range: None,
        }
    }
}
