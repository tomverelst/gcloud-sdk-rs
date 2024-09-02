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

/// BackupConfiguration : Database instance backup configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupConfiguration {
    #[serde(
        rename = "backupRetentionSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_retention_settings: Option<Box<models::BackupRetentionSettings>>,
    /// (MySQL only) Whether binary log is enabled. If backup configuration is disabled, binarylog must be disabled as well.
    #[serde(rename = "binaryLogEnabled", skip_serializing_if = "Option::is_none")]
    pub binary_log_enabled: Option<bool>,
    /// Whether this configuration is enabled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// This is always `sql#backupConfiguration`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Location of the backup
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// (Postgres only) Whether point in time recovery is enabled.
    #[serde(
        rename = "pointInTimeRecoveryEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub point_in_time_recovery_enabled: Option<bool>,
    /// Reserved for future use.
    #[serde(
        rename = "replicationLogArchivingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub replication_log_archiving_enabled: Option<bool>,
    /// Start time for the daily backup configuration in UTC timezone in the 24 hour format - `HH:MM`.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The number of days of transaction logs we retain for point in time restore, from 1-7.
    #[serde(
        rename = "transactionLogRetentionDays",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_log_retention_days: Option<i32>,
}

impl BackupConfiguration {
    /// Database instance backup configuration.
    pub fn new() -> BackupConfiguration {
        BackupConfiguration {
            backup_retention_settings: None,
            binary_log_enabled: None,
            enabled: None,
            kind: None,
            location: None,
            point_in_time_recovery_enabled: None,
            replication_log_archiving_enabled: None,
            start_time: None,
            transaction_log_retention_days: None,
        }
    }
}
