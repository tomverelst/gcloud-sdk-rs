/*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`storage_default_object_access_controls_delete`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodDefaultObjectAccessControlsPeriodDeleteParams {
    /// Name of a bucket.
    pub bucket: String,
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub entity: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
}

/// struct for passing parameters to the method [`storage_default_object_access_controls_get`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodDefaultObjectAccessControlsPeriodGetParams {
    /// Name of a bucket.
    pub bucket: String,
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub entity: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
}

/// struct for passing parameters to the method [`storage_default_object_access_controls_insert`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodDefaultObjectAccessControlsPeriodInsertParams {
    /// Name of a bucket.
    pub bucket: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
    pub object_access_control: Option<models::ObjectAccessControl>,
}

/// struct for passing parameters to the method [`storage_default_object_access_controls_list`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodDefaultObjectAccessControlsPeriodListParams {
    /// Name of a bucket.
    pub bucket: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// If present, only return default ACL listing if the bucket's current metageneration matches this value.
    pub if_metageneration_match: Option<String>,
    /// If present, only return default ACL listing if the bucket's current metageneration does not match the given value.
    pub if_metageneration_not_match: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
}

/// struct for passing parameters to the method [`storage_default_object_access_controls_patch`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodDefaultObjectAccessControlsPeriodPatchParams {
    /// Name of a bucket.
    pub bucket: String,
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub entity: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
    pub object_access_control: Option<models::ObjectAccessControl>,
}

/// struct for passing parameters to the method [`storage_default_object_access_controls_update`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodDefaultObjectAccessControlsPeriodUpdateParams {
    /// Name of a bucket.
    pub bucket: String,
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub entity: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
    pub object_access_control: Option<models::ObjectAccessControl>,
}

/// struct for typed errors of method [`storage_default_object_access_controls_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodDefaultObjectAccessControlsPeriodDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`storage_default_object_access_controls_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodDefaultObjectAccessControlsPeriodGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`storage_default_object_access_controls_insert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodDefaultObjectAccessControlsPeriodInsertError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`storage_default_object_access_controls_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodDefaultObjectAccessControlsPeriodListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`storage_default_object_access_controls_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodDefaultObjectAccessControlsPeriodPatchError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`storage_default_object_access_controls_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodDefaultObjectAccessControlsPeriodUpdateError {
    UnknownValue(serde_json::Value),
}

/// Permanently deletes the default object ACL entry for the specified entity on the specified bucket.
pub async fn storage_default_object_access_controls_delete(
    configuration: &configuration::Configuration,
    params: StoragePeriodDefaultObjectAccessControlsPeriodDeleteParams,
) -> Result<(), Error<StoragePeriodDefaultObjectAccessControlsPeriodDeleteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let entity = params.entity;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let user_project = params.user_project;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/defaultObjectAcl/{entity}",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        entity = crate::google_rest_apis::storage_v1::apis::urlencode(entity)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<StoragePeriodDefaultObjectAccessControlsPeriodDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the default object ACL entry for the specified entity on the specified bucket.
pub async fn storage_default_object_access_controls_get(
    configuration: &configuration::Configuration,
    params: StoragePeriodDefaultObjectAccessControlsPeriodGetParams,
) -> Result<
    models::ObjectAccessControl,
    Error<StoragePeriodDefaultObjectAccessControlsPeriodGetError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let entity = params.entity;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let user_project = params.user_project;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/defaultObjectAcl/{entity}",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        entity = crate::google_rest_apis::storage_v1::apis::urlencode(entity)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodDefaultObjectAccessControlsPeriodGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new default object ACL entry on the specified bucket.
pub async fn storage_default_object_access_controls_insert(
    configuration: &configuration::Configuration,
    params: StoragePeriodDefaultObjectAccessControlsPeriodInsertParams,
) -> Result<
    models::ObjectAccessControl,
    Error<StoragePeriodDefaultObjectAccessControlsPeriodInsertError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let user_project = params.user_project;
    let object_access_control = params.object_access_control;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/defaultObjectAcl",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&object_access_control);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodDefaultObjectAccessControlsPeriodInsertError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves default object ACL entries on the specified bucket.
pub async fn storage_default_object_access_controls_list(
    configuration: &configuration::Configuration,
    params: StoragePeriodDefaultObjectAccessControlsPeriodListParams,
) -> Result<
    models::ObjectAccessControls,
    Error<StoragePeriodDefaultObjectAccessControlsPeriodListError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let if_metageneration_match = params.if_metageneration_match;
    let if_metageneration_not_match = params.if_metageneration_not_match;
    let user_project = params.user_project;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/defaultObjectAcl",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_metageneration_match {
        local_var_req_builder =
            local_var_req_builder.query(&[("ifMetagenerationMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_metageneration_not_match {
        local_var_req_builder = local_var_req_builder
            .query(&[("ifMetagenerationNotMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodDefaultObjectAccessControlsPeriodListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Patches a default object ACL entry on the specified bucket.
pub async fn storage_default_object_access_controls_patch(
    configuration: &configuration::Configuration,
    params: StoragePeriodDefaultObjectAccessControlsPeriodPatchParams,
) -> Result<
    models::ObjectAccessControl,
    Error<StoragePeriodDefaultObjectAccessControlsPeriodPatchError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let entity = params.entity;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let user_project = params.user_project;
    let object_access_control = params.object_access_control;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/defaultObjectAcl/{entity}",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        entity = crate::google_rest_apis::storage_v1::apis::urlencode(entity)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&object_access_control);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodDefaultObjectAccessControlsPeriodPatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a default object ACL entry on the specified bucket.
pub async fn storage_default_object_access_controls_update(
    configuration: &configuration::Configuration,
    params: StoragePeriodDefaultObjectAccessControlsPeriodUpdateParams,
) -> Result<
    models::ObjectAccessControl,
    Error<StoragePeriodDefaultObjectAccessControlsPeriodUpdateError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let entity = params.entity;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let user_project = params.user_project;
    let object_access_control = params.object_access_control;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/defaultObjectAcl/{entity}",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        entity = crate::google_rest_apis::storage_v1::apis::urlencode(entity)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&object_access_control);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodDefaultObjectAccessControlsPeriodUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
