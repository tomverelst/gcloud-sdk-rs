/// The schema for settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Setting {
    /// The resource name of the setting. Must be in one of the following forms:
    ///
    /// * `projects/{project_number}/settings/{setting_name}`
    /// * `folders/{folder_id}/settings/{setting_name}`
    /// * `organizations/{organization_id}/settings/{setting_name}`
    ///
    /// For example, "/projects/123/settings/gcp-enableMyFeature"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Metadata about a setting which is not editable by the end user.
    #[prost(message, optional, tag = "7")]
    pub metadata: ::core::option::Option<SettingMetadata>,
    /// The configured value of the setting at the given parent resource (ignoring
    /// the resource hierarchy). The data type of \[Value][google.cloud.resourcesettings.v1.Value\] must always be
    /// consistent with the data type defined in \[Setting.metadata][google.cloud.resourcesettings.v1.Setting.metadata\].
    #[prost(message, optional, tag = "8")]
    pub local_value: ::core::option::Option<Value>,
    /// Output only. The computed effective value of the setting at the given parent resource
    /// (based on the resource hierarchy).
    ///
    /// The effective value evaluates to one of the following options in the given
    /// order (the next option is used if the previous one does not exist):
    ///
    /// 1. the local setting value on the given resource: \[Setting.local_value][google.cloud.resourcesettings.v1.Setting.local_value\]
    /// 2. if one of the given resource's ancestors have a local setting value,
    ///     the local value at the nearest such ancestor
    /// 3. the setting's default value: \[SettingMetadata.default_value][google.cloud.resourcesettings.v1.SettingMetadata.default_value\]
    /// 4. an empty value (defined as a `Value` with all fields unset)
    ///
    /// The data type of \[Value][google.cloud.resourcesettings.v1.Value\] must always be
    /// consistent with the data type defined in \[Setting.metadata][google.cloud.resourcesettings.v1.Setting.metadata\].
    #[prost(message, optional, tag = "9")]
    pub effective_value: ::core::option::Option<Value>,
    /// A fingerprint used for optimistic concurrency. See
    /// \[UpdateSetting][google.cloud.resourcesettings.v1.ResourceSettingsService.UpdateSetting\] for more
    /// details.
    #[prost(string, tag = "10")]
    pub etag: ::prost::alloc::string::String,
}
/// Metadata about a setting which is not editable by the end user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingMetadata {
    /// The human readable name for this setting.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// A detailed description of what this setting does.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// A flag indicating that values of this setting cannot be modified (see
    /// documentation of the specific setting for updates and reasons).
    #[prost(bool, tag = "3")]
    pub read_only: bool,
    /// The data type for this setting.
    #[prost(enumeration = "setting_metadata::DataType", tag = "4")]
    pub data_type: i32,
    /// The value provided by \[Setting.effective_value][google.cloud.resourcesettings.v1.Setting.effective_value\] if no setting value is
    /// explicitly set.
    ///
    /// Note: not all settings have a default value.
    #[prost(message, optional, tag = "5")]
    pub default_value: ::core::option::Option<Value>,
}
/// Nested message and enum types in `SettingMetadata`.
pub mod setting_metadata {
    /// The data type for setting values of this setting. See \[Value][google.cloud.resourcesettings.v1.Value\] for more
    /// details on the available data types.
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
    pub enum DataType {
        /// Unspecified data type.
        Unspecified = 0,
        /// A boolean setting.
        Boolean = 1,
        /// A string setting.
        String = 2,
        /// A string set setting.
        StringSet = 3,
        /// A Enum setting
        EnumValue = 4,
    }
    impl DataType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataType::Unspecified => "DATA_TYPE_UNSPECIFIED",
                DataType::Boolean => "BOOLEAN",
                DataType::String => "STRING",
                DataType::StringSet => "STRING_SET",
                DataType::EnumValue => "ENUM_VALUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATA_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BOOLEAN" => Some(Self::Boolean),
                "STRING" => Some(Self::String),
                "STRING_SET" => Some(Self::StringSet),
                "ENUM_VALUE" => Some(Self::EnumValue),
                _ => None,
            }
        }
    }
}
/// The data in a setting value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// Selects the data type and associated value.
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// A string set value that can hold a set of strings. The maximum length of
    /// each string is 200 characters and there can be a maximum of 50 strings in
    /// the string set.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringSet {
        /// The strings in the set
        #[prost(string, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A enum value that can hold any enum type setting values.
    /// Each enum type is represented by a number, this representation
    /// is stored in the definitions.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumValue {
        /// The value of this enum
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
    }
    /// Selects the data type and associated value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Defines this value as being a boolean value.
        #[prost(bool, tag = "1")]
        BooleanValue(bool),
        /// Defines this value as being a string value.
        #[prost(string, tag = "2")]
        StringValue(::prost::alloc::string::String),
        /// Defines this value as being a StringSet.
        #[prost(message, tag = "3")]
        StringSetValue(StringSet),
        /// Defines this value as being a Enum.
        #[prost(message, tag = "4")]
        EnumValue(EnumValue),
    }
}
/// The request for ListSettings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSettingsRequest {
    /// Required. The Cloud resource that parents the setting. Must be in one of the
    /// following forms:
    ///
    /// * `projects/{project_number}`
    /// * `projects/{project_id}`
    /// * `folders/{folder_id}`
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Unused. The size of the page to be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Unused. A page token used to retrieve the next page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The SettingView for this request.
    #[prost(enumeration = "SettingView", tag = "4")]
    pub view: i32,
}
/// The response from ListSettings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSettingsResponse {
    /// A list of settings that are available at the specified Cloud resource.
    #[prost(message, repeated, tag = "1")]
    pub settings: ::prost::alloc::vec::Vec<Setting>,
    /// Unused. A page token used to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for GetSetting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingRequest {
    /// Required. The name of the setting to get. See \[Setting][google.cloud.resourcesettings.v1.Setting\] for naming
    /// requirements.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The SettingView for this request.
    #[prost(enumeration = "SettingView", tag = "2")]
    pub view: i32,
}
/// The request for UpdateSetting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingRequest {
    /// Required. The setting to update. See \[Setting][google.cloud.resourcesettings.v1.Setting\] for field requirements.
    #[prost(message, optional, tag = "1")]
    pub setting: ::core::option::Option<Setting>,
}
/// View options for Settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SettingView {
    /// The default / unset value.
    /// The API will default to the SETTING_VIEW_BASIC view.
    Unspecified = 0,
    /// Include \[Setting.metadata][google.cloud.resourcesettings.v1.Setting.metadata\], but nothing else.
    /// This is the default value (for both ListSettings and GetSetting).
    Basic = 1,
    /// Include \[Setting.effective_value][google.cloud.resourcesettings.v1.Setting.effective_value\], but nothing else.
    EffectiveValue = 2,
    /// Include \[Setting.local_value][google.cloud.resourcesettings.v1.Setting.local_value\], but nothing else.
    LocalValue = 3,
}
impl SettingView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SettingView::Unspecified => "SETTING_VIEW_UNSPECIFIED",
            SettingView::Basic => "SETTING_VIEW_BASIC",
            SettingView::EffectiveValue => "SETTING_VIEW_EFFECTIVE_VALUE",
            SettingView::LocalValue => "SETTING_VIEW_LOCAL_VALUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SETTING_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "SETTING_VIEW_BASIC" => Some(Self::Basic),
            "SETTING_VIEW_EFFECTIVE_VALUE" => Some(Self::EffectiveValue),
            "SETTING_VIEW_LOCAL_VALUE" => Some(Self::LocalValue),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod resource_settings_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// An interface to interact with resource settings and setting values throughout
    /// the resource hierarchy.
    ///
    /// Services may surface a number of settings for users to control how their
    /// resources behave. Values of settings applied on a given Cloud resource are
    /// evaluated hierarchically and inherited by all descendants of that resource.
    ///
    /// For all requests, returns a `google.rpc.Status` with
    /// `google.rpc.Code.PERMISSION_DENIED` if the IAM check fails or the `parent`
    /// resource is not in a Cloud Organization.
    /// For all requests, returns a `google.rpc.Status` with
    /// `google.rpc.Code.INVALID_ARGUMENT` if the request is malformed.
    #[derive(Debug, Clone)]
    pub struct ResourceSettingsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ResourceSettingsServiceClient<tonic::transport::Channel> {
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
    impl<T> ResourceSettingsServiceClient<T>
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
        ) -> ResourceSettingsServiceClient<InterceptedService<T, F>>
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
            ResourceSettingsServiceClient::new(
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
        /// Lists all the settings that are available on the Cloud resource `parent`.
        pub async fn list_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSettingsRequest>,
        ) -> Result<tonic::Response<super::ListSettingsResponse>, tonic::Status> {
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
                "/google.cloud.resourcesettings.v1.ResourceSettingsService/ListSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a setting.
        ///
        /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
        /// setting does not exist.
        pub async fn get_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSettingRequest>,
        ) -> Result<tonic::Response<super::Setting>, tonic::Status> {
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
                "/google.cloud.resourcesettings.v1.ResourceSettingsService/GetSetting",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a setting.
        ///
        /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
        /// setting does not exist.
        /// Returns a `google.rpc.Status` with `google.rpc.Code.FAILED_PRECONDITION` if
        /// the setting is flagged as read only.
        /// Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag
        /// supplied in the request does not match the persisted etag of the setting
        /// value.
        ///
        /// On success, the response will contain only `name`, `local_value` and
        /// `etag`.  The `metadata` and `effective_value` cannot be updated through
        /// this API.
        ///
        /// Note: the supplied setting will perform a full overwrite of the
        /// `local_value` field.
        pub async fn update_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingRequest>,
        ) -> Result<tonic::Response<super::Setting>, tonic::Status> {
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
                "/google.cloud.resourcesettings.v1.ResourceSettingsService/UpdateSetting",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
