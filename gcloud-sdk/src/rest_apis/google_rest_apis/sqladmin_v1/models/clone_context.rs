/*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::sqladmin_v1::models;
use serde::{Deserialize, Serialize};

/// CloneContext : Database instance clone context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloneContext {
    /// The name of the allocated ip range for the private ip Cloud SQL instance. For example: \"google-managed-services-default\". If set, the cloned instance ip will be created in the allocated range. The range name must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035). Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?. Reserved for future use.
    #[serde(rename = "allocatedIpRange", skip_serializing_if = "Option::is_none")]
    pub allocated_ip_range: Option<String>,
    #[serde(rename = "binLogCoordinates", skip_serializing_if = "Option::is_none")]
    pub bin_log_coordinates: Option<Box<models::BinLogCoordinates>>,
    /// (SQL Server only) Clone only the specified databases from the source instance. Clone all databases if empty.
    #[serde(rename = "databaseNames", skip_serializing_if = "Option::is_none")]
    pub database_names: Option<Vec<String>>,
    /// Name of the Cloud SQL instance to be created as a clone.
    #[serde(
        rename = "destinationInstanceName",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_instance_name: Option<String>,
    /// This is always `sql#cloneContext`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Reserved for future use.
    #[serde(rename = "pitrTimestampMs", skip_serializing_if = "Option::is_none")]
    pub pitr_timestamp_ms: Option<String>,
    /// Timestamp, if specified, identifies the time to which the source instance is cloned.
    #[serde(rename = "pointInTime", skip_serializing_if = "Option::is_none")]
    pub point_in_time: Option<String>,
}

impl CloneContext {
    /// Database instance clone context.
    pub fn new() -> CloneContext {
        CloneContext {
            allocated_ip_range: None,
            bin_log_coordinates: None,
            database_names: None,
            destination_instance_name: None,
            kind: None,
            pitr_timestamp_ms: None,
            point_in_time: None,
        }
    }
}
