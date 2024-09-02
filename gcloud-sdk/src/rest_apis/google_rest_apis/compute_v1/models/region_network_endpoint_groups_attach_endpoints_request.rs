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
pub struct RegionNetworkEndpointGroupsAttachEndpointsRequest {
    /// The list of network endpoints to be attached.
    #[serde(rename = "networkEndpoints", skip_serializing_if = "Option::is_none")]
    pub network_endpoints:
        Option<Vec<crate::google_rest_apis::compute_v1::models::NetworkEndpoint>>,
}

impl RegionNetworkEndpointGroupsAttachEndpointsRequest {
    pub fn new() -> RegionNetworkEndpointGroupsAttachEndpointsRequest {
        RegionNetworkEndpointGroupsAttachEndpointsRequest {
            network_endpoints: None,
        }
    }
}
