use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstanceGroupsRemoveInstancesRequest {
    /// The list of instances to remove from the instance group.
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<crate::google_rest_apis::compute_v1::models::InstanceReference>>,
}

impl InstanceGroupsRemoveInstancesRequest {
    pub fn new() -> InstanceGroupsRemoveInstancesRequest {
        InstanceGroupsRemoveInstancesRequest { instances: None }
    }
}
