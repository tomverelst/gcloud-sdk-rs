use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ServiceAttachmentConnectedEndpoint : [Output Only] A connection connected to this service attachment.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceAttachmentConnectedEndpoint {
    /// The url of the consumer network.
    #[serde(rename = "consumerNetwork", skip_serializing_if = "Option::is_none")]
    pub consumer_network: Option<String>,
    /// The url of a connected endpoint.
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// The PSC connection id of the connected endpoint.
    #[serde(rename = "pscConnectionId", skip_serializing_if = "Option::is_none")]
    pub psc_connection_id: Option<String>,
    /// The status of a connected endpoint to this service attachment.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl ServiceAttachmentConnectedEndpoint {
    /// [Output Only] A connection connected to this service attachment.
    pub fn new() -> ServiceAttachmentConnectedEndpoint {
        ServiceAttachmentConnectedEndpoint {
            consumer_network: None,
            endpoint: None,
            psc_connection_id: None,
            status: None,
        }
    }
}

/// The status of a connected endpoint to this service attachment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "NEEDS_ATTENTION")]
    NeedsAttention,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "STATUS_UNSPECIFIED")]
    StatusUnspecified,
}

impl Default for Status {
    fn default() -> Status {
        Self::Accepted
    }
}
