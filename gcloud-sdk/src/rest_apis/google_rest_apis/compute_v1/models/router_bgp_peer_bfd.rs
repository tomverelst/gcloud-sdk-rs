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
pub struct RouterBgpPeerBfd {
    /// The minimum interval, in milliseconds, between BFD control packets received from the peer router. The actual value is negotiated between the two routers and is equal to the greater of this value and the transmit interval of the other router. If set, this value must be between 1000 and 30000. The default is 1000.
    #[serde(rename = "minReceiveInterval", skip_serializing_if = "Option::is_none")]
    pub min_receive_interval: Option<i32>,
    /// The minimum interval, in milliseconds, between BFD control packets transmitted to the peer router. The actual value is negotiated between the two routers and is equal to the greater of this value and the corresponding receive interval of the other router. If set, this value must be between 1000 and 30000. The default is 1000.
    #[serde(
        rename = "minTransmitInterval",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_transmit_interval: Option<i32>,
    /// The number of consecutive BFD packets that must be missed before BFD declares that a peer is unavailable. If set, the value must be a value between 5 and 16. The default is 5.
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<i32>,
    /// The BFD session initialization mode for this BGP peer. If set to ACTIVE, the Cloud Router will initiate the BFD session for this BGP peer. If set to PASSIVE, the Cloud Router will wait for the peer router to initiate the BFD session for this BGP peer. If set to DISABLED, BFD is disabled for this BGP peer. The default is DISABLED.
    #[serde(
        rename = "sessionInitializationMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_initialization_mode: Option<SessionInitializationMode>,
}

impl RouterBgpPeerBfd {
    pub fn new() -> RouterBgpPeerBfd {
        RouterBgpPeerBfd {
            min_receive_interval: None,
            min_transmit_interval: None,
            multiplier: None,
            session_initialization_mode: None,
        }
    }
}
/// The BFD session initialization mode for this BGP peer. If set to ACTIVE, the Cloud Router will initiate the BFD session for this BGP peer. If set to PASSIVE, the Cloud Router will wait for the peer router to initiate the BFD session for this BGP peer. If set to DISABLED, BFD is disabled for this BGP peer. The default is DISABLED.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SessionInitializationMode {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "PASSIVE")]
    Passive,
}

impl Default for SessionInitializationMode {
    fn default() -> SessionInitializationMode {
        Self::Active
    }
}
