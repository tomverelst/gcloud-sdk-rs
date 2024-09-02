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

use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaterializedViewDefinition {
    /// [Optional] [TrustedTester] Enable automatic refresh of the materialized view when the base table is updated. The default value is \"true\".
    #[serde(rename = "enableRefresh", skip_serializing_if = "Option::is_none")]
    pub enable_refresh: Option<bool>,
    /// [Output-only] [TrustedTester] The time when this materialized view was last modified, in milliseconds since the epoch.
    #[serde(rename = "lastRefreshTime", skip_serializing_if = "Option::is_none")]
    pub last_refresh_time: Option<String>,
    /// [Optional] Max staleness of data that could be returned when materizlized view is queried (formatted as Google SQL Interval type).
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "maxStaleness", skip_serializing_if = "Option::is_none")]
    pub max_staleness: Option<Vec<u8>>,
    /// [Required] A query whose result is persisted.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// [Optional] [TrustedTester] The maximum frequency at which this materialized view will be refreshed. The default value is \"1800000\" (30 minutes).
    #[serde(rename = "refreshIntervalMs", skip_serializing_if = "Option::is_none")]
    pub refresh_interval_ms: Option<String>,
}

impl MaterializedViewDefinition {
    pub fn new() -> MaterializedViewDefinition {
        MaterializedViewDefinition {
            enable_refresh: None,
            last_refresh_time: None,
            max_staleness: None,
            query: None,
            refresh_interval_ms: None,
        }
    }
}
