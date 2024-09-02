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
pub struct ResponsePoliciesPatchResponse {
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<models::ResponseHeader>>,
    #[serde(rename = "responsePolicy", skip_serializing_if = "Option::is_none")]
    pub response_policy: Option<Box<models::ResponsePolicy>>,
}

impl ResponsePoliciesPatchResponse {
    pub fn new() -> ResponsePoliciesPatchResponse {
        ResponsePoliciesPatchResponse {
            header: None,
            response_policy: None,
        }
    }
}
