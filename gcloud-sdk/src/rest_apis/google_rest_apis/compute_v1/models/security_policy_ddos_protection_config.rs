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
pub struct SecurityPolicyDdosProtectionConfig {
    #[serde(rename = "ddosProtection", skip_serializing_if = "Option::is_none")]
    pub ddos_protection: Option<DdosProtection>,
}

impl SecurityPolicyDdosProtectionConfig {
    pub fn new() -> SecurityPolicyDdosProtectionConfig {
        SecurityPolicyDdosProtectionConfig {
            ddos_protection: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DdosProtection {
    #[serde(rename = "ADVANCED")]
    Advanced,
    #[serde(rename = "STANDARD")]
    Standard,
}

impl Default for DdosProtection {
    fn default() -> DdosProtection {
        Self::Advanced
    }
}
