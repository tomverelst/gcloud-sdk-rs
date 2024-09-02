/*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::storage_v1::models;
use serde::{Deserialize, Serialize};

/// BucketLifecycle : The bucket's lifecycle configuration. See lifecycle management for more information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BucketLifecycle {
    /// A lifecycle management rule, which is made of an action to take and the condition(s) under which the action will be taken.
    #[serde(rename = "rule", skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<models::BucketLifecycleRuleInner>>,
}

impl BucketLifecycle {
    /// The bucket's lifecycle configuration. See lifecycle management for more information.
    pub fn new() -> BucketLifecycle {
        BucketLifecycle { rule: None }
    }
}
