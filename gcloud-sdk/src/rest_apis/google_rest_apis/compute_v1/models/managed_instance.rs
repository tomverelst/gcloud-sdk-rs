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

/// ManagedInstance : A Managed Instance resource.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedInstance {
    /// [Output Only] The current action that the managed instance group has scheduled for the instance. Possible values: - NONE The instance is running, and the managed instance group does not have any scheduled actions for this instance. - CREATING The managed instance group is creating this instance. If the group fails to create this instance, it will try again until it is successful. - CREATING_WITHOUT_RETRIES The managed instance group is attempting to create this instance only once. If the group fails to create this instance, it does not try again and the group's targetSize value is decreased instead. - RECREATING The managed instance group is recreating this instance. - DELETING The managed instance group is permanently deleting this instance. - ABANDONING The managed instance group is abandoning this instance. The instance will be removed from the instance group and from any target pools that are associated with this group. - RESTARTING The managed instance group is restarting the instance. - REFRESHING The managed instance group is applying configuration changes to the instance without stopping it. For example, the group can update the target pool list for an instance without stopping that instance. - VERIFYING The managed instance group has created the instance and it is in the process of being verified.
    #[serde(rename = "currentAction", skip_serializing_if = "Option::is_none")]
    pub current_action: Option<CurrentAction>,
    /// [Output only] The unique identifier for this resource. This field is empty when instance does not exist.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] The URL of the instance. The URL can exist even if the instance has not yet been created.
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// [Output Only] Health state of the instance per health-check.
    #[serde(rename = "instanceHealth", skip_serializing_if = "Option::is_none")]
    pub instance_health: Option<Vec<models::ManagedInstanceInstanceHealth>>,
    /// [Output Only] The status of the instance. This field is empty when the instance does not exist.
    #[serde(rename = "instanceStatus", skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<InstanceStatus>,
    #[serde(rename = "lastAttempt", skip_serializing_if = "Option::is_none")]
    pub last_attempt: Option<Box<models::ManagedInstanceLastAttempt>>,
    /// [Output Only] The name of the instance. The name always exists even if the instance has not yet been created.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "preservedStateFromConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub preserved_state_from_config: Option<Box<models::PreservedState>>,
    #[serde(
        rename = "preservedStateFromPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub preserved_state_from_policy: Option<Box<models::PreservedState>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::ManagedInstanceVersion>>,
}

impl ManagedInstance {
    /// A Managed Instance resource.
    pub fn new() -> ManagedInstance {
        ManagedInstance {
            current_action: None,
            id: None,
            instance: None,
            instance_health: None,
            instance_status: None,
            last_attempt: None,
            name: None,
            preserved_state_from_config: None,
            preserved_state_from_policy: None,
            version: None,
        }
    }
}
/// [Output Only] The current action that the managed instance group has scheduled for the instance. Possible values: - NONE The instance is running, and the managed instance group does not have any scheduled actions for this instance. - CREATING The managed instance group is creating this instance. If the group fails to create this instance, it will try again until it is successful. - CREATING_WITHOUT_RETRIES The managed instance group is attempting to create this instance only once. If the group fails to create this instance, it does not try again and the group's targetSize value is decreased instead. - RECREATING The managed instance group is recreating this instance. - DELETING The managed instance group is permanently deleting this instance. - ABANDONING The managed instance group is abandoning this instance. The instance will be removed from the instance group and from any target pools that are associated with this group. - RESTARTING The managed instance group is restarting the instance. - REFRESHING The managed instance group is applying configuration changes to the instance without stopping it. For example, the group can update the target pool list for an instance without stopping that instance. - VERIFYING The managed instance group has created the instance and it is in the process of being verified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CurrentAction {
    #[serde(rename = "ABANDONING")]
    Abandoning,
    #[serde(rename = "CREATING")]
    Creating,
    #[serde(rename = "CREATING_WITHOUT_RETRIES")]
    CreatingWithoutRetries,
    #[serde(rename = "DELETING")]
    Deleting,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "RECREATING")]
    Recreating,
    #[serde(rename = "REFRESHING")]
    Refreshing,
    #[serde(rename = "RESTARTING")]
    Restarting,
    #[serde(rename = "RESUMING")]
    Resuming,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "STOPPING")]
    Stopping,
    #[serde(rename = "SUSPENDING")]
    Suspending,
    #[serde(rename = "VERIFYING")]
    Verifying,
}

impl Default for CurrentAction {
    fn default() -> CurrentAction {
        Self::Abandoning
    }
}
/// [Output Only] The status of the instance. This field is empty when the instance does not exist.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceStatus {
    #[serde(rename = "DEPROVISIONING")]
    Deprovisioning,
    #[serde(rename = "PROVISIONING")]
    Provisioning,
    #[serde(rename = "REPAIRING")]
    Repairing,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STAGING")]
    Staging,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "STOPPING")]
    Stopping,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "SUSPENDING")]
    Suspending,
    #[serde(rename = "TERMINATED")]
    Terminated,
}

impl Default for InstanceStatus {
    fn default() -> InstanceStatus {
        Self::Deprovisioning
    }
}
