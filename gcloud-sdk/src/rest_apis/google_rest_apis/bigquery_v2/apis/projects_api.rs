/*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`bigquery_projects_get_service_account`]
#[derive(Clone, Debug, Default)]
pub struct BigqueryPeriodProjectsPeriodGetServiceAccountParams {
    /// Project ID for which the service account is requested.
    pub project_id: String,
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
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
}

/// struct for passing parameters to the method [`bigquery_projects_list`]
#[derive(Clone, Debug, Default)]
pub struct BigqueryPeriodProjectsPeriodListParams {
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
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// Maximum number of results to return
    pub max_results: Option<i32>,
    /// Page token, returned by a previous call, to request the next page of results
    pub page_token: Option<String>,
}

/// struct for typed errors of method [`bigquery_projects_get_service_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BigqueryPeriodProjectsPeriodGetServiceAccountError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bigquery_projects_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BigqueryPeriodProjectsPeriodListError {
    UnknownValue(serde_json::Value),
}

/// Returns the email address of the service account for your project used for interactions with Google Cloud KMS.
pub async fn bigquery_projects_get_service_account(
    configuration: &configuration::Configuration,
    params: BigqueryPeriodProjectsPeriodGetServiceAccountParams,
) -> Result<
    models::GetServiceAccountResponse,
    Error<BigqueryPeriodProjectsPeriodGetServiceAccountError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project_id = params.project_id;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let user_ip = params.user_ip;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{projectId}/serviceAccount",
        local_var_configuration.base_path,
        projectId = crate::google_rest_apis::bigquery_v2::apis::urlencode(project_id)
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
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
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
        let local_var_entity: Option<BigqueryPeriodProjectsPeriodGetServiceAccountError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all projects to which you have been granted any project role.
pub async fn bigquery_projects_list(
    configuration: &configuration::Configuration,
    params: BigqueryPeriodProjectsPeriodListParams,
) -> Result<models::ProjectList, Error<BigqueryPeriodProjectsPeriodListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let user_ip = params.user_ip;
    let max_results = params.max_results;
    let page_token = params.page_token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects", local_var_configuration.base_path);
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
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_results {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxResults", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
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
        let local_var_entity: Option<BigqueryPeriodProjectsPeriodListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
