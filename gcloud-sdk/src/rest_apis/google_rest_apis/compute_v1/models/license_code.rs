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

/// LicenseCode : Represents a License Code resource. A License Code is a unique identifier used to represent a license resource. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseCode {
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// [Output Only] Description of this License Code.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of resource. Always compute#licenseCode for licenses.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output Only] URL and description aliases of Licenses with the same License Code.
    #[serde(rename = "licenseAlias", skip_serializing_if = "Option::is_none")]
    pub license_alias: Option<Vec<models::LicenseCodeLicenseAlias>>,
    /// [Output Only] Name of the resource. The name is 1-20 characters long and must be a valid 64 bit integer.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output Only] Current state of this License Code.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// [Output Only] If true, the license will remain attached when creating images or snapshots from disks. Otherwise, the license is not transferred.
    #[serde(rename = "transferable", skip_serializing_if = "Option::is_none")]
    pub transferable: Option<bool>,
}

impl LicenseCode {
    /// Represents a License Code resource. A License Code is a unique identifier used to represent a license resource. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.
    pub fn new() -> LicenseCode {
        LicenseCode {
            creation_timestamp: None,
            description: None,
            id: None,
            kind: None,
            license_alias: None,
            name: None,
            self_link: None,
            state: None,
            transferable: None,
        }
    }
}
/// [Output Only] Current state of this License Code.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "RESTRICTED")]
    Restricted,
    #[serde(rename = "STATE_UNSPECIFIED")]
    StateUnspecified,
    #[serde(rename = "TERMINATED")]
    Terminated,
}

impl Default for State {
    fn default() -> State {
        Self::Disabled
    }
}
