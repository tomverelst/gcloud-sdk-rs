/*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::storage_v1::models;
use serde::{Deserialize, Serialize};

/// Channel : An notification channel used to watch for resource changes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    /// The address where notifications are delivered for this channel.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// A UUID or similar unique string that identifies this channel.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Additional parameters controlling delivery channel behavior. Optional.
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::HashMap<String, String>>,
    /// A Boolean value to indicate whether payload is wanted. Optional.
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<bool>,
    /// An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// A version-specific identifier for the watched resource.
    #[serde(rename = "resourceUri", skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    /// An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The type of delivery mechanism used for this channel.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl Channel {
    /// An notification channel used to watch for resource changes.
    pub fn new() -> Channel {
        Channel {
            address: None,
            expiration: None,
            id: None,
            kind: None,
            params: None,
            payload: None,
            resource_id: None,
            resource_uri: None,
            token: None,
            r#type: None,
        }
    }
}
