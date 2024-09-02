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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatistics2ReservationUsageInner {
    /// [Output only] Reservation name or \"unreserved\" for on-demand resources usage.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output only] Slot-milliseconds the job spent in the given reservation.
    #[serde(rename = "slotMs", skip_serializing_if = "Option::is_none")]
    pub slot_ms: Option<String>,
}

impl JobStatistics2ReservationUsageInner {
    pub fn new() -> JobStatistics2ReservationUsageInner {
        JobStatistics2ReservationUsageInner {
            name: None,
            slot_ms: None,
        }
    }
}
