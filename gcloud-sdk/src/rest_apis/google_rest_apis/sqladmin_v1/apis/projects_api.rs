/*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`sql_projects_instances_reschedule_maintenance`]
#[derive(Clone, Debug, Default)]
pub struct SqlPeriodProjectsPeriodInstancesPeriodRescheduleMaintenanceParams {
    /// ID of the project that contains the instance.
    pub project: String,
    /// Cloud SQL instance ID. This does not include the project ID.
    pub instance: String,
    /// V1 error format.
    pub dollar_xgafv: Option<String>,
    /// OAuth access token.
    pub access_token: Option<String>,
    /// Data format for response.
    pub alt: Option<String>,
    /// JSONP
    pub callback: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"raw\", \"multipart\").
    pub upload_protocol: Option<String>,
    /// Legacy upload protocol for media (e.g. \"media\", \"multipart\").
    pub upload_type: Option<String>,
    pub sql_instances_reschedule_maintenance_request_body:
        Option<models::SqlInstancesRescheduleMaintenanceRequestBody>,
}

/// struct for passing parameters to the method [`sql_projects_instances_start_external_sync`]
#[derive(Clone, Debug, Default)]
pub struct SqlPeriodProjectsPeriodInstancesPeriodStartExternalSyncParams {
    /// ID of the project that contains the instance.
    pub project: String,
    /// Cloud SQL instance ID. This does not include the project ID.
    pub instance: String,
    /// V1 error format.
    pub dollar_xgafv: Option<String>,
    /// OAuth access token.
    pub access_token: Option<String>,
    /// Data format for response.
    pub alt: Option<String>,
    /// JSONP
    pub callback: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"raw\", \"multipart\").
    pub upload_protocol: Option<String>,
    /// Legacy upload protocol for media (e.g. \"media\", \"multipart\").
    pub upload_type: Option<String>,
    pub sql_instances_start_external_sync_request:
        Option<models::SqlInstancesStartExternalSyncRequest>,
}

/// struct for passing parameters to the method [`sql_projects_instances_verify_external_sync_settings`]
#[derive(Clone, Debug, Default)]
pub struct SqlPeriodProjectsPeriodInstancesPeriodVerifyExternalSyncSettingsParams {
    /// Project ID of the project that contains the instance.
    pub project: String,
    /// Cloud SQL instance ID. This does not include the project ID.
    pub instance: String,
    /// V1 error format.
    pub dollar_xgafv: Option<String>,
    /// OAuth access token.
    pub access_token: Option<String>,
    /// Data format for response.
    pub alt: Option<String>,
    /// JSONP
    pub callback: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"raw\", \"multipart\").
    pub upload_protocol: Option<String>,
    /// Legacy upload protocol for media (e.g. \"media\", \"multipart\").
    pub upload_type: Option<String>,
    pub sql_instances_verify_external_sync_settings_request:
        Option<models::SqlInstancesVerifyExternalSyncSettingsRequest>,
}

/// struct for typed errors of method [`sql_projects_instances_reschedule_maintenance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SqlPeriodProjectsPeriodInstancesPeriodRescheduleMaintenanceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sql_projects_instances_start_external_sync`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SqlPeriodProjectsPeriodInstancesPeriodStartExternalSyncError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sql_projects_instances_verify_external_sync_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SqlPeriodProjectsPeriodInstancesPeriodVerifyExternalSyncSettingsError {
    UnknownValue(serde_json::Value),
}

/// Reschedules the maintenance on the given instance.
pub async fn sql_projects_instances_reschedule_maintenance(
    configuration: &configuration::Configuration,
    params: SqlPeriodProjectsPeriodInstancesPeriodRescheduleMaintenanceParams,
) -> Result<
    models::Operation,
    Error<SqlPeriodProjectsPeriodInstancesPeriodRescheduleMaintenanceError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project = params.project;
    let instance = params.instance;
    let dollar_xgafv = params.dollar_xgafv;
    let access_token = params.access_token;
    let alt = params.alt;
    let callback = params.callback;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_protocol = params.upload_protocol;
    let upload_type = params.upload_type;
    let sql_instances_reschedule_maintenance_request_body =
        params.sql_instances_reschedule_maintenance_request_body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/projects/{project}/instances/{instance}/rescheduleMaintenance",
        local_var_configuration.base_path,
        project = crate::google_rest_apis::sqladmin_v1::apis::urlencode(project),
        instance = crate::google_rest_apis::sqladmin_v1::apis::urlencode(instance)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = dollar_xgafv {
        local_var_req_builder =
            local_var_req_builder.query(&[("$.xgafv", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = access_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("access_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = callback {
        local_var_req_builder =
            local_var_req_builder.query(&[("callback", &local_var_str.to_string())]);
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
    if let Some(ref local_var_str) = upload_protocol {
        local_var_req_builder =
            local_var_req_builder.query(&[("upload_protocol", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
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
    local_var_req_builder =
        local_var_req_builder.json(&sql_instances_reschedule_maintenance_request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<
            SqlPeriodProjectsPeriodInstancesPeriodRescheduleMaintenanceError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Start External primary instance migration.
pub async fn sql_projects_instances_start_external_sync(
    configuration: &configuration::Configuration,
    params: SqlPeriodProjectsPeriodInstancesPeriodStartExternalSyncParams,
) -> Result<models::Operation, Error<SqlPeriodProjectsPeriodInstancesPeriodStartExternalSyncError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let project = params.project;
    let instance = params.instance;
    let dollar_xgafv = params.dollar_xgafv;
    let access_token = params.access_token;
    let alt = params.alt;
    let callback = params.callback;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_protocol = params.upload_protocol;
    let upload_type = params.upload_type;
    let sql_instances_start_external_sync_request =
        params.sql_instances_start_external_sync_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/projects/{project}/instances/{instance}/startExternalSync",
        local_var_configuration.base_path,
        project = crate::google_rest_apis::sqladmin_v1::apis::urlencode(project),
        instance = crate::google_rest_apis::sqladmin_v1::apis::urlencode(instance)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = dollar_xgafv {
        local_var_req_builder =
            local_var_req_builder.query(&[("$.xgafv", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = access_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("access_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = callback {
        local_var_req_builder =
            local_var_req_builder.query(&[("callback", &local_var_str.to_string())]);
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
    if let Some(ref local_var_str) = upload_protocol {
        local_var_req_builder =
            local_var_req_builder.query(&[("upload_protocol", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
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
    local_var_req_builder = local_var_req_builder.json(&sql_instances_start_external_sync_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SqlPeriodProjectsPeriodInstancesPeriodStartExternalSyncError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Verify External primary instance external sync settings.
pub async fn sql_projects_instances_verify_external_sync_settings(
    configuration: &configuration::Configuration,
    params: SqlPeriodProjectsPeriodInstancesPeriodVerifyExternalSyncSettingsParams,
) -> Result<
    models::SqlInstancesVerifyExternalSyncSettingsResponse,
    Error<SqlPeriodProjectsPeriodInstancesPeriodVerifyExternalSyncSettingsError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project = params.project;
    let instance = params.instance;
    let dollar_xgafv = params.dollar_xgafv;
    let access_token = params.access_token;
    let alt = params.alt;
    let callback = params.callback;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_protocol = params.upload_protocol;
    let upload_type = params.upload_type;
    let sql_instances_verify_external_sync_settings_request =
        params.sql_instances_verify_external_sync_settings_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/projects/{project}/instances/{instance}/verifyExternalSyncSettings",
        local_var_configuration.base_path,
        project = crate::google_rest_apis::sqladmin_v1::apis::urlencode(project),
        instance = crate::google_rest_apis::sqladmin_v1::apis::urlencode(instance)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = dollar_xgafv {
        local_var_req_builder =
            local_var_req_builder.query(&[("$.xgafv", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = access_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("access_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = callback {
        local_var_req_builder =
            local_var_req_builder.query(&[("callback", &local_var_str.to_string())]);
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
    if let Some(ref local_var_str) = upload_protocol {
        local_var_req_builder =
            local_var_req_builder.query(&[("upload_protocol", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
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
    local_var_req_builder =
        local_var_req_builder.json(&sql_instances_verify_external_sync_settings_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<
            SqlPeriodProjectsPeriodInstancesPeriodVerifyExternalSyncSettingsError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
