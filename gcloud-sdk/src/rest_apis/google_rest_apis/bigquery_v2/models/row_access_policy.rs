use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// RowAccessPolicy : Represents access on a subset of rows on the specified table, defined by its filter predicate. Access to the subset of rows is controlled by its IAM policy.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RowAccessPolicy {
    /// Output only. The time when this row access policy was created, in milliseconds since the epoch.
    #[serde(rename = "creationTime", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// Output only. A hash of this resource.
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// Required. A SQL boolean expression that represents the rows defined by this row access policy, similar to the boolean expression in a WHERE clause of a SELECT query on a table. References to other tables, routines, and temporary functions are not supported. Examples: region=\"EU\" date_field = CAST('2019-9-27' as DATE) nullable_field is not NULL numeric_field BETWEEN 1.0 AND 5.0
    #[serde(rename = "filterPredicate", skip_serializing_if = "Option::is_none")]
    pub filter_predicate: Option<String>,
    /// Output only. The time when this row access policy was last modified, in milliseconds since the epoch.
    #[serde(rename = "lastModifiedTime", skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(
        rename = "rowAccessPolicyReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub row_access_policy_reference:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::RowAccessPolicyReference>>,
}

impl RowAccessPolicy {
    /// Represents access on a subset of rows on the specified table, defined by its filter predicate. Access to the subset of rows is controlled by its IAM policy.
    pub fn new() -> RowAccessPolicy {
        RowAccessPolicy {
            creation_time: None,
            etag: None,
            filter_predicate: None,
            last_modified_time: None,
            row_access_policy_reference: None,
        }
    }
}
