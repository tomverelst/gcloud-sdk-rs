/*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::storage_v1::models;
use serde::{Deserialize, Serialize};

/// BucketIamConfigurationBucketPolicyOnly : The bucket's uniform bucket-level access configuration. The feature was formerly known as Bucket Policy Only. For backward compatibility, this field will be populated with identical information as the uniformBucketLevelAccess field. We recommend using the uniformBucketLevelAccess field to enable and disable the feature.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BucketIamConfigurationBucketPolicyOnly {
    /// If set, access is controlled only by bucket-level or above IAM policies.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The deadline for changing iamConfiguration.bucketPolicyOnly.enabled from true to false in RFC 3339 format. iamConfiguration.bucketPolicyOnly.enabled may be changed from true to false until the locked time, after which the field is immutable.
    #[serde(rename = "lockedTime", skip_serializing_if = "Option::is_none")]
    pub locked_time: Option<String>,
}

impl BucketIamConfigurationBucketPolicyOnly {
    /// The bucket's uniform bucket-level access configuration. The feature was formerly known as Bucket Policy Only. For backward compatibility, this field will be populated with identical information as the uniformBucketLevelAccess field. We recommend using the uniformBucketLevelAccess field to enable and disable the feature.
    pub fn new() -> BucketIamConfigurationBucketPolicyOnly {
        BucketIamConfigurationBucketPolicyOnly {
            enabled: None,
            locked_time: None,
        }
    }
}
