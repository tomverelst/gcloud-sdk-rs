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

/// InstanceGroupManagersApplyUpdatesRequest : InstanceGroupManagers.applyUpdatesToInstances
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceGroupManagersApplyUpdatesRequest {
    /// Flag to update all instances instead of specified list of “instances”. If the flag is set to true then the instances may not be specified in the request.
    #[serde(rename = "allInstances", skip_serializing_if = "Option::is_none")]
    pub all_instances: Option<bool>,
    /// The list of URLs of one or more instances for which you want to apply updates. Each URL can be a full URL or a partial URL, such as zones/[ZONE]/instances/[INSTANCE_NAME].
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
    /// The minimal action that you want to perform on each instance during the update: - REPLACE: At minimum, delete the instance and create it again. - RESTART: Stop the instance and start it again. - REFRESH: Do not stop the instance and limit disruption as much as possible. - NONE: Do not disrupt the instance at all. By default, the minimum action is NONE. If your update requires a more disruptive action than you set with this flag, the necessary action is performed to execute the update.
    #[serde(rename = "minimalAction", skip_serializing_if = "Option::is_none")]
    pub minimal_action: Option<MinimalAction>,
    /// The most disruptive action that you want to perform on each instance during the update: - REPLACE: Delete the instance and create it again. - RESTART: Stop the instance and start it again. - REFRESH: Do not stop the instance and limit disruption as much as possible. - NONE: Do not disrupt the instance at all. By default, the most disruptive allowed action is REPLACE. If your update requires a more disruptive action than you set with this flag, the update request will fail.
    #[serde(
        rename = "mostDisruptiveAllowedAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub most_disruptive_allowed_action: Option<MostDisruptiveAllowedAction>,
}

impl InstanceGroupManagersApplyUpdatesRequest {
    /// InstanceGroupManagers.applyUpdatesToInstances
    pub fn new() -> InstanceGroupManagersApplyUpdatesRequest {
        InstanceGroupManagersApplyUpdatesRequest {
            all_instances: None,
            instances: None,
            minimal_action: None,
            most_disruptive_allowed_action: None,
        }
    }
}
/// The minimal action that you want to perform on each instance during the update: - REPLACE: At minimum, delete the instance and create it again. - RESTART: Stop the instance and start it again. - REFRESH: Do not stop the instance and limit disruption as much as possible. - NONE: Do not disrupt the instance at all. By default, the minimum action is NONE. If your update requires a more disruptive action than you set with this flag, the necessary action is performed to execute the update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MinimalAction {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "REFRESH")]
    Refresh,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "RESTART")]
    Restart,
}

impl Default for MinimalAction {
    fn default() -> MinimalAction {
        Self::None
    }
}
/// The most disruptive action that you want to perform on each instance during the update: - REPLACE: Delete the instance and create it again. - RESTART: Stop the instance and start it again. - REFRESH: Do not stop the instance and limit disruption as much as possible. - NONE: Do not disrupt the instance at all. By default, the most disruptive allowed action is REPLACE. If your update requires a more disruptive action than you set with this flag, the update request will fail.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MostDisruptiveAllowedAction {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "REFRESH")]
    Refresh,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "RESTART")]
    Restart,
}

impl Default for MostDisruptiveAllowedAction {
    fn default() -> MostDisruptiveAllowedAction {
        Self::None
    }
}
