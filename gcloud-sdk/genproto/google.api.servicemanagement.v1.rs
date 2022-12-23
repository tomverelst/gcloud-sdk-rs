/// The full representation of a Service that is managed by
/// Google Service Management.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedService {
    /// The name of the service. See the
    /// \[overview\](<https://cloud.google.com/service-infrastructure/docs/overview>)
    /// for naming requirements.
    #[prost(string, tag = "2")]
    pub service_name: ::prost::alloc::string::String,
    /// ID of the project that produces and owns this service.
    #[prost(string, tag = "3")]
    pub producer_project_id: ::prost::alloc::string::String,
}
/// The metadata associated with a long running operation resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The full name of the resources that this operation is directly
    /// associated with.
    #[prost(string, repeated, tag = "1")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Detailed status information for each step. The order is undetermined.
    #[prost(message, repeated, tag = "2")]
    pub steps: ::prost::alloc::vec::Vec<operation_metadata::Step>,
    /// Percentage of completion of this operation, ranging from 0 to 100.
    #[prost(int32, tag = "3")]
    pub progress_percentage: i32,
    /// The start time of the operation.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// Represents the status of one operation step.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Step {
        /// The short description of the step.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// The status code.
        #[prost(enumeration = "Status", tag = "4")]
        pub status: i32,
    }
    /// Code describes the status of the operation (or one of its steps).
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
    pub enum Status {
        /// Unspecifed code.
        Unspecified = 0,
        /// The operation or step has completed without errors.
        Done = 1,
        /// The operation or step has not started yet.
        NotStarted = 2,
        /// The operation or step is in progress.
        InProgress = 3,
        /// The operation or step has completed with errors. If the operation is
        /// rollbackable, the rollback completed with errors too.
        Failed = 4,
        /// The operation or step has completed with cancellation.
        Cancelled = 5,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Done => "DONE",
                Status::NotStarted => "NOT_STARTED",
                Status::InProgress => "IN_PROGRESS",
                Status::Failed => "FAILED",
                Status::Cancelled => "CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "DONE" => Some(Self::Done),
                "NOT_STARTED" => Some(Self::NotStarted),
                "IN_PROGRESS" => Some(Self::InProgress),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
}
/// Represents a diagnostic message (error or warning)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Diagnostic {
    /// File name and line number of the error or warning.
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    /// The kind of diagnostic information provided.
    #[prost(enumeration = "diagnostic::Kind", tag = "2")]
    pub kind: i32,
    /// Message describing the error or warning.
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Diagnostic`.
pub mod diagnostic {
    /// The kind of diagnostic information possible.
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
    pub enum Kind {
        /// Warnings and errors
        Warning = 0,
        /// Only errors
        Error = 1,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Warning => "WARNING",
                Kind::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WARNING" => Some(Self::Warning),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// Represents a source file which is used to generate the service configuration
/// defined by `google.api.Service`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSource {
    /// A unique ID for a specific instance of this message, typically assigned
    /// by the client for tracking purpose. If empty, the server may choose to
    /// generate one instead.
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
    /// Set of source configuration files that are used to generate a service
    /// configuration (`google.api.Service`).
    #[prost(message, repeated, tag = "2")]
    pub files: ::prost::alloc::vec::Vec<ConfigFile>,
}
/// Generic specification of a source configuration file
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigFile {
    /// The file name of the configuration file (full or relative path).
    #[prost(string, tag = "1")]
    pub file_path: ::prost::alloc::string::String,
    /// The bytes that constitute the file.
    #[prost(bytes = "vec", tag = "3")]
    pub file_contents: ::prost::alloc::vec::Vec<u8>,
    /// The type of configuration file this represents.
    #[prost(enumeration = "config_file::FileType", tag = "4")]
    pub file_type: i32,
}
/// Nested message and enum types in `ConfigFile`.
pub mod config_file {
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
    pub enum FileType {
        /// Unknown file type.
        Unspecified = 0,
        /// YAML-specification of service.
        ServiceConfigYaml = 1,
        /// OpenAPI specification, serialized in JSON.
        OpenApiJson = 2,
        /// OpenAPI specification, serialized in YAML.
        OpenApiYaml = 3,
        /// FileDescriptorSet, generated by protoc.
        ///
        /// To generate, use protoc with imports and source info included.
        /// For an example test.proto file, the following command would put the value
        /// in a new file named out.pb.
        ///
        /// $protoc --include_imports --include_source_info test.proto -o out.pb
        FileDescriptorSetProto = 4,
        /// Uncompiled Proto file. Used for storage and display purposes only,
        /// currently server-side compilation is not supported. Should match the
        /// inputs to 'protoc' command used to generated FILE_DESCRIPTOR_SET_PROTO. A
        /// file of this type can only be included if at least one file of type
        /// FILE_DESCRIPTOR_SET_PROTO is included.
        ProtoFile = 6,
    }
    impl FileType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FileType::Unspecified => "FILE_TYPE_UNSPECIFIED",
                FileType::ServiceConfigYaml => "SERVICE_CONFIG_YAML",
                FileType::OpenApiJson => "OPEN_API_JSON",
                FileType::OpenApiYaml => "OPEN_API_YAML",
                FileType::FileDescriptorSetProto => "FILE_DESCRIPTOR_SET_PROTO",
                FileType::ProtoFile => "PROTO_FILE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FILE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SERVICE_CONFIG_YAML" => Some(Self::ServiceConfigYaml),
                "OPEN_API_JSON" => Some(Self::OpenApiJson),
                "OPEN_API_YAML" => Some(Self::OpenApiYaml),
                "FILE_DESCRIPTOR_SET_PROTO" => Some(Self::FileDescriptorSetProto),
                "PROTO_FILE" => Some(Self::ProtoFile),
                _ => None,
            }
        }
    }
}
/// Represents a service configuration with its name and id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRef {
    /// Resource name of a service config. It must have the following
    /// format: "services/{service name}/configs/{config id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Change report associated with a particular service configuration.
///
/// It contains a list of ConfigChanges based on the comparison between
/// two service configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeReport {
    /// List of changes between two service configurations.
    /// The changes will be alphabetically sorted based on the identifier
    /// of each change.
    /// A ConfigChange identifier is a dot separated path to the configuration.
    /// Example: visibility.rules\[selector='LibraryService.CreateBook'\].restriction
    #[prost(message, repeated, tag = "1")]
    pub config_changes: ::prost::alloc::vec::Vec<super::super::ConfigChange>,
}
/// A rollout resource that defines how service configuration versions are pushed
/// to control plane systems. Typically, you create a new version of the
/// service config, and then create a Rollout to push the service config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rollout {
    /// Optional. Unique identifier of this Rollout. Must be no longer than 63
    /// characters and only lower case letters, digits, '.', '_' and '-' are
    /// allowed.
    ///
    /// If not specified by client, the server will generate one. The generated id
    /// will have the form of <date><revision number>, where "date" is the create
    /// date in ISO 8601 format.  "revision number" is a monotonically increasing
    /// positive number that is reset every day for each service.
    /// An example of the generated rollout_id is '2016-02-16r1'
    #[prost(string, tag = "1")]
    pub rollout_id: ::prost::alloc::string::String,
    /// Creation time of the rollout. Readonly.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The user who created the Rollout. Readonly.
    #[prost(string, tag = "3")]
    pub created_by: ::prost::alloc::string::String,
    /// The status of this rollout. Readonly. In case of a failed rollout,
    /// the system will automatically rollback to the current Rollout
    /// version. Readonly.
    #[prost(enumeration = "rollout::RolloutStatus", tag = "4")]
    pub status: i32,
    /// The name of the service associated with this Rollout.
    #[prost(string, tag = "8")]
    pub service_name: ::prost::alloc::string::String,
    /// Strategy that defines which versions of service configurations should be
    /// pushed
    /// and how they should be used at runtime.
    #[prost(oneof = "rollout::Strategy", tags = "5, 200")]
    pub strategy: ::core::option::Option<rollout::Strategy>,
}
/// Nested message and enum types in `Rollout`.
pub mod rollout {
    /// Strategy that specifies how clients of Google Service Controller want to
    /// send traffic to use different config versions. This is generally
    /// used by API proxy to split traffic based on your configured percentage for
    /// each config version.
    ///
    /// One example of how to gradually rollout a new service configuration using
    /// this
    /// strategy:
    /// Day 1
    ///
    ///      Rollout {
    ///        id: "example.googleapis.com/rollout_20160206"
    ///        traffic_percent_strategy {
    ///          percentages: {
    ///            "example.googleapis.com/20160201": 70.00
    ///            "example.googleapis.com/20160206": 30.00
    ///          }
    ///        }
    ///      }
    ///
    /// Day 2
    ///
    ///      Rollout {
    ///        id: "example.googleapis.com/rollout_20160207"
    ///        traffic_percent_strategy: {
    ///          percentages: {
    ///            "example.googleapis.com/20160206": 100.00
    ///          }
    ///        }
    ///      }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrafficPercentStrategy {
        /// Maps service configuration IDs to their corresponding traffic percentage.
        /// Key is the service configuration ID, Value is the traffic percentage
        /// which must be greater than 0.0 and the sum must equal to 100.0.
        #[prost(map = "string, double", tag = "1")]
        pub percentages: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            f64,
        >,
    }
    /// Strategy used to delete a service. This strategy is a placeholder only
    /// used by the system generated rollout to delete a service.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeleteServiceStrategy {}
    /// Status of a Rollout.
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
    pub enum RolloutStatus {
        /// No status specified.
        Unspecified = 0,
        /// The Rollout is in progress.
        InProgress = 1,
        /// The Rollout has completed successfully.
        Success = 2,
        /// The Rollout has been cancelled. This can happen if you have overlapping
        /// Rollout pushes, and the previous ones will be cancelled.
        Cancelled = 3,
        /// The Rollout has failed and the rollback attempt has failed too.
        Failed = 4,
        /// The Rollout has not started yet and is pending for execution.
        Pending = 5,
        /// The Rollout has failed and rolled back to the previous successful
        /// Rollout.
        FailedRolledBack = 6,
    }
    impl RolloutStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RolloutStatus::Unspecified => "ROLLOUT_STATUS_UNSPECIFIED",
                RolloutStatus::InProgress => "IN_PROGRESS",
                RolloutStatus::Success => "SUCCESS",
                RolloutStatus::Cancelled => "CANCELLED",
                RolloutStatus::Failed => "FAILED",
                RolloutStatus::Pending => "PENDING",
                RolloutStatus::FailedRolledBack => "FAILED_ROLLED_BACK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLLOUT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "SUCCESS" => Some(Self::Success),
                "CANCELLED" => Some(Self::Cancelled),
                "FAILED" => Some(Self::Failed),
                "PENDING" => Some(Self::Pending),
                "FAILED_ROLLED_BACK" => Some(Self::FailedRolledBack),
                _ => None,
            }
        }
    }
    /// Strategy that defines which versions of service configurations should be
    /// pushed
    /// and how they should be used at runtime.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Strategy {
        /// Google Service Control selects service configurations based on
        /// traffic percentage.
        #[prost(message, tag = "5")]
        TrafficPercentStrategy(TrafficPercentStrategy),
        /// The strategy associated with a rollout to delete a `ManagedService`.
        /// Readonly.
        #[prost(message, tag = "200")]
        DeleteServiceStrategy(DeleteServiceStrategy),
    }
}
/// Request message for `ListServices` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Include services produced by the specified project.
    #[prost(string, tag = "1")]
    pub producer_project_id: ::prost::alloc::string::String,
    /// The max number of items to include in the response list. Page size is 50
    /// if not specified. Maximum value is 100.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// Token identifying which result to start with; returned by a previous list
    /// call.
    #[prost(string, tag = "6")]
    pub page_token: ::prost::alloc::string::String,
    /// Include services consumed by the specified consumer.
    ///
    /// The Google Service Management implementation accepts the following
    /// forms:
    /// - project:<project_id>
    #[deprecated]
    #[prost(string, tag = "7")]
    pub consumer_id: ::prost::alloc::string::String,
}
/// Response message for `ListServices` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The returned services will only have the name field set.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<ManagedService>,
    /// Token that can be passed to `ListServices` to resume a paginated query.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `GetService` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Required. The name of the service.  See the `ServiceManager` overview for
    /// naming requirements.  For example: `example.googleapis.com`.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
}
/// Request message for CreateService method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Required. Initial values for the service resource.
    #[prost(message, optional, tag = "1")]
    pub service: ::core::option::Option<ManagedService>,
}
/// Request message for DeleteService method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Required. The name of the service.  See the
    /// \[overview\](<https://cloud.google.com/service-infrastructure/docs/overview>) for naming requirements.  For
    /// example: `example.googleapis.com`.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
}
/// Request message for UndeleteService method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteServiceRequest {
    /// Required. The name of the service. See the
    /// \[overview\](<https://cloud.google.com/service-infrastructure/docs/overview>) for naming requirements. For
    /// example: `example.googleapis.com`.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
}
/// Response message for UndeleteService method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteServiceResponse {
    /// Revived service resource.
    #[prost(message, optional, tag = "1")]
    pub service: ::core::option::Option<ManagedService>,
}
/// Request message for GetServiceConfig method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceConfigRequest {
    /// Required. The name of the service.  See the
    /// \[overview\](<https://cloud.google.com/service-infrastructure/docs/overview>) for naming requirements.  For
    /// example: `example.googleapis.com`.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// Required. The id of the service configuration resource.
    ///
    /// This field must be specified for the server to return all fields, including
    /// `SourceInfo`.
    #[prost(string, tag = "2")]
    pub config_id: ::prost::alloc::string::String,
    /// Specifies which parts of the Service Config should be returned in the
    /// response.
    #[prost(enumeration = "get_service_config_request::ConfigView", tag = "3")]
    pub view: i32,
}
/// Nested message and enum types in `GetServiceConfigRequest`.
pub mod get_service_config_request {
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
    pub enum ConfigView {
        /// Server response includes all fields except SourceInfo.
        Basic = 0,
        /// Server response includes all fields including SourceInfo.
        /// SourceFiles are of type 'google.api.servicemanagement.v1.ConfigFile'
        /// and are only available for configs created using the
        /// SubmitConfigSource method.
        Full = 1,
    }
    impl ConfigView {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConfigView::Basic => "BASIC",
                ConfigView::Full => "FULL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BASIC" => Some(Self::Basic),
                "FULL" => Some(Self::Full),
                _ => None,
            }
        }
    }
}
/// Request message for ListServiceConfigs method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceConfigsRequest {
    /// Required. The name of the service.  See the
    /// \[overview\](<https://cloud.google.com/service-infrastructure/docs/overview>) for naming requirements.  For
    /// example: `example.googleapis.com`.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// The token of the page to retrieve.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// The max number of items to include in the response list. Page size is 50
    /// if not specified. Maximum value is 100.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for ListServiceConfigs method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceConfigsResponse {
    /// The list of service configuration resources.
    #[prost(message, repeated, tag = "1")]
    pub service_configs: ::prost::alloc::vec::Vec<super::super::Service>,
    /// The token of the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateServiceConfig method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceConfigRequest {
    /// Required. The name of the service.  See the
    /// \[overview\](<https://cloud.google.com/service-infrastructure/docs/overview>) for naming requirements.  For
    /// example: `example.googleapis.com`.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// Required. The service configuration resource.
    #[prost(message, optional, tag = "2")]
    pub service_config: ::core::option::Option<super::super::Service>,
}
/// Request message for SubmitConfigSource method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitConfigSourceRequest {
    /// Required. The name of the service.  See the
    /// \[overview\](<https://cloud.google.com/service-infrastructure/docs/overview>) for naming requirements.  For
    /// example: `example.googleapis.com`.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// Required. The source configuration for the service.
    #[prost(message, optional, tag = "2")]
    pub config_source: ::core::option::Option<ConfigSource>,
    /// Optional. If set, this will result in the generation of a
    /// `google.api.Service` configuration based on the `ConfigSource` provided,
    /// but the generated config and the sources will NOT be persisted.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Response message for SubmitConfigSource method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitConfigSourceResponse {
    /// The generated service configuration.
    #[prost(message, optional, tag = "1")]
    pub service_config: ::core::option::Option<super::super::Service>,
}
/// Request message for 'CreateServiceRollout'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRolloutRequest {
    /// Required. The name of the service.  See the
    /// \[overview\](<https://cloud.google.com/service-infrastructure/docs/overview>) for naming requirements.  For
    /// example: `example.googleapis.com`.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// Required. The rollout resource. The `service_name` field is output only.
    #[prost(message, optional, tag = "2")]
    pub rollout: ::core::option::Option<Rollout>,
}
/// Request message for 'ListServiceRollouts'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceRolloutsRequest {
    /// Required. The name of the service.  See the
    /// \[overview\](<https://cloud.google.com/service-infrastructure/docs/overview>) for naming requirements.  For
    /// example: `example.googleapis.com`.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// The token of the page to retrieve.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// The max number of items to include in the response list. Page size is 50
    /// if not specified. Maximum value is 100.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Required. Use `filter` to return subset of rollouts.
    /// The following filters are supported:
    ///    -- To limit the results to only those in
    ///       status (google.api.servicemanagement.v1.RolloutStatus) 'SUCCESS',
    ///       use filter='status=SUCCESS'
    ///    -- To limit the results to those in
    ///       status (google.api.servicemanagement.v1.RolloutStatus) 'CANCELLED'
    ///       or 'FAILED', use filter='status=CANCELLED OR status=FAILED'
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for ListServiceRollouts method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceRolloutsResponse {
    /// The list of rollout resources.
    #[prost(message, repeated, tag = "1")]
    pub rollouts: ::prost::alloc::vec::Vec<Rollout>,
    /// The token of the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetServiceRollout method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRolloutRequest {
    /// Required. The name of the service.  See the
    /// \[overview\](<https://cloud.google.com/service-infrastructure/docs/overview>) for naming requirements.  For
    /// example: `example.googleapis.com`.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// Required. The id of the rollout resource.
    #[prost(string, tag = "2")]
    pub rollout_id: ::prost::alloc::string::String,
}
/// Request message for GenerateConfigReport method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateConfigReportRequest {
    /// Required. Service configuration for which we want to generate the report.
    /// For this version of API, the supported types are
    /// \[google.api.servicemanagement.v1.ConfigRef][google.api.servicemanagement.v1.ConfigRef\],
    /// \[google.api.servicemanagement.v1.ConfigSource][google.api.servicemanagement.v1.ConfigSource\],
    /// and \[google.api.Service][google.api.Service\]
    #[prost(message, optional, tag = "1")]
    pub new_config: ::core::option::Option<::prost_types::Any>,
    /// Optional. Service configuration against which the comparison will be done.
    /// For this version of API, the supported types are
    /// \[google.api.servicemanagement.v1.ConfigRef][google.api.servicemanagement.v1.ConfigRef\],
    /// \[google.api.servicemanagement.v1.ConfigSource][google.api.servicemanagement.v1.ConfigSource\],
    /// and \[google.api.Service][google.api.Service\]
    #[prost(message, optional, tag = "2")]
    pub old_config: ::core::option::Option<::prost_types::Any>,
}
/// Response message for GenerateConfigReport method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateConfigReportResponse {
    /// Name of the service this report belongs to.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// ID of the service configuration this report belongs to.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// list of ChangeReport, each corresponding to comparison between two
    /// service configurations.
    #[prost(message, repeated, tag = "3")]
    pub change_reports: ::prost::alloc::vec::Vec<ChangeReport>,
    /// Errors / Linter warnings associated with the service definition this
    /// report
    /// belongs to.
    #[prost(message, repeated, tag = "4")]
    pub diagnostics: ::prost::alloc::vec::Vec<Diagnostic>,
}
/// Generated client implementations.
pub mod service_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// [Google Service Management
    /// API](https://cloud.google.com/service-infrastructure/docs/overview)
    #[derive(Debug, Clone)]
    pub struct ServiceManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServiceManagerClient<tonic::transport::Channel> {
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
    impl<T> ServiceManagerClient<T>
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
        ) -> ServiceManagerClient<InterceptedService<T, F>>
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
            ServiceManagerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists managed services.
        ///
        /// Returns all public services. For authenticated users, also returns all
        /// services the calling user has "servicemanagement.services.get" permission
        /// for.
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/ListServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a managed service. Authentication is required unless the service is
        /// public.
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::ManagedService>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/GetService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new managed service.
        ///
        /// A managed service is immutable, and is subject to mandatory 30-day
        /// data retention. You cannot move a service or recreate it within 30 days
        /// after deletion.
        ///
        /// One producer project can own no more than 500 services. For security and
        /// reliability purposes, a production service should be hosted in a
        /// dedicated producer project.
        ///
        /// Operation<response: ManagedService>
        pub async fn create_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.servicemanagement.v1.ServiceManager/CreateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a managed service. This method will change the service to the
        /// `Soft-Delete` state for 30 days. Within this period, service producers may
        /// call
        /// [UndeleteService][google.api.servicemanagement.v1.ServiceManager.UndeleteService]
        /// to restore the service. After 30 days, the service will be permanently
        /// deleted.
        ///
        /// Operation<response: google.protobuf.Empty>
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.servicemanagement.v1.ServiceManager/DeleteService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Revives a previously deleted managed service. The method restores the
        /// service using the configuration at the time the service was deleted.
        /// The target service must exist and must have been deleted within the
        /// last 30 days.
        ///
        /// Operation<response: UndeleteServiceResponse>
        pub async fn undelete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.servicemanagement.v1.ServiceManager/UndeleteService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the history of the service configuration for a managed service,
        /// from the newest to the oldest.
        pub async fn list_service_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServiceConfigsRequest>,
        ) -> Result<tonic::Response<super::ListServiceConfigsResponse>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/ListServiceConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a service configuration (version) for a managed service.
        pub async fn get_service_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceConfigRequest>,
        ) -> Result<tonic::Response<super::super::super::Service>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/GetServiceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new service configuration (version) for a managed service.
        /// This method only stores the service configuration. To roll out the service
        /// configuration to backend systems please call
        /// [CreateServiceRollout][google.api.servicemanagement.v1.ServiceManager.CreateServiceRollout].
        ///
        /// Only the 100 most recent service configurations and ones referenced by
        /// existing rollouts are kept for each service. The rest will be deleted
        /// eventually.
        pub async fn create_service_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceConfigRequest>,
        ) -> Result<tonic::Response<super::super::super::Service>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/CreateServiceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new service configuration (version) for a managed service based
        /// on
        /// user-supplied configuration source files (for example: OpenAPI
        /// Specification). This method stores the source configurations as well as the
        /// generated service configuration. To rollout the service configuration to
        /// other services,
        /// please call
        /// [CreateServiceRollout][google.api.servicemanagement.v1.ServiceManager.CreateServiceRollout].
        ///
        /// Only the 100 most recent configuration sources and ones referenced by
        /// existing service configurtions are kept for each service. The rest will be
        /// deleted eventually.
        ///
        /// Operation<response: SubmitConfigSourceResponse>
        pub async fn submit_config_source(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitConfigSourceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.servicemanagement.v1.ServiceManager/SubmitConfigSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the history of the service configuration rollouts for a managed
        /// service, from the newest to the oldest.
        pub async fn list_service_rollouts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServiceRolloutsRequest>,
        ) -> Result<tonic::Response<super::ListServiceRolloutsResponse>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/ListServiceRollouts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a service configuration
        /// [rollout][google.api.servicemanagement.v1.Rollout].
        pub async fn get_service_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRolloutRequest>,
        ) -> Result<tonic::Response<super::Rollout>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/GetServiceRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new service configuration rollout. Based on rollout, the
        /// Google Service Management will roll out the service configurations to
        /// different backend services. For example, the logging configuration will be
        /// pushed to Google Cloud Logging.
        ///
        /// Please note that any previous pending and running Rollouts and associated
        /// Operations will be automatically cancelled so that the latest Rollout will
        /// not be blocked by previous Rollouts.
        ///
        /// Only the 100 most recent (in any state) and the last 10 successful (if not
        /// already part of the set of 100 most recent) rollouts are kept for each
        /// service. The rest will be deleted eventually.
        ///
        /// Operation<response: Rollout>
        pub async fn create_service_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRolloutRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.servicemanagement.v1.ServiceManager/CreateServiceRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Generates and returns a report (errors, warnings and changes from
        /// existing configurations) associated with
        /// GenerateConfigReportRequest.new_value
        ///
        /// If GenerateConfigReportRequest.old_value is specified,
        /// GenerateConfigReportRequest will contain a single ChangeReport based on the
        /// comparison between GenerateConfigReportRequest.new_value and
        /// GenerateConfigReportRequest.old_value.
        /// If GenerateConfigReportRequest.old_value is not specified, this method
        /// will compare GenerateConfigReportRequest.new_value with the last pushed
        /// service configuration.
        pub async fn generate_config_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateConfigReportRequest>,
        ) -> Result<
            tonic::Response<super::GenerateConfigReportResponse>,
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
                "/google.api.servicemanagement.v1.ServiceManager/GenerateConfigReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
