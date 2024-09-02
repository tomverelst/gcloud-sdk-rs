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

/// InterconnectAttachmentPartnerMetadata : Informational metadata about Partner attachments from Partners to display to customers. These fields are propagated from PARTNER_PROVIDER attachments to their corresponding PARTNER attachments.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterconnectAttachmentPartnerMetadata {
    /// Plain text name of the Interconnect this attachment is connected to, as displayed in the Partner's portal. For instance \"Chicago 1\". This value may be validated to match approved Partner values.
    #[serde(rename = "interconnectName", skip_serializing_if = "Option::is_none")]
    pub interconnect_name: Option<String>,
    /// Plain text name of the Partner providing this attachment. This value may be validated to match approved Partner values.
    #[serde(rename = "partnerName", skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    /// URL of the Partner's portal for this Attachment. Partners may customise this to be a deep link to the specific resource on the Partner portal. This value may be validated to match approved Partner values.
    #[serde(rename = "portalUrl", skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
}

impl InterconnectAttachmentPartnerMetadata {
    /// Informational metadata about Partner attachments from Partners to display to customers. These fields are propagated from PARTNER_PROVIDER attachments to their corresponding PARTNER attachments.
    pub fn new() -> InterconnectAttachmentPartnerMetadata {
        InterconnectAttachmentPartnerMetadata {
            interconnect_name: None,
            partner_name: None,
            portal_url: None,
        }
    }
}
