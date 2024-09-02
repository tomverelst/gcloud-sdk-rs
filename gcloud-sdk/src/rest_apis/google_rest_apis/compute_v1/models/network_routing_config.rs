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

/// NetworkRoutingConfig : A routing configuration attached to a network resource. The message includes the list of routers associated with the network, and a flag indicating the type of routing behavior to enforce network-wide.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRoutingConfig {
    /// The network-wide routing mode to use. If set to REGIONAL, this network's Cloud Routers will only advertise routes with subnets of this network in the same region as the router. If set to GLOBAL, this network's Cloud Routers will advertise routes with all subnets of this network, across regions.
    #[serde(rename = "routingMode", skip_serializing_if = "Option::is_none")]
    pub routing_mode: Option<RoutingMode>,
}

impl NetworkRoutingConfig {
    /// A routing configuration attached to a network resource. The message includes the list of routers associated with the network, and a flag indicating the type of routing behavior to enforce network-wide.
    pub fn new() -> NetworkRoutingConfig {
        NetworkRoutingConfig { routing_mode: None }
    }
}
/// The network-wide routing mode to use. If set to REGIONAL, this network's Cloud Routers will only advertise routes with subnets of this network in the same region as the router. If set to GLOBAL, this network's Cloud Routers will advertise routes with all subnets of this network, across regions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoutingMode {
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "REGIONAL")]
    Regional,
}

impl Default for RoutingMode {
    fn default() -> RoutingMode {
        Self::Global
    }
}
