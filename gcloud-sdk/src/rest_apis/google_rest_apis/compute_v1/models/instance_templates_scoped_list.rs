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
pub struct InstanceTemplatesScopedList {
    /// [Output Only] A list of instance templates that are contained within the specified project and zone.
    #[serde(rename = "instanceTemplates", skip_serializing_if = "Option::is_none")]
    pub instance_templates: Option<Vec<models::InstanceTemplate>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::InstanceTemplatesScopedListWarning>>,
}

impl InstanceTemplatesScopedList {
    pub fn new() -> InstanceTemplatesScopedList {
        InstanceTemplatesScopedList {
            instance_templates: None,
            warning: None,
        }
    }
}
