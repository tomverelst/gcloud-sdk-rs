use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// InstancesExportRequest : Database instance export request.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstancesExportRequest {
    #[serde(rename = "exportContext", skip_serializing_if = "Option::is_none")]
    pub export_context: Option<Box<crate::google_rest_apis::sqladmin_v1::models::ExportContext>>,
}

impl InstancesExportRequest {
    /// Database instance export request.
    pub fn new() -> InstancesExportRequest {
        InstancesExportRequest {
            export_context: None,
        }
    }
}
