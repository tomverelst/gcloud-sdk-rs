/*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`compute_region_instance_groups_get`]
#[derive(Clone, Debug, Default)]
pub struct ComputePeriodRegionInstanceGroupsPeriodGetParams {
    /// Project ID for this request.
    pub project: String,
    /// Name of the region scoping this request.
    pub region: String,
    /// Name of the instance group resource to return.
    pub instance_group: String,
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
    /// Legacy name for parameter that has been superseded by `quotaUser`.
    pub user_ip: Option<String>,
}

/// struct for passing parameters to the method [`compute_region_instance_groups_list`]
#[derive(Clone, Debug, Default)]
pub struct ComputePeriodRegionInstanceGroupsPeriodListParams {
    /// Project ID for this request.
    pub project: String,
    /// Name of the region scoping this request.
    pub region: String,
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
    /// Legacy name for parameter that has been superseded by `quotaUser`.
    pub user_ip: Option<String>,
    /// A filter expression that filters resources listed in the response. Most Compute resources support two types of filter expressions: expressions that support regular expressions and expressions that follow API improvement proposal AIP-160. These two types of filter expressions cannot be mixed in one request. If you want to use AIP-160, your expression must specify the field name, an operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The operator must be either `=`, `!=`, `>`, `<`, `<=`, `>=` or `:`. For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`. The `:*` comparison can be used to test whether a key has been defined. For example, to find all objects with `owner` label use: ``` labels.owner:* ``` You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels. To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\") AND (scheduling.automaticRestart = true) ``` If you want to use a regular expression, use the `eq` (equal) or `ne` (not equal) operator against a single un-parenthesized expression with or without quotes or against multiple parenthesized expressions. Examples: `fieldname eq unquoted literal` `fieldname eq 'single quoted literal'` `fieldname eq \"double quoted literal\"` `(fieldname1 eq literal) (fieldname2 ne \"literal\")` The literal value is interpreted as a regular expression using Google RE2 library syntax. The literal value must match the entire field. For example, to filter for instances that do not end with name \"instance\", you would use `name ne .*instance`. You cannot combine constraints on multiple fields using regular expressions.
    pub filter: Option<String>,
    /// The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)
    pub max_results: Option<i32>,
    /// Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name. You can also sort results in descending order based on the creation timestamp using `orderBy=\"creationTimestamp desc\"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first. Currently, only sorting by `name` or `creationTimestamp desc` is supported.
    pub order_by: Option<String>,
    /// Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results.
    pub page_token: Option<String>,
    /// Opt-in for partial success behavior which provides partial results in case of failure. The default value is false. For example, when partial success behavior is enabled, aggregatedList for a single zone scope either returns all resources in the zone or no resources, with an error code.
    pub return_partial_success: Option<bool>,
}

/// struct for passing parameters to the method [`compute_region_instance_groups_list_instances`]
#[derive(Clone, Debug, Default)]
pub struct ComputePeriodRegionInstanceGroupsPeriodListInstancesParams {
    /// Project ID for this request.
    pub project: String,
    /// Name of the region scoping this request.
    pub region: String,
    /// Name of the regional instance group for which we want to list the instances.
    pub instance_group: String,
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
    /// Legacy name for parameter that has been superseded by `quotaUser`.
    pub user_ip: Option<String>,
    /// A filter expression that filters resources listed in the response. Most Compute resources support two types of filter expressions: expressions that support regular expressions and expressions that follow API improvement proposal AIP-160. These two types of filter expressions cannot be mixed in one request. If you want to use AIP-160, your expression must specify the field name, an operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The operator must be either `=`, `!=`, `>`, `<`, `<=`, `>=` or `:`. For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`. The `:*` comparison can be used to test whether a key has been defined. For example, to find all objects with `owner` label use: ``` labels.owner:* ``` You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels. To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\") AND (scheduling.automaticRestart = true) ``` If you want to use a regular expression, use the `eq` (equal) or `ne` (not equal) operator against a single un-parenthesized expression with or without quotes or against multiple parenthesized expressions. Examples: `fieldname eq unquoted literal` `fieldname eq 'single quoted literal'` `fieldname eq \"double quoted literal\"` `(fieldname1 eq literal) (fieldname2 ne \"literal\")` The literal value is interpreted as a regular expression using Google RE2 library syntax. The literal value must match the entire field. For example, to filter for instances that do not end with name \"instance\", you would use `name ne .*instance`. You cannot combine constraints on multiple fields using regular expressions.
    pub filter: Option<String>,
    /// The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)
    pub max_results: Option<i32>,
    /// Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name. You can also sort results in descending order based on the creation timestamp using `orderBy=\"creationTimestamp desc\"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first. Currently, only sorting by `name` or `creationTimestamp desc` is supported.
    pub order_by: Option<String>,
    /// Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results.
    pub page_token: Option<String>,
    /// Opt-in for partial success behavior which provides partial results in case of failure. The default value is false. For example, when partial success behavior is enabled, aggregatedList for a single zone scope either returns all resources in the zone or no resources, with an error code.
    pub return_partial_success: Option<bool>,
    pub region_instance_groups_list_instances_request:
        Option<models::RegionInstanceGroupsListInstancesRequest>,
}

/// struct for passing parameters to the method [`compute_region_instance_groups_set_named_ports`]
#[derive(Clone, Debug, Default)]
pub struct ComputePeriodRegionInstanceGroupsPeriodSetNamedPortsParams {
    /// Project ID for this request.
    pub project: String,
    /// Name of the region scoping this request.
    pub region: String,
    /// The name of the regional instance group where the named ports are updated.
    pub instance_group: String,
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
    /// Legacy name for parameter that has been superseded by `quotaUser`.
    pub user_ip: Option<String>,
    /// An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported ( 00000000-0000-0000-0000-000000000000).
    pub request_id: Option<String>,
    pub region_instance_groups_set_named_ports_request:
        Option<models::RegionInstanceGroupsSetNamedPortsRequest>,
}

/// struct for typed errors of method [`compute_region_instance_groups_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputePeriodRegionInstanceGroupsPeriodGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`compute_region_instance_groups_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputePeriodRegionInstanceGroupsPeriodListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`compute_region_instance_groups_list_instances`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputePeriodRegionInstanceGroupsPeriodListInstancesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`compute_region_instance_groups_set_named_ports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputePeriodRegionInstanceGroupsPeriodSetNamedPortsError {
    UnknownValue(serde_json::Value),
}

/// Returns the specified instance group resource.
pub async fn compute_region_instance_groups_get(
    configuration: &configuration::Configuration,
    params: ComputePeriodRegionInstanceGroupsPeriodGetParams,
) -> Result<models::InstanceGroup, Error<ComputePeriodRegionInstanceGroupsPeriodGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project = params.project;
    let region = params.region;
    let instance_group = params.instance_group;
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
    let user_ip = params.user_ip;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project}/regions/{region}/instanceGroups/{instanceGroup}",
        local_var_configuration.base_path,
        project = crate::google_rest_apis::compute_v1::apis::urlencode(project),
        region = crate::google_rest_apis::compute_v1::apis::urlencode(region),
        instanceGroup = crate::google_rest_apis::compute_v1::apis::urlencode(instance_group)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ComputePeriodRegionInstanceGroupsPeriodGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the list of instance group resources contained within the specified region.
pub async fn compute_region_instance_groups_list(
    configuration: &configuration::Configuration,
    params: ComputePeriodRegionInstanceGroupsPeriodListParams,
) -> Result<models::RegionInstanceGroupList, Error<ComputePeriodRegionInstanceGroupsPeriodListError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let project = params.project;
    let region = params.region;
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
    let user_ip = params.user_ip;
    let filter = params.filter;
    let max_results = params.max_results;
    let order_by = params.order_by;
    let page_token = params.page_token;
    let return_partial_success = params.return_partial_success;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project}/regions/{region}/instanceGroups",
        local_var_configuration.base_path,
        project = crate::google_rest_apis::compute_v1::apis::urlencode(project),
        region = crate::google_rest_apis::compute_v1::apis::urlencode(region)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_results {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxResults", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("orderBy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = return_partial_success {
        local_var_req_builder =
            local_var_req_builder.query(&[("returnPartialSuccess", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ComputePeriodRegionInstanceGroupsPeriodListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists the instances in the specified instance group and displays information about the named ports. Depending on the specified options, this method can list all instances or only the instances that are running. The orderBy query parameter is not supported.
pub async fn compute_region_instance_groups_list_instances(
    configuration: &configuration::Configuration,
    params: ComputePeriodRegionInstanceGroupsPeriodListInstancesParams,
) -> Result<
    models::RegionInstanceGroupsListInstances,
    Error<ComputePeriodRegionInstanceGroupsPeriodListInstancesError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project = params.project;
    let region = params.region;
    let instance_group = params.instance_group;
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
    let user_ip = params.user_ip;
    let filter = params.filter;
    let max_results = params.max_results;
    let order_by = params.order_by;
    let page_token = params.page_token;
    let return_partial_success = params.return_partial_success;
    let region_instance_groups_list_instances_request =
        params.region_instance_groups_list_instances_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project}/regions/{region}/instanceGroups/{instanceGroup}/listInstances",
        local_var_configuration.base_path,
        project = crate::google_rest_apis::compute_v1::apis::urlencode(project),
        region = crate::google_rest_apis::compute_v1::apis::urlencode(region),
        instanceGroup = crate::google_rest_apis::compute_v1::apis::urlencode(instance_group)
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
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_results {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxResults", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("orderBy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = return_partial_success {
        local_var_req_builder =
            local_var_req_builder.query(&[("returnPartialSuccess", &local_var_str.to_string())]);
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
        local_var_req_builder.json(&region_instance_groups_list_instances_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ComputePeriodRegionInstanceGroupsPeriodListInstancesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets the named ports for the specified regional instance group.
pub async fn compute_region_instance_groups_set_named_ports(
    configuration: &configuration::Configuration,
    params: ComputePeriodRegionInstanceGroupsPeriodSetNamedPortsParams,
) -> Result<models::Operation, Error<ComputePeriodRegionInstanceGroupsPeriodSetNamedPortsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project = params.project;
    let region = params.region;
    let instance_group = params.instance_group;
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
    let user_ip = params.user_ip;
    let request_id = params.request_id;
    let region_instance_groups_set_named_ports_request =
        params.region_instance_groups_set_named_ports_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project}/regions/{region}/instanceGroups/{instanceGroup}/setNamedPorts",
        local_var_configuration.base_path,
        project = crate::google_rest_apis::compute_v1::apis::urlencode(project),
        region = crate::google_rest_apis::compute_v1::apis::urlencode(region),
        instanceGroup = crate::google_rest_apis::compute_v1::apis::urlencode(instance_group)
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
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = request_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("requestId", &local_var_str.to_string())]);
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
        local_var_req_builder.json(&region_instance_groups_set_named_ports_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ComputePeriodRegionInstanceGroupsPeriodSetNamedPortsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
