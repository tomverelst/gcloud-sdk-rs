/// The request to ListTunnelDestGroups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTunnelDestGroupsRequest {
    /// Required. Google Cloud Project ID and location.
    /// In the following format:
    /// `projects/{project_number/id}/iap_tunnel/locations/{location}`.
    /// A `-` can be used for the location to group across all locations.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of groups to return. The service might return fewer than
    /// this value.
    /// If unspecified, at most 100 groups are returned.
    /// The maximum value is 1000; values above 1000 are coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListTunnelDestGroups`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListTunnelDestGroups` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from ListTunnelDestGroups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTunnelDestGroupsResponse {
    /// TunnelDestGroup existing in the project.
    #[prost(message, repeated, tag = "1")]
    pub tunnel_dest_groups: ::prost::alloc::vec::Vec<TunnelDestGroup>,
    /// A token that you can send as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to CreateTunnelDestGroup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTunnelDestGroupRequest {
    /// Required. Google Cloud Project ID and location.
    /// In the following format:
    /// `projects/{project_number/id}/iap_tunnel/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The TunnelDestGroup to create.
    #[prost(message, optional, tag = "2")]
    pub tunnel_dest_group: ::core::option::Option<TunnelDestGroup>,
    /// Required. The ID to use for the TunnelDestGroup, which becomes the final
    /// component of the resource name.
    ///
    /// This value must be 4-63 characters, and valid characters
    /// are `\[a-z\]-`.
    #[prost(string, tag = "3")]
    pub tunnel_dest_group_id: ::prost::alloc::string::String,
}
/// The request to GetTunnelDestGroup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTunnelDestGroupRequest {
    /// Required. Name of the TunnelDestGroup to be fetched.
    /// In the following format:
    /// `projects/{project_number/id}/iap_tunnel/locations/{location}/destGroups/{dest_group}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request to DeleteTunnelDestGroup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTunnelDestGroupRequest {
    /// Required. Name of the TunnelDestGroup to delete.
    /// In the following format:
    /// `projects/{project_number/id}/iap_tunnel/locations/{location}/destGroups/{dest_group}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request to UpdateTunnelDestGroup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTunnelDestGroupRequest {
    /// Required. The new values for the TunnelDestGroup.
    #[prost(message, optional, tag = "1")]
    pub tunnel_dest_group: ::core::option::Option<TunnelDestGroup>,
    /// A field mask that specifies which IAP settings to update.
    /// If omitted, then all of the settings are updated. See
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A TunnelDestGroup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TunnelDestGroup {
    /// Required. Immutable. Identifier for the TunnelDestGroup. Must be unique
    /// within the project and contain only lower case letters (a-z) and dashes
    /// (-).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Unordered list. List of CIDRs that this group applies to.
    #[prost(string, repeated, tag = "2")]
    pub cidrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Unordered list. List of FQDNs that this group applies to.
    #[prost(string, repeated, tag = "3")]
    pub fqdns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request sent to GetIapSettings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIapSettingsRequest {
    /// Required. The resource name for which to retrieve the settings.
    /// Authorization: Requires the `getSettings` permission for the associated
    /// resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to UpdateIapSettings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIapSettingsRequest {
    /// Required. The new values for the IAP settings to be updated.
    /// Authorization: Requires the `updateSettings` permission for the associated
    /// resource.
    #[prost(message, optional, tag = "1")]
    pub iap_settings: ::core::option::Option<IapSettings>,
    /// The field mask specifying which IAP settings should be updated.
    /// If omitted, then all of the settings are updated. See
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask.>
    ///
    /// Note: All IAP reauth settings must always be set together, using the
    /// field mask: `iapSettings.accessSettings.reauthSettings`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The IAP configurable settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IapSettings {
    /// Required. The resource name of the IAP protected resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Top level wrapper for all access related setting in IAP
    #[prost(message, optional, tag = "5")]
    pub access_settings: ::core::option::Option<AccessSettings>,
    /// Top level wrapper for all application related settings in IAP
    #[prost(message, optional, tag = "6")]
    pub application_settings: ::core::option::Option<ApplicationSettings>,
}
/// Access related settings for IAP protected apps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessSettings {
    /// GCIP claims and endpoint configurations for 3p identity providers.
    #[prost(message, optional, tag = "1")]
    pub gcip_settings: ::core::option::Option<GcipSettings>,
    /// Configuration to allow cross-origin requests via IAP.
    #[prost(message, optional, tag = "2")]
    pub cors_settings: ::core::option::Option<CorsSettings>,
    /// Settings to configure IAP's OAuth behavior.
    #[prost(message, optional, tag = "3")]
    pub oauth_settings: ::core::option::Option<OAuthSettings>,
    /// Settings to configure reauthentication policies in IAP.
    #[prost(message, optional, tag = "6")]
    pub reauth_settings: ::core::option::Option<ReauthSettings>,
    /// Settings to configure and enable allowed domains.
    #[prost(message, optional, tag = "7")]
    pub allowed_domains_settings: ::core::option::Option<AllowedDomainsSettings>,
}
/// Allows customers to configure tenant_id for GCIP instance per-app.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcipSettings {
    /// GCIP tenant ids that are linked to the IAP resource.
    /// tenant_ids could be a string beginning with a number character to indicate
    /// authenticating with GCIP tenant flow, or in the format of _<ProjectNumber>
    /// to indicate authenticating with GCIP agent flow.
    /// If agent flow is used, tenant_ids should only contain one single element,
    /// while for tenant flow, tenant_ids can contain multiple elements.
    #[prost(string, repeated, tag = "1")]
    pub tenant_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Login page URI associated with the GCIP tenants.
    /// Typically, all resources within the same project share the same login page,
    /// though it could be overridden at the sub resource level.
    #[prost(message, optional, tag = "2")]
    pub login_page_uri: ::core::option::Option<::prost::alloc::string::String>,
}
/// Allows customers to configure HTTP request paths that'll allow HTTP OPTIONS
/// call to bypass authentication and authorization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CorsSettings {
    /// Configuration to allow HTTP OPTIONS calls to skip authorization. If
    /// undefined, IAP will not apply any special logic to OPTIONS requests.
    #[prost(message, optional, tag = "1")]
    pub allow_http_options: ::core::option::Option<bool>,
}
/// Configuration for OAuth login&consent flow behavior as well as for OAuth
/// Credentials.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuthSettings {
    /// Domain hint to send as hd=? parameter in OAuth request flow. Enables
    /// redirect to primary IDP by skipping Google's login screen.
    /// <https://developers.google.com/identity/protocols/OpenIDConnect#hd-param>
    /// Note: IAP does not verify that the id token's hd claim matches this value
    /// since access behavior is managed by IAM policies.
    #[prost(message, optional, tag = "2")]
    pub login_hint: ::core::option::Option<::prost::alloc::string::String>,
}
/// Configuration for IAP reauthentication policies.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReauthSettings {
    /// Reauth method requested.
    #[prost(enumeration = "reauth_settings::Method", tag = "1")]
    pub method: i32,
    /// Reauth session lifetime, how long before a user has to reauthenticate
    /// again.
    #[prost(message, optional, tag = "2")]
    pub max_age: ::core::option::Option<::prost_types::Duration>,
    /// How IAP determines the effective policy in cases of hierarchial policies.
    /// Policies are merged from higher in the hierarchy to lower in the hierarchy.
    #[prost(enumeration = "reauth_settings::PolicyType", tag = "3")]
    pub policy_type: i32,
}
/// Nested message and enum types in `ReauthSettings`.
pub mod reauth_settings {
    /// Types of reauthentication methods supported by IAP.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Method {
        /// Reauthentication disabled.
        Unspecified = 0,
        /// Prompts the user to log in again.
        Login = 1,
        /// Deprecated, no longer accepted by IAP APIs.
        Password = 2,
        /// User must use their secure key 2nd factor device.
        SecureKey = 3,
        /// User can use any enabled 2nd factor.
        EnrolledSecondFactors = 4,
    }
    impl Method {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Method::Unspecified => "METHOD_UNSPECIFIED",
                Method::Login => "LOGIN",
                Method::Password => "PASSWORD",
                Method::SecureKey => "SECURE_KEY",
                Method::EnrolledSecondFactors => "ENROLLED_SECOND_FACTORS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "METHOD_UNSPECIFIED" => Some(Self::Unspecified),
                "LOGIN" => Some(Self::Login),
                "PASSWORD" => Some(Self::Password),
                "SECURE_KEY" => Some(Self::SecureKey),
                "ENROLLED_SECOND_FACTORS" => Some(Self::EnrolledSecondFactors),
                _ => None,
            }
        }
    }
    /// Type of policy in the case of hierarchial policies.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PolicyType {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// This policy acts as a minimum to other policies, lower in the hierarchy.
        /// Effective policy may only be the same or stricter.
        Minimum = 1,
        /// This policy acts as a default if no other reauth policy is set.
        Default = 2,
    }
    impl PolicyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PolicyType::Unspecified => "POLICY_TYPE_UNSPECIFIED",
                PolicyType::Minimum => "MINIMUM",
                PolicyType::Default => "DEFAULT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "POLICY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "MINIMUM" => Some(Self::Minimum),
                "DEFAULT" => Some(Self::Default),
                _ => None,
            }
        }
    }
}
/// Configuration for IAP allowed domains. Lets you to restrict access to an app
/// and allow access to only the domains that you list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedDomainsSettings {
    /// Configuration for customers to opt in for the feature.
    #[prost(bool, optional, tag = "1")]
    pub enable: ::core::option::Option<bool>,
    /// List of trusted domains.
    #[prost(string, repeated, tag = "2")]
    pub domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Wrapper over application specific settings for IAP.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationSettings {
    /// Settings to configure IAP's behavior for a service mesh.
    #[prost(message, optional, tag = "1")]
    pub csm_settings: ::core::option::Option<CsmSettings>,
    /// Customization for Access Denied page.
    #[prost(message, optional, tag = "2")]
    pub access_denied_page_settings: ::core::option::Option<AccessDeniedPageSettings>,
    /// The Domain value to set for cookies generated by IAP. This value is not
    /// validated by the API, but will be ignored at runtime if invalid.
    #[prost(message, optional, tag = "3")]
    pub cookie_domain: ::core::option::Option<::prost::alloc::string::String>,
    /// Settings to configure attribute propagation.
    #[prost(message, optional, tag = "4")]
    pub attribute_propagation_settings: ::core::option::Option<
        AttributePropagationSettings,
    >,
}
/// Configuration for RCToken generated for service mesh workloads protected by
/// IAP. RCToken are IAP generated JWTs that can be verified at the application.
/// The RCToken is primarily used for service mesh deployments, and can be scoped
/// to a single mesh by configuring the audience field accordingly.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsmSettings {
    /// Audience claim set in the generated RCToken. This value is not validated by
    /// IAP.
    #[prost(message, optional, tag = "1")]
    pub rctoken_aud: ::core::option::Option<::prost::alloc::string::String>,
}
/// Custom content configuration for access denied page.
/// IAP allows customers to define a custom URI to use as the error page when
/// access is denied to users. If IAP prevents access to this page, the default
/// IAP error page will be displayed instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDeniedPageSettings {
    /// The URI to be redirected to when access is denied.
    #[prost(message, optional, tag = "1")]
    pub access_denied_page_uri: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether to generate a troubleshooting URL on access denied events to this
    /// application.
    #[prost(message, optional, tag = "2")]
    pub generate_troubleshooting_uri: ::core::option::Option<bool>,
    /// Whether to generate remediation token on access denied events to this
    /// application.
    #[prost(message, optional, tag = "3")]
    pub remediation_token_generation_enabled: ::core::option::Option<bool>,
}
/// Configuration for propagating attributes to applications protected
/// by IAP.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributePropagationSettings {
    /// Raw string CEL expression. Must return a list of attributes. Maximum of 45
    /// attributes can be selected. Expressions can select different attribute
    /// types from `attributes`: `attributes.saml_attributes`,
    /// `attributes.iap_attributes`. Limited functions are supported:
    ///   - `filter: <list>.filter(<iter_var>, <predicate>)` -> returns a subset of
    ///   `<list>` where `<predicate>` is true for every item.
    ///   - `in: <var> in <list>` -> returns true if `<list>` contains `<var>`
    ///   - `selectByName: <list>.selectByName(<string>)` -> returns the attribute
    ///   in
    ///   `<list>` with the given `<string>` name, otherwise returns empty.
    ///   - `emitAs: <attribute>.emitAs(<string>)` -> sets the `<attribute>` name
    ///   field to the given `<string>` for propagation in selected output
    ///   credentials.
    ///   - `strict: <attribute>.strict()` -> ignore the `x-goog-iap-attr-` prefix
    ///   for the provided `<attribute>` when propagating via the `HEADER` output
    ///   credential, i.e. request headers.
    ///   - `append: <target_list>.append(<attribute>)` OR
    ///   `<target_list>.append(<list>)` -> append the provided `<attribute>` or
    ///   `<list>` onto the end of `<target_list>`.
    ///
    /// Example expression: `attributes.saml_attributes.filter(x, x.name in
    /// \['test'\]).append(attributes.iap_attributes.selectByName('exact').emitAs('custom').strict())`
    #[prost(string, optional, tag = "1")]
    pub expression: ::core::option::Option<::prost::alloc::string::String>,
    /// Which output credentials attributes selected by the CEL expression should
    /// be propagated in. All attributes will be fully duplicated in each selected
    /// output credential.
    #[prost(
        enumeration = "attribute_propagation_settings::OutputCredentials",
        repeated,
        tag = "2"
    )]
    pub output_credentials: ::prost::alloc::vec::Vec<i32>,
    /// Whether the provided attribute propagation settings should be evaluated on
    /// user requests. If set to true, attributes returned from the expression will
    /// be propagated in the set output credentials.
    #[prost(bool, optional, tag = "3")]
    pub enable: ::core::option::Option<bool>,
}
/// Nested message and enum types in `AttributePropagationSettings`.
pub mod attribute_propagation_settings {
    /// Supported output credentials for attribute propagation. Each output
    /// credential maps to a "field" in the response. For example, selecting JWT
    /// will propagate all attributes in the IAP JWT, header in the headers, etc.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OutputCredentials {
        /// No output credential. This is an unsupported default.
        Unspecified = 0,
        /// Propagate attributes in the headers with "x-goog-iap-attr-" prefix.
        Header = 1,
        /// Propagate attributes in the JWT of the form: `"additional_claims": {
        /// "my_attribute": ["value1", "value2"] }`
        Jwt = 2,
        /// Propagate attributes in the RCToken of the form: `"additional_claims": {
        /// "my_attribute": ["value1", "value2"] }`
        Rctoken = 3,
    }
    impl OutputCredentials {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OutputCredentials::Unspecified => "OUTPUT_CREDENTIALS_UNSPECIFIED",
                OutputCredentials::Header => "HEADER",
                OutputCredentials::Jwt => "JWT",
                OutputCredentials::Rctoken => "RCTOKEN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OUTPUT_CREDENTIALS_UNSPECIFIED" => Some(Self::Unspecified),
                "HEADER" => Some(Self::Header),
                "JWT" => Some(Self::Jwt),
                "RCTOKEN" => Some(Self::Rctoken),
                _ => None,
            }
        }
    }
}
/// The request sent to ListBrands.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBrandsRequest {
    /// Required. GCP Project number/id.
    /// In the following format: projects/{project_number/id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response message for ListBrands.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBrandsResponse {
    /// Brands existing in the project.
    #[prost(message, repeated, tag = "1")]
    pub brands: ::prost::alloc::vec::Vec<Brand>,
}
/// The request sent to CreateBrand.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBrandRequest {
    /// Required. GCP Project number/id under which the brand is to be created.
    /// In the following format: projects/{project_number/id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The brand to be created.
    #[prost(message, optional, tag = "2")]
    pub brand: ::core::option::Option<Brand>,
}
/// The request sent to GetBrand.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandRequest {
    /// Required. Name of the brand to be fetched.
    /// In the following format: projects/{project_number/id}/brands/{brand}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to ListIdentityAwareProxyClients.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIdentityAwareProxyClientsRequest {
    /// Required. Full brand path.
    /// In the following format: projects/{project_number/id}/brands/{brand}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of clients to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 100 clients will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListIdentityAwareProxyClients`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListIdentityAwareProxyClients` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListIdentityAwareProxyClients.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIdentityAwareProxyClientsResponse {
    /// Clients existing in the brand.
    #[prost(message, repeated, tag = "1")]
    pub identity_aware_proxy_clients: ::prost::alloc::vec::Vec<IdentityAwareProxyClient>,
    /// A token, which can be send as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request sent to CreateIdentityAwareProxyClient.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIdentityAwareProxyClientRequest {
    /// Required. Path to create the client in.
    /// In the following format:
    /// projects/{project_number/id}/brands/{brand}.
    /// The project must belong to a G Suite account.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Identity Aware Proxy Client to be created.
    #[prost(message, optional, tag = "2")]
    pub identity_aware_proxy_client: ::core::option::Option<IdentityAwareProxyClient>,
}
/// The request sent to GetIdentityAwareProxyClient.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdentityAwareProxyClientRequest {
    /// Required. Name of the Identity Aware Proxy client to be fetched.
    /// In the following format:
    /// projects/{project_number/id}/brands/{brand}/identityAwareProxyClients/{client_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to ResetIdentityAwareProxyClientSecret.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetIdentityAwareProxyClientSecretRequest {
    /// Required. Name of the Identity Aware Proxy client to that will have its
    /// secret reset. In the following format:
    /// projects/{project_number/id}/brands/{brand}/identityAwareProxyClients/{client_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to DeleteIdentityAwareProxyClient.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIdentityAwareProxyClientRequest {
    /// Required. Name of the Identity Aware Proxy client to be deleted.
    /// In the following format:
    /// projects/{project_number/id}/brands/{brand}/identityAwareProxyClients/{client_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// OAuth brand data.
/// NOTE: Only contains a portion of the data that describes a brand.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brand {
    /// Output only. Identifier of the brand.
    /// NOTE: GCP project number achieves the same brand identification purpose as
    /// only one brand per project can be created.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Support email displayed on the OAuth consent screen.
    #[prost(string, tag = "2")]
    pub support_email: ::prost::alloc::string::String,
    /// Application name displayed on OAuth consent screen.
    #[prost(string, tag = "3")]
    pub application_title: ::prost::alloc::string::String,
    /// Output only. Whether the brand is only intended for usage inside the
    /// G Suite organization only.
    #[prost(bool, tag = "4")]
    pub org_internal_only: bool,
}
/// Contains the data that describes an Identity Aware Proxy owned client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityAwareProxyClient {
    /// Output only. Unique identifier of the OAuth client.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Client secret of the OAuth client.
    #[prost(string, tag = "2")]
    pub secret: ::prost::alloc::string::String,
    /// Human-friendly name given to the OAuth client.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod identity_aware_proxy_admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// APIs for Identity-Aware Proxy Admin configurations.
    #[derive(Debug, Clone)]
    pub struct IdentityAwareProxyAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IdentityAwareProxyAdminServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IdentityAwareProxyAdminServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> IdentityAwareProxyAdminServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            IdentityAwareProxyAdminServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Sets the access control policy for an Identity-Aware Proxy protected
        /// resource. Replaces any existing policy.
        /// More information about managing access via IAP can be found at:
        /// https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy for an Identity-Aware Proxy protected
        /// resource.
        /// More information about managing access via IAP can be found at:
        /// https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns permissions that a caller has on the Identity-Aware Proxy protected
        /// resource.
        /// More information about managing access via IAP can be found at:
        /// https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the IAP settings on a particular IAP protected resource.
        pub async fn get_iap_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIapSettingsRequest>,
        ) -> Result<tonic::Response<super::IapSettings>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/GetIapSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the IAP settings on a particular IAP protected resource. It
        /// replaces all fields unless the `update_mask` is set.
        pub async fn update_iap_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIapSettingsRequest>,
        ) -> Result<tonic::Response<super::IapSettings>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/UpdateIapSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the existing TunnelDestGroups. To group across all locations, use a
        /// `-` as the location ID. For example:
        /// `/v1/projects/123/iap_tunnel/locations/-/destGroups`
        pub async fn list_tunnel_dest_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTunnelDestGroupsRequest>,
        ) -> Result<
            tonic::Response<super::ListTunnelDestGroupsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/ListTunnelDestGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new TunnelDestGroup.
        pub async fn create_tunnel_dest_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTunnelDestGroupRequest>,
        ) -> Result<tonic::Response<super::TunnelDestGroup>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/CreateTunnelDestGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves an existing TunnelDestGroup.
        pub async fn get_tunnel_dest_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTunnelDestGroupRequest>,
        ) -> Result<tonic::Response<super::TunnelDestGroup>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/GetTunnelDestGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a TunnelDestGroup.
        pub async fn delete_tunnel_dest_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTunnelDestGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/DeleteTunnelDestGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a TunnelDestGroup.
        pub async fn update_tunnel_dest_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTunnelDestGroupRequest>,
        ) -> Result<tonic::Response<super::TunnelDestGroup>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/UpdateTunnelDestGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod identity_aware_proxy_o_auth_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API to programmatically create, list and retrieve Identity Aware Proxy (IAP)
    /// OAuth brands; and create, retrieve, delete and reset-secret of IAP OAuth
    /// clients.
    #[derive(Debug, Clone)]
    pub struct IdentityAwareProxyOAuthServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IdentityAwareProxyOAuthServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IdentityAwareProxyOAuthServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> IdentityAwareProxyOAuthServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            IdentityAwareProxyOAuthServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Lists the existing brands for the project.
        pub async fn list_brands(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBrandsRequest>,
        ) -> Result<tonic::Response<super::ListBrandsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/ListBrands",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Constructs a new OAuth brand for the project if one does not exist.
        /// The created brand is "internal only", meaning that OAuth clients created
        /// under it only accept requests from users who belong to the same Google
        /// Workspace organization as the project. The brand is created in an
        /// un-reviewed status. NOTE: The "internal only" status can be manually
        /// changed in the Google Cloud Console. Requires that a brand does not already
        /// exist for the project, and that the specified support email is owned by the
        /// caller.
        pub async fn create_brand(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBrandRequest>,
        ) -> Result<tonic::Response<super::Brand>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/CreateBrand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the OAuth brand of the project.
        pub async fn get_brand(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBrandRequest>,
        ) -> Result<tonic::Response<super::Brand>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/GetBrand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an Identity Aware Proxy (IAP) OAuth client. The client is owned
        /// by IAP. Requires that the brand for the project exists and that it is
        /// set for internal-only use.
        pub async fn create_identity_aware_proxy_client(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateIdentityAwareProxyClientRequest,
            >,
        ) -> Result<tonic::Response<super::IdentityAwareProxyClient>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/CreateIdentityAwareProxyClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the existing clients for the brand.
        pub async fn list_identity_aware_proxy_clients(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIdentityAwareProxyClientsRequest>,
        ) -> Result<
            tonic::Response<super::ListIdentityAwareProxyClientsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/ListIdentityAwareProxyClients",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves an Identity Aware Proxy (IAP) OAuth client.
        /// Requires that the client is owned by IAP.
        pub async fn get_identity_aware_proxy_client(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIdentityAwareProxyClientRequest>,
        ) -> Result<tonic::Response<super::IdentityAwareProxyClient>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/GetIdentityAwareProxyClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Resets an Identity Aware Proxy (IAP) OAuth client secret. Useful if the
        /// secret was compromised. Requires that the client is owned by IAP.
        pub async fn reset_identity_aware_proxy_client_secret(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetIdentityAwareProxyClientSecretRequest,
            >,
        ) -> Result<tonic::Response<super::IdentityAwareProxyClient>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/ResetIdentityAwareProxyClientSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an Identity Aware Proxy (IAP) OAuth client. Useful for removing
        /// obsolete clients, managing the number of clients in a given project, and
        /// cleaning up after tests. Requires that the client is owned by IAP.
        pub async fn delete_identity_aware_proxy_client(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteIdentityAwareProxyClientRequest,
            >,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/DeleteIdentityAwareProxyClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
