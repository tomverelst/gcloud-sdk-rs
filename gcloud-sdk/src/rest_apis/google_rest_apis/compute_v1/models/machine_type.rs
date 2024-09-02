/*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::compute_v1::models;
use serde::{Deserialize, Serialize};

/// MachineType : Represents a Machine Type resource. You can use specific machine types for your VM instances based on performance and pricing requirements. For more information, read Machine Types.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineType {
    /// [Output Only] A list of accelerator configurations assigned to this machine type.
    #[serde(rename = "accelerators", skip_serializing_if = "Option::is_none")]
    pub accelerators: Option<Vec<models::MachineTypeAcceleratorsInner>>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<Box<models::DeprecationStatus>>,
    /// [Output Only] An optional textual description of the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] The number of virtual CPUs that are available to the instance.
    #[serde(rename = "guestCpus", skip_serializing_if = "Option::is_none")]
    pub guest_cpus: Option<i32>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Deprecated] This property is deprecated and will never be populated with any relevant values.
    #[serde(rename = "imageSpaceGb", skip_serializing_if = "Option::is_none")]
    pub image_space_gb: Option<i32>,
    /// [Output Only] Whether this machine type has a shared CPU. See Shared-core machine types for more information.
    #[serde(rename = "isSharedCpu", skip_serializing_if = "Option::is_none")]
    pub is_shared_cpu: Option<bool>,
    /// [Output Only] The type of the resource. Always compute#machineType for machine types.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output Only] Maximum persistent disks allowed.
    #[serde(
        rename = "maximumPersistentDisks",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_persistent_disks: Option<i32>,
    /// [Output Only] Maximum total persistent disks size (GB) allowed.
    #[serde(
        rename = "maximumPersistentDisksSizeGb",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_persistent_disks_size_gb: Option<String>,
    /// [Output Only] The amount of physical memory available to the instance, defined in MB.
    #[serde(rename = "memoryMb", skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i32>,
    /// [Output Only] Name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] A list of extended scratch disks assigned to the instance.
    #[serde(rename = "scratchDisks", skip_serializing_if = "Option::is_none")]
    pub scratch_disks: Option<Vec<models::MachineTypeScratchDisksInner>>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output Only] The name of the zone where the machine type resides, such as us-central1-a.
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl MachineType {
    /// Represents a Machine Type resource. You can use specific machine types for your VM instances based on performance and pricing requirements. For more information, read Machine Types.
    pub fn new() -> MachineType {
        MachineType {
            accelerators: None,
            creation_timestamp: None,
            deprecated: None,
            description: None,
            guest_cpus: None,
            id: None,
            image_space_gb: None,
            is_shared_cpu: None,
            kind: None,
            maximum_persistent_disks: None,
            maximum_persistent_disks_size_gb: None,
            memory_mb: None,
            name: None,
            scratch_disks: None,
            self_link: None,
            zone: None,
        }
    }
}
