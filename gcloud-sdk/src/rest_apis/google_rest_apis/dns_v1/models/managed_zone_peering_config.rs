/*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::dns_v1::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedZonePeeringConfig {
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "targetNetwork", skip_serializing_if = "Option::is_none")]
    pub target_network: Option<Box<models::ManagedZonePeeringConfigTargetNetwork>>,
}

impl ManagedZonePeeringConfig {
    pub fn new() -> ManagedZonePeeringConfig {
        ManagedZonePeeringConfig {
            kind: None,
            target_network: None,
        }
    }
}
