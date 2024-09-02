use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Subsetting : Subsetting configuration for this BackendService. Currently this is applicable only for Internal TCP/UDP load balancing, Internal HTTP(S) load balancing and Traffic Director.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Subsetting {
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

impl Subsetting {
    /// Subsetting configuration for this BackendService. Currently this is applicable only for Internal TCP/UDP load balancing, Internal HTTP(S) load balancing and Traffic Director.
    pub fn new() -> Subsetting {
        Subsetting { policy: None }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Policy {
    #[serde(rename = "CONSISTENT_HASH_SUBSETTING")]
    ConsistentHashSubsetting,
    #[serde(rename = "NONE")]
    None,
}

impl Default for Policy {
    fn default() -> Policy {
        Self::ConsistentHashSubsetting
    }
}
