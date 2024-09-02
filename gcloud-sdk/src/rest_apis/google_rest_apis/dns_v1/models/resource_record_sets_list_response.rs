use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResourceRecordSetsListResponse {
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<crate::google_rest_apis::dns_v1::models::ResponseHeader>>,
    /// Type of resource.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. This lets you retrieve the complete contents of even larger collections, one page at a time. However, if the collection changes between paginated list requests, the set of elements returned is an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The resource record set resources.
    #[serde(rename = "rrsets", skip_serializing_if = "Option::is_none")]
    pub rrsets: Option<Vec<crate::google_rest_apis::dns_v1::models::ResourceRecordSet>>,
}

impl ResourceRecordSetsListResponse {
    pub fn new() -> ResourceRecordSetsListResponse {
        ResourceRecordSetsListResponse {
            header: None,
            kind: None,
            next_page_token: None,
            rrsets: None,
        }
    }
}
