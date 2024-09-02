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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionInstanceGroupsListInstancesRequest {
    /// Instances in which state should be returned. Valid options are: 'ALL', 'RUNNING'. By default, it lists all instances.
    #[serde(rename = "instanceState", skip_serializing_if = "Option::is_none")]
    pub instance_state: Option<InstanceState>,
    /// Name of port user is interested in. It is optional. If it is set, only information about this ports will be returned. If it is not set, all the named ports will be returned. Always lists all instances.
    #[serde(rename = "portName", skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

impl RegionInstanceGroupsListInstancesRequest {
    pub fn new() -> RegionInstanceGroupsListInstancesRequest {
        RegionInstanceGroupsListInstancesRequest {
            instance_state: None,
            port_name: None,
        }
    }
}
/// Instances in which state should be returned. Valid options are: 'ALL', 'RUNNING'. By default, it lists all instances.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceState {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "RUNNING")]
    Running,
}

impl Default for InstanceState {
    fn default() -> InstanceState {
        Self::All
    }
}
