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
pub struct InstanceGroupManagerStatusStateful {
    /// [Output Only] A bit indicating whether the managed instance group has stateful configuration, that is, if you have configured any items in a stateful policy or in per-instance configs. The group might report that it has no stateful configuration even when there is still some preserved state on a managed instance, for example, if you have deleted all PICs but not yet applied those deletions.
    #[serde(rename = "hasStatefulConfig", skip_serializing_if = "Option::is_none")]
    pub has_stateful_config: Option<bool>,
    #[serde(rename = "perInstanceConfigs", skip_serializing_if = "Option::is_none")]
    pub per_instance_configs:
        Option<Box<models::InstanceGroupManagerStatusStatefulPerInstanceConfigs>>,
}

impl InstanceGroupManagerStatusStateful {
    pub fn new() -> InstanceGroupManagerStatusStateful {
        InstanceGroupManagerStatusStateful {
            has_stateful_config: None,
            per_instance_configs: None,
        }
    }
}
