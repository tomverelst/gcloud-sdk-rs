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

/// Settings : Database instance settings.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    /// The activation policy specifies when the instance is activated; it is applicable only when the instance state is RUNNABLE. Valid values: * `ALWAYS`: The instance is on, and remains so even in the absence of connection requests. * `NEVER`: The instance is off; it is not activated, even if a connection request arrives.
    #[serde(rename = "activationPolicy", skip_serializing_if = "Option::is_none")]
    pub activation_policy: Option<ActivationPolicy>,
    #[serde(
        rename = "activeDirectoryConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_directory_config: Option<Box<models::SqlActiveDirectoryConfig>>,
    /// The App Engine app IDs that can access this instance. (Deprecated) Applied to First Generation instances only.
    #[serde(
        rename = "authorizedGaeApplications",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorized_gae_applications: Option<Vec<String>>,
    /// Availability type. Potential values: * `ZONAL`: The instance serves data from only one zone. Outages in that zone affect data accessibility. * `REGIONAL`: The instance can serve data from more than one zone in a region (it is highly available)./ For more information, see [Overview of the High Availability Configuration](https://cloud.google.com/sql/docs/mysql/high-availability).
    #[serde(rename = "availabilityType", skip_serializing_if = "Option::is_none")]
    pub availability_type: Option<AvailabilityType>,
    #[serde(
        rename = "backupConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_configuration: Option<Box<models::BackupConfiguration>>,
    /// The name of server Instance collation.
    #[serde(rename = "collation", skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    /// Specifies if connections must use Cloud SQL connectors. Option values include the following: `NOT_REQUIRED` (Cloud SQL instances can be connected without Cloud SQL Connectors) and `REQUIRED` (Only allow connections that use Cloud SQL Connectors). Note that using REQUIRED disables all existing authorized networks. If this field is not specified when creating a new instance, NOT_REQUIRED is used. If this field is not specified when patching or updating an existing instance, it is left unchanged in the instance.
    #[serde(
        rename = "connectorEnforcement",
        skip_serializing_if = "Option::is_none"
    )]
    pub connector_enforcement: Option<ConnectorEnforcement>,
    /// Configuration specific to read replica instances. Indicates whether database flags for crash-safe replication are enabled. This property was only applicable to First Generation instances.
    #[serde(
        rename = "crashSafeReplicationEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub crash_safe_replication_enabled: Option<bool>,
    /// The size of data disk, in GB. The data disk size minimum is 10GB.
    #[serde(rename = "dataDiskSizeGb", skip_serializing_if = "Option::is_none")]
    pub data_disk_size_gb: Option<String>,
    /// The type of data disk: `PD_SSD` (default) or `PD_HDD`. Not used for First Generation instances.
    #[serde(rename = "dataDiskType", skip_serializing_if = "Option::is_none")]
    pub data_disk_type: Option<DataDiskType>,
    /// The database flags passed to the instance at startup.
    #[serde(rename = "databaseFlags", skip_serializing_if = "Option::is_none")]
    pub database_flags: Option<Vec<models::DatabaseFlags>>,
    /// Configuration specific to read replica instances. Indicates whether replication is enabled or not. WARNING: Changing this restarts the instance.
    #[serde(
        rename = "databaseReplicationEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub database_replication_enabled: Option<bool>,
    /// Configuration to protect against accidental instance deletion.
    #[serde(
        rename = "deletionProtectionEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub deletion_protection_enabled: Option<bool>,
    /// Deny maintenance periods
    #[serde(
        rename = "denyMaintenancePeriods",
        skip_serializing_if = "Option::is_none"
    )]
    pub deny_maintenance_periods: Option<Vec<models::DenyMaintenancePeriod>>,
    #[serde(rename = "insightsConfig", skip_serializing_if = "Option::is_none")]
    pub insights_config: Option<Box<models::InsightsConfig>>,
    #[serde(rename = "ipConfiguration", skip_serializing_if = "Option::is_none")]
    pub ip_configuration: Option<Box<models::IpConfiguration>>,
    /// This is always `sql#settings`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "locationPreference", skip_serializing_if = "Option::is_none")]
    pub location_preference: Option<Box<models::LocationPreference>>,
    #[serde(rename = "maintenanceWindow", skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<Box<models::MaintenanceWindow>>,
    #[serde(
        rename = "passwordValidationPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub password_validation_policy: Option<Box<models::PasswordValidationPolicy>>,
    /// The pricing plan for this instance. This can be either `PER_USE` or `PACKAGE`. Only `PER_USE` is supported for Second Generation instances.
    #[serde(rename = "pricingPlan", skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<PricingPlan>,
    /// The type of replication this instance uses. This can be either `ASYNCHRONOUS` or `SYNCHRONOUS`. (Deprecated) This property was only applicable to First Generation instances.
    #[serde(rename = "replicationType", skip_serializing_if = "Option::is_none")]
    pub replication_type: Option<ReplicationType>,
    /// The version of instance settings. This is a required field for update method to make sure concurrent updates are handled properly. During update, use the most recent settingsVersion value for this instance and do not try to update this value.
    #[serde(rename = "settingsVersion", skip_serializing_if = "Option::is_none")]
    pub settings_version: Option<String>,
    #[serde(
        rename = "sqlServerAuditConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub sql_server_audit_config: Option<Box<models::SqlServerAuditConfig>>,
    /// Configuration to increase storage size automatically. The default value is true.
    #[serde(rename = "storageAutoResize", skip_serializing_if = "Option::is_none")]
    pub storage_auto_resize: Option<bool>,
    /// The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[serde(
        rename = "storageAutoResizeLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_auto_resize_limit: Option<String>,
    /// The tier (or machine type) for this instance, for example `db-custom-1-3840`. WARNING: Changing this restarts the instance.
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Server timezone, relevant only for Cloud SQL for SQL Server.
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// User-provided labels, represented as a dictionary where each label is a single key value pair.
    #[serde(rename = "userLabels", skip_serializing_if = "Option::is_none")]
    pub user_labels: Option<std::collections::HashMap<String, String>>,
}

impl Settings {
    /// Database instance settings.
    pub fn new() -> Settings {
        Settings {
            activation_policy: None,
            active_directory_config: None,
            authorized_gae_applications: None,
            availability_type: None,
            backup_configuration: None,
            collation: None,
            connector_enforcement: None,
            crash_safe_replication_enabled: None,
            data_disk_size_gb: None,
            data_disk_type: None,
            database_flags: None,
            database_replication_enabled: None,
            deletion_protection_enabled: None,
            deny_maintenance_periods: None,
            insights_config: None,
            ip_configuration: None,
            kind: None,
            location_preference: None,
            maintenance_window: None,
            password_validation_policy: None,
            pricing_plan: None,
            replication_type: None,
            settings_version: None,
            sql_server_audit_config: None,
            storage_auto_resize: None,
            storage_auto_resize_limit: None,
            tier: None,
            time_zone: None,
            user_labels: None,
        }
    }
}
/// The activation policy specifies when the instance is activated; it is applicable only when the instance state is RUNNABLE. Valid values: * `ALWAYS`: The instance is on, and remains so even in the absence of connection requests. * `NEVER`: The instance is off; it is not activated, even if a connection request arrives.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActivationPolicy {
    #[serde(rename = "SQL_ACTIVATION_POLICY_UNSPECIFIED")]
    SqlActivationPolicyUnspecified,
    #[serde(rename = "ALWAYS")]
    Always,
    #[serde(rename = "NEVER")]
    Never,
    #[serde(rename = "ON_DEMAND")]
    OnDemand,
}

impl Default for ActivationPolicy {
    fn default() -> ActivationPolicy {
        Self::SqlActivationPolicyUnspecified
    }
}
/// Availability type. Potential values: * `ZONAL`: The instance serves data from only one zone. Outages in that zone affect data accessibility. * `REGIONAL`: The instance can serve data from more than one zone in a region (it is highly available)./ For more information, see [Overview of the High Availability Configuration](https://cloud.google.com/sql/docs/mysql/high-availability).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AvailabilityType {
    #[serde(rename = "SQL_AVAILABILITY_TYPE_UNSPECIFIED")]
    SqlAvailabilityTypeUnspecified,
    #[serde(rename = "ZONAL")]
    Zonal,
    #[serde(rename = "REGIONAL")]
    Regional,
}

impl Default for AvailabilityType {
    fn default() -> AvailabilityType {
        Self::SqlAvailabilityTypeUnspecified
    }
}
/// Specifies if connections must use Cloud SQL connectors. Option values include the following: `NOT_REQUIRED` (Cloud SQL instances can be connected without Cloud SQL Connectors) and `REQUIRED` (Only allow connections that use Cloud SQL Connectors). Note that using REQUIRED disables all existing authorized networks. If this field is not specified when creating a new instance, NOT_REQUIRED is used. If this field is not specified when patching or updating an existing instance, it is left unchanged in the instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectorEnforcement {
    #[serde(rename = "CONNECTOR_ENFORCEMENT_UNSPECIFIED")]
    ConnectorEnforcementUnspecified,
    #[serde(rename = "NOT_REQUIRED")]
    NotRequired,
    #[serde(rename = "REQUIRED")]
    Required,
}

impl Default for ConnectorEnforcement {
    fn default() -> ConnectorEnforcement {
        Self::ConnectorEnforcementUnspecified
    }
}
/// The type of data disk: `PD_SSD` (default) or `PD_HDD`. Not used for First Generation instances.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataDiskType {
    #[serde(rename = "SQL_DATA_DISK_TYPE_UNSPECIFIED")]
    SqlDataDiskTypeUnspecified,
    #[serde(rename = "PD_SSD")]
    PdSsd,
    #[serde(rename = "PD_HDD")]
    PdHdd,
    #[serde(rename = "OBSOLETE_LOCAL_SSD")]
    ObsoleteLocalSsd,
}

impl Default for DataDiskType {
    fn default() -> DataDiskType {
        Self::SqlDataDiskTypeUnspecified
    }
}
/// The pricing plan for this instance. This can be either `PER_USE` or `PACKAGE`. Only `PER_USE` is supported for Second Generation instances.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PricingPlan {
    #[serde(rename = "SQL_PRICING_PLAN_UNSPECIFIED")]
    SqlPricingPlanUnspecified,
    #[serde(rename = "PACKAGE")]
    Package,
    #[serde(rename = "PER_USE")]
    PerUse,
}

impl Default for PricingPlan {
    fn default() -> PricingPlan {
        Self::SqlPricingPlanUnspecified
    }
}
/// The type of replication this instance uses. This can be either `ASYNCHRONOUS` or `SYNCHRONOUS`. (Deprecated) This property was only applicable to First Generation instances.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReplicationType {
    #[serde(rename = "SQL_REPLICATION_TYPE_UNSPECIFIED")]
    SqlReplicationTypeUnspecified,
    #[serde(rename = "SYNCHRONOUS")]
    Synchronous,
    #[serde(rename = "ASYNCHRONOUS")]
    Asynchronous,
}

impl Default for ReplicationType {
    fn default() -> ReplicationType {
        Self::SqlReplicationTypeUnspecified
    }
}
