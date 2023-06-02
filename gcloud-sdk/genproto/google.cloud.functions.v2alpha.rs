/// Describes a Cloud Function that contains user computation executed in
/// response to an event. It encapsulates function and trigger configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Function {
    /// A user-defined name of the function. Function names must be unique
    /// globally and match pattern `projects/*/locations/*/functions/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User-provided description of a function.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Describes the Build step of the function that builds a container from the
    /// given source.
    #[prost(message, optional, tag = "3")]
    pub build_config: ::core::option::Option<BuildConfig>,
    /// Describes the Service being deployed. Currently deploys services to Cloud
    /// Run (fully managed).
    #[prost(message, optional, tag = "4")]
    pub service_config: ::core::option::Option<ServiceConfig>,
    /// An Eventarc trigger managed by Google Cloud Functions that fires events in
    /// response to a condition in another service.
    #[prost(message, optional, tag = "5")]
    pub event_trigger: ::core::option::Option<EventTrigger>,
    /// Output only. State of the function.
    #[prost(enumeration = "function::State", tag = "6")]
    pub state: i32,
    /// Output only. The last update timestamp of a Cloud Function.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this Cloud Function.
    #[prost(map = "string, string", tag = "8")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. State Messages for this Cloud Function.
    #[prost(message, repeated, tag = "9")]
    pub state_messages: ::prost::alloc::vec::Vec<StateMessage>,
    /// Describe whether the function is 1st Gen or 2nd Gen.
    #[prost(enumeration = "Environment", tag = "10")]
    pub environment: i32,
    /// Output only. The deployed url for the function.
    #[prost(string, tag = "14")]
    pub url: ::prost::alloc::string::String,
    /// \[Preview\] Resource name of a KMS crypto key (managed by the user) used to
    /// encrypt/decrypt function resources.
    ///
    /// It must match the pattern
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    #[prost(string, tag = "25")]
    pub kms_key_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Function`.
pub mod function {
    /// Describes the current state of the function.
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
    pub enum State {
        /// Not specified. Invalid state.
        Unspecified = 0,
        /// Function has been successfully deployed and is serving.
        Active = 1,
        /// Function deployment failed and the function is not serving.
        Failed = 2,
        /// Function is being created or updated.
        Deploying = 3,
        /// Function is being deleted.
        Deleting = 4,
        /// Function deployment failed and the function serving state is undefined.
        /// The function should be updated or deleted to move it out of this state.
        Unknown = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Failed => "FAILED",
                State::Deploying => "DEPLOYING",
                State::Deleting => "DELETING",
                State::Unknown => "UNKNOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "FAILED" => Some(Self::Failed),
                "DEPLOYING" => Some(Self::Deploying),
                "DELETING" => Some(Self::Deleting),
                "UNKNOWN" => Some(Self::Unknown),
                _ => None,
            }
        }
    }
}
/// Informational messages about the state of the Cloud Function or Operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateMessage {
    /// Severity of the state message.
    #[prost(enumeration = "state_message::Severity", tag = "1")]
    pub severity: i32,
    /// One-word CamelCase type of the state message.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// The message.
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StateMessage`.
pub mod state_message {
    /// Severity of the state message.
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
    pub enum Severity {
        /// Not specified. Invalid severity.
        Unspecified = 0,
        /// ERROR-level severity.
        Error = 1,
        /// WARNING-level severity.
        Warning = 2,
        /// INFO-level severity.
        Info = 3,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Error => "ERROR",
                Severity::Warning => "WARNING",
                Severity::Info => "INFO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "ERROR" => Some(Self::Error),
                "WARNING" => Some(Self::Warning),
                "INFO" => Some(Self::Info),
                _ => None,
            }
        }
    }
}
/// Location of the source in an archive file in Google Cloud Storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageSource {
    /// Google Cloud Storage bucket containing the source (see
    /// [Bucket Name
    /// Requirements](<https://cloud.google.com/storage/docs/bucket-naming#requirements>)).
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// Google Cloud Storage object containing the source.
    ///
    /// This object must be a gzipped archive file (`.tar.gz`) containing source to
    /// build.
    #[prost(string, tag = "2")]
    pub object: ::prost::alloc::string::String,
    /// Google Cloud Storage generation for the object. If the generation is
    /// omitted, the latest generation will be used.
    #[prost(int64, tag = "3")]
    pub generation: i64,
}
/// Location of the source in a Google Cloud Source Repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepoSource {
    /// ID of the project that owns the Cloud Source Repository. If omitted, the
    /// project ID requesting the build is assumed.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Name of the Cloud Source Repository.
    #[prost(string, tag = "2")]
    pub repo_name: ::prost::alloc::string::String,
    /// Directory, relative to the source root, in which to run the build.
    ///
    /// This must be a relative path. If a step's `dir` is specified and is an
    /// absolute path, this value is ignored for that step's execution.
    /// eg. helloworld (no leading slash allowed)
    #[prost(string, tag = "6")]
    pub dir: ::prost::alloc::string::String,
    /// Only trigger a build if the revision regex does NOT match the revision
    /// regex.
    #[prost(bool, tag = "7")]
    pub invert_regex: bool,
    /// A revision within the Cloud Source Repository must be specified in
    /// one of these ways.
    #[prost(oneof = "repo_source::Revision", tags = "3, 4, 5")]
    pub revision: ::core::option::Option<repo_source::Revision>,
}
/// Nested message and enum types in `RepoSource`.
pub mod repo_source {
    /// A revision within the Cloud Source Repository must be specified in
    /// one of these ways.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Revision {
        /// Regex matching branches to build.
        ///
        /// The syntax of the regular expressions accepted is the syntax accepted by
        /// RE2 and described at <https://github.com/google/re2/wiki/Syntax>
        #[prost(string, tag = "3")]
        BranchName(::prost::alloc::string::String),
        /// Regex matching tags to build.
        ///
        /// The syntax of the regular expressions accepted is the syntax accepted by
        /// RE2 and described at <https://github.com/google/re2/wiki/Syntax>
        #[prost(string, tag = "4")]
        TagName(::prost::alloc::string::String),
        /// Explicit commit SHA to build.
        #[prost(string, tag = "5")]
        CommitSha(::prost::alloc::string::String),
    }
}
/// The location of the function source code.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// Location of the source.
    /// At least one source needs to be provided for the deployment to succeed.
    #[prost(oneof = "source::Source", tags = "1, 2")]
    pub source: ::core::option::Option<source::Source>,
}
/// Nested message and enum types in `Source`.
pub mod source {
    /// Location of the source.
    /// At least one source needs to be provided for the deployment to succeed.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// If provided, get the source from this location in Google Cloud Storage.
        #[prost(message, tag = "1")]
        StorageSource(super::StorageSource),
        /// If provided, get the source from this location in a Cloud Source
        /// Repository.
        #[prost(message, tag = "2")]
        RepoSource(super::RepoSource),
    }
}
/// Provenance of the source. Ways to find the original source, or verify that
/// some source was used for this build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceProvenance {
    /// A copy of the build's `source.storage_source`, if exists, with any
    /// generations resolved.
    #[prost(message, optional, tag = "1")]
    pub resolved_storage_source: ::core::option::Option<StorageSource>,
    /// A copy of the build's `source.repo_source`, if exists, with any
    /// revisions resolved.
    #[prost(message, optional, tag = "2")]
    pub resolved_repo_source: ::core::option::Option<RepoSource>,
}
/// Describes the Build step of the function that builds a container from the
/// given source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildConfig {
    /// Output only. The Cloud Build name of the latest successful deployment of
    /// the function.
    #[prost(string, tag = "1")]
    pub build: ::prost::alloc::string::String,
    /// The runtime in which to run the function. Required when deploying a new
    /// function, optional when updating an existing function. For a complete
    /// list of possible choices, see the
    /// [`gcloud` command
    /// reference](<https://cloud.google.com/sdk/gcloud/reference/functions/deploy#--runtime>).
    #[prost(string, tag = "2")]
    pub runtime: ::prost::alloc::string::String,
    /// The name of the function (as defined in source code) that will be
    /// executed. Defaults to the resource name suffix, if not specified. For
    /// backward compatibility, if function with given name is not found, then the
    /// system will try to use function named "function".
    /// For Node.js this is name of a function exported by the module specified
    /// in `source_location`.
    #[prost(string, tag = "3")]
    pub entry_point: ::prost::alloc::string::String,
    /// The location of the function source code.
    #[prost(message, optional, tag = "4")]
    pub source: ::core::option::Option<Source>,
    /// Output only. A permanent fixed identifier for source.
    #[prost(message, optional, tag = "8")]
    pub source_provenance: ::core::option::Option<SourceProvenance>,
    /// Name of the Cloud Build Custom Worker Pool that should be used to build the
    /// function. The format of this field is
    /// `projects/{project}/locations/{region}/workerPools/{workerPool}` where
    /// {project} and {region} are the project id and region respectively where the
    /// worker pool is defined and {workerPool} is the short name of the worker
    /// pool.
    ///
    /// If the project id is not the same as the function, then the Cloud
    /// Functions Service Agent
    /// (service-<project_number>@gcf-admin-robot.iam.gserviceaccount.com) must be
    /// granted the role Cloud Build Custom Workers Builder
    /// (roles/cloudbuild.customworkers.builder) in the project.
    #[prost(string, tag = "5")]
    pub worker_pool: ::prost::alloc::string::String,
    /// User-provided build-time environment variables for the function
    #[prost(map = "string, string", tag = "6")]
    pub environment_variables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Docker Registry to use for this deployment. This configuration is only
    /// applicable to 1st Gen functions, 2nd Gen functions can only use Artifact
    /// Registry.
    ///
    /// If `docker_repository` field is specified, this field will be automatically
    /// set as `ARTIFACT_REGISTRY`.
    /// If unspecified, it currently defaults to `CONTAINER_REGISTRY`.
    /// This field may be overridden by the backend for eligible deployments.
    #[prost(enumeration = "build_config::DockerRegistry", tag = "10")]
    pub docker_registry: i32,
    /// User managed repository created in Artifact Registry optionally
    /// with a customer managed encryption key. This is the repository to which the
    /// function docker image will be pushed after it is built by Cloud Build.
    /// If unspecified, GCF will create and use a repository named 'gcf-artifacts'
    /// for every deployed region.
    ///
    /// It must match the pattern
    /// `projects/{project}/locations/{location}/repositories/{repository}`.
    ///
    /// Cross-project repositories are not supported.
    /// Cross-location repositories are not supported.
    /// Repository format must be 'DOCKER'.
    #[prost(string, tag = "7")]
    pub docker_repository: ::prost::alloc::string::String,
}
/// Nested message and enum types in `BuildConfig`.
pub mod build_config {
    /// Docker Registry to use for storing function Docker images.
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
    pub enum DockerRegistry {
        /// Unspecified.
        Unspecified = 0,
        /// Docker images will be stored in multi-regional Container Registry
        /// repositories named `gcf`.
        ContainerRegistry = 1,
        /// Docker images will be stored in regional Artifact Registry repositories.
        /// By default, GCF will create and use repositories named `gcf-artifacts`
        /// in every region in which a function is deployed. But the repository to
        /// use can also be specified by the user using the `docker_repository`
        /// field.
        ArtifactRegistry = 2,
    }
    impl DockerRegistry {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DockerRegistry::Unspecified => "DOCKER_REGISTRY_UNSPECIFIED",
                DockerRegistry::ContainerRegistry => "CONTAINER_REGISTRY",
                DockerRegistry::ArtifactRegistry => "ARTIFACT_REGISTRY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DOCKER_REGISTRY_UNSPECIFIED" => Some(Self::Unspecified),
                "CONTAINER_REGISTRY" => Some(Self::ContainerRegistry),
                "ARTIFACT_REGISTRY" => Some(Self::ArtifactRegistry),
                _ => None,
            }
        }
    }
}
/// Describes the Service being deployed.
/// Currently Supported : Cloud Run (fully managed).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceConfig {
    /// Output only. Name of the service associated with a Function.
    /// The format of this field is
    /// `projects/{project}/locations/{region}/services/{service}`
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// The function execution timeout. Execution is considered failed and
    /// can be terminated if the function is not completed at the end of the
    /// timeout period. Defaults to 60 seconds.
    #[prost(int32, tag = "2")]
    pub timeout_seconds: i32,
    /// The amount of memory available for a function.
    /// Defaults to 256M. Supported units are k, M, G, Mi, Gi. If no unit is
    /// supplied the value is interpreted as bytes.
    /// See
    /// <https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go>
    /// a full description.
    #[prost(string, tag = "13")]
    pub available_memory: ::prost::alloc::string::String,
    /// \[Preview\] The number of CPUs used in a single container instance.
    /// Default value is calculated from available memory.
    /// Supports the same values as Cloud Run, see
    /// <https://cloud.google.com/run/docs/reference/rest/v1/Container#resourcerequirements>
    /// Example: "1" indicates 1 vCPU
    #[prost(string, tag = "22")]
    pub available_cpu: ::prost::alloc::string::String,
    /// Environment variables that shall be available during function execution.
    #[prost(map = "string, string", tag = "4")]
    pub environment_variables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The limit on the maximum number of function instances that may coexist at a
    /// given time.
    ///
    /// In some cases, such as rapid traffic surges, Cloud Functions may, for a
    /// short period of time, create more instances than the specified max
    /// instances limit. If your function cannot tolerate this temporary behavior,
    /// you may want to factor in a safety margin and set a lower max instances
    /// value than your function can tolerate.
    ///
    /// See the [Max
    /// Instances](<https://cloud.google.com/functions/docs/max-instances>) Guide for
    /// more details.
    #[prost(int32, tag = "5")]
    pub max_instance_count: i32,
    /// The limit on the minimum number of function instances that may coexist at a
    /// given time.
    ///
    /// Function instances are kept in idle state for a short period after they
    /// finished executing the request to reduce cold start time for subsequent
    /// requests. Setting a minimum instance count will ensure that the given
    /// number of instances are kept running in idle state always. This can help
    /// with cold start times when jump in incoming request count occurs after the
    /// idle instance would have been stopped in the default case.
    #[prost(int32, tag = "12")]
    pub min_instance_count: i32,
    /// The Serverless VPC Access connector that this cloud function can connect
    /// to. The format of this field is `projects/*/locations/*/connectors/*`.
    #[prost(string, tag = "6")]
    pub vpc_connector: ::prost::alloc::string::String,
    /// The egress settings for the connector, controlling what traffic is diverted
    /// through it.
    #[prost(enumeration = "service_config::VpcConnectorEgressSettings", tag = "7")]
    pub vpc_connector_egress_settings: i32,
    /// The ingress settings for the function, controlling what traffic can reach
    /// it.
    #[prost(enumeration = "service_config::IngressSettings", tag = "8")]
    pub ingress_settings: i32,
    /// Output only. URI of the Service deployed.
    #[prost(string, tag = "9")]
    pub uri: ::prost::alloc::string::String,
    /// The email of the service's service account. If empty, defaults to
    /// `{project_number}-compute@developer.gserviceaccount.com`.
    #[prost(string, tag = "10")]
    pub service_account_email: ::prost::alloc::string::String,
    /// Whether 100% of traffic is routed to the latest revision.
    /// On CreateFunction and UpdateFunction, when set to true, the revision being
    /// deployed will serve 100% of traffic, ignoring any traffic split settings,
    /// if any. On GetFunction, true will be returned if the latest revision is
    /// serving 100% of traffic.
    #[prost(bool, tag = "16")]
    pub all_traffic_on_latest_revision: bool,
    /// Secret environment variables configuration.
    #[prost(message, repeated, tag = "17")]
    pub secret_environment_variables: ::prost::alloc::vec::Vec<SecretEnvVar>,
    /// Secret volumes configuration.
    #[prost(message, repeated, tag = "19")]
    pub secret_volumes: ::prost::alloc::vec::Vec<SecretVolume>,
    /// Output only. The name of service revision.
    #[prost(string, tag = "18")]
    pub revision: ::prost::alloc::string::String,
    /// \[Preview\] Sets the maximum number of concurrent requests that each instance
    /// can receive. Defaults to 1.
    #[prost(int32, tag = "20")]
    pub max_instance_request_concurrency: i32,
    /// Security level configure whether the function only accepts https.
    /// This configuration is only applicable to 1st Gen functions with Http
    /// trigger. By default https is optional for 1st Gen functions; 2nd Gen
    /// functions are https ONLY.
    #[prost(enumeration = "service_config::SecurityLevel", tag = "21")]
    pub security_level: i32,
}
/// Nested message and enum types in `ServiceConfig`.
pub mod service_config {
    /// Available egress settings.
    ///
    /// This controls what traffic is diverted through the VPC Access Connector
    /// resource. By default PRIVATE_RANGES_ONLY will be used.
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
    pub enum VpcConnectorEgressSettings {
        /// Unspecified.
        Unspecified = 0,
        /// Use the VPC Access Connector only for private IP space from RFC1918.
        PrivateRangesOnly = 1,
        /// Force the use of VPC Access Connector for all egress traffic from the
        /// function.
        AllTraffic = 2,
    }
    impl VpcConnectorEgressSettings {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VpcConnectorEgressSettings::Unspecified => {
                    "VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED"
                }
                VpcConnectorEgressSettings::PrivateRangesOnly => "PRIVATE_RANGES_ONLY",
                VpcConnectorEgressSettings::AllTraffic => "ALL_TRAFFIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIVATE_RANGES_ONLY" => Some(Self::PrivateRangesOnly),
                "ALL_TRAFFIC" => Some(Self::AllTraffic),
                _ => None,
            }
        }
    }
    /// Available ingress settings.
    ///
    /// This controls what traffic can reach the function.
    ///
    /// If unspecified, ALLOW_ALL will be used.
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
    pub enum IngressSettings {
        /// Unspecified.
        Unspecified = 0,
        /// Allow HTTP traffic from public and private sources.
        AllowAll = 1,
        /// Allow HTTP traffic from only private VPC sources.
        AllowInternalOnly = 2,
        /// Allow HTTP traffic from private VPC sources and through GCLB.
        AllowInternalAndGclb = 3,
    }
    impl IngressSettings {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IngressSettings::Unspecified => "INGRESS_SETTINGS_UNSPECIFIED",
                IngressSettings::AllowAll => "ALLOW_ALL",
                IngressSettings::AllowInternalOnly => "ALLOW_INTERNAL_ONLY",
                IngressSettings::AllowInternalAndGclb => "ALLOW_INTERNAL_AND_GCLB",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INGRESS_SETTINGS_UNSPECIFIED" => Some(Self::Unspecified),
                "ALLOW_ALL" => Some(Self::AllowAll),
                "ALLOW_INTERNAL_ONLY" => Some(Self::AllowInternalOnly),
                "ALLOW_INTERNAL_AND_GCLB" => Some(Self::AllowInternalAndGclb),
                _ => None,
            }
        }
    }
    /// Available security level settings.
    ///
    /// This enforces security protocol on function URL.
    ///
    /// Security level is only configurable for 1st Gen functions, If unspecified,
    /// SECURE_OPTIONAL will be used. 2nd Gen functions are SECURE_ALWAYS ONLY.
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
    pub enum SecurityLevel {
        /// Unspecified.
        Unspecified = 0,
        /// Requests for a URL that match this handler that do not use HTTPS are
        /// automatically redirected to the HTTPS URL with the same path. Query
        /// parameters are reserved for the redirect.
        SecureAlways = 1,
        /// Both HTTP and HTTPS requests with URLs that match the handler succeed
        /// without redirects. The application can examine the request to determine
        /// which protocol was used and respond accordingly.
        SecureOptional = 2,
    }
    impl SecurityLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SecurityLevel::Unspecified => "SECURITY_LEVEL_UNSPECIFIED",
                SecurityLevel::SecureAlways => "SECURE_ALWAYS",
                SecurityLevel::SecureOptional => "SECURE_OPTIONAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SECURITY_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                "SECURE_ALWAYS" => Some(Self::SecureAlways),
                "SECURE_OPTIONAL" => Some(Self::SecureOptional),
                _ => None,
            }
        }
    }
}
/// Configuration for a secret environment variable. It has the information
/// necessary to fetch the secret value from secret manager and expose it as an
/// environment variable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretEnvVar {
    /// Name of the environment variable.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Project identifier (preferably project number but can also be the
    /// project ID) of the project that contains the secret. If not set, it is
    /// assumed that the secret is in the same project as the function.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Name of the secret in secret manager (not the full resource name).
    #[prost(string, tag = "3")]
    pub secret: ::prost::alloc::string::String,
    /// Version of the secret (version number or the string 'latest'). It is
    /// recommended to use a numeric version for secret environment variables as
    /// any updates to the secret value is not reflected until new instances
    /// start.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
}
/// Configuration for a secret volume. It has the information necessary to fetch
/// the secret value from secret manager and make it available as files mounted
/// at the requested paths within the application container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretVolume {
    /// The path within the container to mount the secret volume. For example,
    /// setting the mount_path as `/etc/secrets` would mount the secret value files
    /// under the `/etc/secrets` directory. This directory will also be completely
    /// shadowed and unavailable to mount any other secrets.
    /// Recommended mount path: /etc/secrets
    #[prost(string, tag = "1")]
    pub mount_path: ::prost::alloc::string::String,
    /// Project identifier (preferably project number but can also be the project
    /// ID) of the project that contains the secret. If not set, it is
    /// assumed that the secret is in the same project as the function.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Name of the secret in secret manager (not the full resource name).
    #[prost(string, tag = "3")]
    pub secret: ::prost::alloc::string::String,
    /// List of secret versions to mount for this secret. If empty, the `latest`
    /// version of the secret will be made available in a file named after the
    /// secret under the mount point.
    #[prost(message, repeated, tag = "4")]
    pub versions: ::prost::alloc::vec::Vec<secret_volume::SecretVersion>,
}
/// Nested message and enum types in `SecretVolume`.
pub mod secret_volume {
    /// Configuration for a single version.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecretVersion {
        /// Version of the secret (version number or the string 'latest'). It is
        /// preferable to use `latest` version with secret volumes as secret value
        /// changes are reflected immediately.
        #[prost(string, tag = "1")]
        pub version: ::prost::alloc::string::String,
        /// Relative path of the file under the mount path where the secret value for
        /// this version will be fetched and made available. For example, setting the
        /// mount_path as '/etc/secrets' and path as `secret_foo` would mount the
        /// secret value file at `/etc/secrets/secret_foo`.
        #[prost(string, tag = "2")]
        pub path: ::prost::alloc::string::String,
    }
}
/// Describes EventTrigger, used to request events to be sent from another
/// service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTrigger {
    /// Output only. The resource name of the Eventarc trigger. The format of this
    /// field is `projects/{project}/locations/{region}/triggers/{trigger}`.
    #[prost(string, tag = "1")]
    pub trigger: ::prost::alloc::string::String,
    /// The region that the trigger will be in. The trigger will only receive
    /// events originating in this region. It can be the same
    /// region as the function, a different region or multi-region, or the global
    /// region. If not provided, defaults to the same region as the function.
    #[prost(string, tag = "2")]
    pub trigger_region: ::prost::alloc::string::String,
    /// Required. The type of event to observe. For example:
    /// `google.cloud.audit.log.v1.written` or
    /// `google.cloud.pubsub.topic.v1.messagePublished`.
    #[prost(string, tag = "3")]
    pub event_type: ::prost::alloc::string::String,
    /// Criteria used to filter events.
    #[prost(message, repeated, tag = "4")]
    pub event_filters: ::prost::alloc::vec::Vec<EventFilter>,
    /// Optional. The name of a Pub/Sub topic in the same project that will be used
    /// as the transport topic for the event delivery. Format:
    /// `projects/{project}/topics/{topic}`.
    ///
    /// This is only valid for events of type
    /// `google.cloud.pubsub.topic.v1.messagePublished`. The topic provided here
    /// will not be deleted at function deletion.
    #[prost(string, tag = "5")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// Optional. The email of the trigger's service account. The service account
    /// must have permission to invoke Cloud Run services, the permission is
    /// `run.routes.invoke`.
    /// If empty, defaults to the Compute Engine default service account:
    /// `{project_number}-compute@developer.gserviceaccount.com`.
    #[prost(string, tag = "6")]
    pub service_account_email: ::prost::alloc::string::String,
    /// Optional. If unset, then defaults to ignoring failures (i.e. not retrying
    /// them).
    #[prost(enumeration = "event_trigger::RetryPolicy", tag = "7")]
    pub retry_policy: i32,
    /// Optional. The name of the channel associated with the trigger in
    /// `projects/{project}/locations/{location}/channels/{channel}` format.
    /// You must provide a channel to receive events from Eventarc SaaS partners.
    #[prost(string, tag = "8")]
    pub channel: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EventTrigger`.
pub mod event_trigger {
    /// Describes the retry policy in case of function's execution failure.
    /// Retried execution is charged as any other execution.
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
    pub enum RetryPolicy {
        /// Not specified.
        Unspecified = 0,
        /// Do not retry.
        DoNotRetry = 1,
        /// Retry on any failure, retry up to 7 days with an exponential backoff
        /// (capped at 10 seconds).
        Retry = 2,
    }
    impl RetryPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RetryPolicy::Unspecified => "RETRY_POLICY_UNSPECIFIED",
                RetryPolicy::DoNotRetry => "RETRY_POLICY_DO_NOT_RETRY",
                RetryPolicy::Retry => "RETRY_POLICY_RETRY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RETRY_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                "RETRY_POLICY_DO_NOT_RETRY" => Some(Self::DoNotRetry),
                "RETRY_POLICY_RETRY" => Some(Self::Retry),
                _ => None,
            }
        }
    }
}
/// Filters events based on exact matches on the CloudEvents attributes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFilter {
    /// Required. The name of a CloudEvents attribute.
    #[prost(string, tag = "1")]
    pub attribute: ::prost::alloc::string::String,
    /// Required. The value for the attribute.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// Optional. The operator used for matching the events with the value of the
    /// filter. If not specified, only events that have an exact key-value pair
    /// specified in the filter are matched. The only allowed value is
    /// `match-path-pattern`.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// Request for the `GetFunction` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFunctionRequest {
    /// Required. The name of the function which details should be obtained.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListFunctions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsRequest {
    /// Required. The project and location from which the function should be
    /// listed, specified in the format `projects/*/locations/*` If you want to
    /// list functions in all locations, use "-" in place of a location. When
    /// listing functions in all locations, if one or more location(s) are
    /// unreachable, the response will contain functions from all reachable
    /// locations along with the names of any unreachable locations.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of functions to return per call. The largest allowed
    /// page_size is 1,000, if the page_size is omitted or specified as greater
    /// than 1,000 then it will be replaced as 1,000. The size of the list
    /// response can be less than specified when used with filters.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last
    /// `ListFunctionsResponse`; indicates that
    /// this is a continuation of a prior `ListFunctions` call, and that the
    /// system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The filter for Functions that match the filter expression,
    /// following the syntax outlined in <https://google.aip.dev/160.>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// The sorting order of the resources returned. Value should be a comma
    /// separated list of fields. The default sorting oder is ascending.
    /// See <https://google.aip.dev/132#ordering.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the `ListFunctions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsResponse {
    /// The functions that match the request.
    #[prost(message, repeated, tag = "1")]
    pub functions: ::prost::alloc::vec::Vec<Function>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached. The response does not include any
    /// functions from these locations.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `CreateFunction` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFunctionRequest {
    /// Required. The project and location in which the function should be created,
    /// specified in the format `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Function to be created.
    #[prost(message, optional, tag = "2")]
    pub function: ::core::option::Option<Function>,
    /// The ID to use for the function, which will become the final component of
    /// the function's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    #[prost(string, tag = "3")]
    pub function_id: ::prost::alloc::string::String,
}
/// Request for the `UpdateFunction` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFunctionRequest {
    /// Required. New version of the function.
    #[prost(message, optional, tag = "1")]
    pub function: ::core::option::Option<Function>,
    /// The list of fields to be updated.
    /// If no field mask is provided, all provided fields in the request will be
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the `DeleteFunction` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFunctionRequest {
    /// Required. The name of the function which should be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request of `GenerateSourceUploadUrl` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateUploadUrlRequest {
    /// Required. The project and location in which the Google Cloud Storage signed
    /// URL should be generated, specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// \[Preview\] Resource name of a KMS crypto key (managed by the user) used to
    /// encrypt/decrypt function source code objects in intermediate Cloud Storage
    /// buckets. When you generate an upload url and upload your source code, it
    /// gets copied to an intermediate Cloud Storage bucket. The source code is
    /// then copied to a versioned directory in the sources bucket in the consumer
    /// project during the function deployment.
    ///
    /// It must match the pattern
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    ///
    /// The Google Cloud Functions service account
    /// (service-{project_number}@gcf-admin-robot.iam.gserviceaccount.com) must be
    /// granted the role 'Cloud KMS CryptoKey Encrypter/Decrypter
    /// (roles/cloudkms.cryptoKeyEncrypterDecrypter)' on the
    /// Key/KeyRing/Project/Organization (least access preferred).
    #[prost(string, tag = "2")]
    pub kms_key_name: ::prost::alloc::string::String,
}
/// Response of `GenerateSourceUploadUrl` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateUploadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for a
    /// function source code upload. The uploaded file should be a zip archive
    /// which contains a function.
    #[prost(string, tag = "1")]
    pub upload_url: ::prost::alloc::string::String,
    /// The location of the source code in the upload bucket.
    ///
    /// Once the archive is uploaded using the `upload_url` use this field to
    /// set the `function.build_config.source.storage_source`
    /// during CreateFunction and UpdateFunction.
    ///
    /// Generation defaults to 0, as Cloud Storage provides a new generation only
    /// upon uploading a new object or version of an object.
    #[prost(message, optional, tag = "2")]
    pub storage_source: ::core::option::Option<StorageSource>,
}
/// Request of `GenerateDownloadUrl` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDownloadUrlRequest {
    /// Required. The name of function for which source code Google Cloud Storage
    /// signed URL should be generated.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response of `GenerateDownloadUrl` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDownloadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for
    /// function source code download.
    #[prost(string, tag = "1")]
    pub download_url: ::prost::alloc::string::String,
}
/// Request for the `ListRuntimes` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimesRequest {
    /// Required. The project and location from which the runtimes should be
    /// listed, specified in the format `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter for Runtimes that match the filter expression,
    /// following the syntax outlined in <https://google.aip.dev/160.>
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
}
/// Response for the `ListRuntimes` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimesResponse {
    /// The runtimes that match the request.
    #[prost(message, repeated, tag = "1")]
    pub runtimes: ::prost::alloc::vec::Vec<list_runtimes_response::Runtime>,
}
/// Nested message and enum types in `ListRuntimesResponse`.
pub mod list_runtimes_response {
    /// Describes a runtime and any special information (e.g., deprecation status)
    /// related to it.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Runtime {
        /// The name of the runtime, e.g., 'go113', 'nodejs12', etc.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The user facing name, eg 'Go 1.13', 'Node.js 12', etc.
        #[prost(string, tag = "5")]
        pub display_name: ::prost::alloc::string::String,
        /// The stage of life this runtime is in, e.g., BETA, GA, etc.
        #[prost(enumeration = "RuntimeStage", tag = "2")]
        pub stage: i32,
        /// Warning messages, e.g., a deprecation warning.
        #[prost(string, repeated, tag = "3")]
        pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The environment for the runtime.
        #[prost(enumeration = "super::Environment", tag = "4")]
        pub environment: i32,
    }
    /// The various stages that a runtime can be in.
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
    pub enum RuntimeStage {
        /// Not specified.
        Unspecified = 0,
        /// The runtime is in development.
        Development = 1,
        /// The runtime is in the Alpha stage.
        Alpha = 2,
        /// The runtime is in the Beta stage.
        Beta = 3,
        /// The runtime is generally available.
        Ga = 4,
        /// The runtime is deprecated.
        Deprecated = 5,
        /// The runtime is no longer supported.
        Decommissioned = 6,
    }
    impl RuntimeStage {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RuntimeStage::Unspecified => "RUNTIME_STAGE_UNSPECIFIED",
                RuntimeStage::Development => "DEVELOPMENT",
                RuntimeStage::Alpha => "ALPHA",
                RuntimeStage::Beta => "BETA",
                RuntimeStage::Ga => "GA",
                RuntimeStage::Deprecated => "DEPRECATED",
                RuntimeStage::Decommissioned => "DECOMMISSIONED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RUNTIME_STAGE_UNSPECIFIED" => Some(Self::Unspecified),
                "DEVELOPMENT" => Some(Self::Development),
                "ALPHA" => Some(Self::Alpha),
                "BETA" => Some(Self::Beta),
                "GA" => Some(Self::Ga),
                "DEPRECATED" => Some(Self::Deprecated),
                "DECOMMISSIONED" => Some(Self::Decommissioned),
                _ => None,
            }
        }
    }
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// The original request that started the operation.
    #[prost(message, optional, tag = "8")]
    pub request_resource: ::core::option::Option<::prost_types::Any>,
    /// Mechanism for reporting in-progress stages
    #[prost(message, repeated, tag = "9")]
    pub stages: ::prost::alloc::vec::Vec<Stage>,
}
/// Extra GCF specific location information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// The Cloud Function environments this location supports.
    #[prost(enumeration = "Environment", repeated, tag = "1")]
    pub environments: ::prost::alloc::vec::Vec<i32>,
}
/// Each Stage of the deployment process
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stage {
    /// Name of the Stage. This will be unique for each Stage.
    #[prost(enumeration = "stage::Name", tag = "1")]
    pub name: i32,
    /// Message describing the Stage
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// Current state of the Stage
    #[prost(enumeration = "stage::State", tag = "3")]
    pub state: i32,
    /// Resource of the Stage
    #[prost(string, tag = "4")]
    pub resource: ::prost::alloc::string::String,
    /// Link to the current Stage resource
    #[prost(string, tag = "5")]
    pub resource_uri: ::prost::alloc::string::String,
    /// State messages from the current Stage.
    #[prost(message, repeated, tag = "6")]
    pub state_messages: ::prost::alloc::vec::Vec<StateMessage>,
}
/// Nested message and enum types in `Stage`.
pub mod stage {
    /// Possible names for a Stage
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
    pub enum Name {
        /// Not specified. Invalid name.
        Unspecified = 0,
        /// Artifact Regsitry Stage
        ArtifactRegistry = 1,
        /// Build Stage
        Build = 2,
        /// Service Stage
        Service = 3,
        /// Trigger Stage
        Trigger = 4,
        /// Service Rollback Stage
        ServiceRollback = 5,
        /// Trigger Rollback Stage
        TriggerRollback = 6,
    }
    impl Name {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Name::Unspecified => "NAME_UNSPECIFIED",
                Name::ArtifactRegistry => "ARTIFACT_REGISTRY",
                Name::Build => "BUILD",
                Name::Service => "SERVICE",
                Name::Trigger => "TRIGGER",
                Name::ServiceRollback => "SERVICE_ROLLBACK",
                Name::TriggerRollback => "TRIGGER_ROLLBACK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NAME_UNSPECIFIED" => Some(Self::Unspecified),
                "ARTIFACT_REGISTRY" => Some(Self::ArtifactRegistry),
                "BUILD" => Some(Self::Build),
                "SERVICE" => Some(Self::Service),
                "TRIGGER" => Some(Self::Trigger),
                "SERVICE_ROLLBACK" => Some(Self::ServiceRollback),
                "TRIGGER_ROLLBACK" => Some(Self::TriggerRollback),
                _ => None,
            }
        }
    }
    /// Possible states for a Stage
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
    pub enum State {
        /// Not specified. Invalid state.
        Unspecified = 0,
        /// Stage has not started.
        NotStarted = 1,
        /// Stage is in progress.
        InProgress = 2,
        /// Stage has completed.
        Complete = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::NotStarted => "NOT_STARTED",
                State::InProgress => "IN_PROGRESS",
                State::Complete => "COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "NOT_STARTED" => Some(Self::NotStarted),
                "IN_PROGRESS" => Some(Self::InProgress),
                "COMPLETE" => Some(Self::Complete),
                _ => None,
            }
        }
    }
}
/// The environment the function is hosted on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Environment {
    /// Unspecified
    Unspecified = 0,
    /// Gen 1
    Gen1 = 1,
    /// Gen 2
    Gen2 = 2,
}
impl Environment {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Environment::Unspecified => "ENVIRONMENT_UNSPECIFIED",
            Environment::Gen1 => "GEN_1",
            Environment::Gen2 => "GEN_2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENVIRONMENT_UNSPECIFIED" => Some(Self::Unspecified),
            "GEN_1" => Some(Self::Gen1),
            "GEN_2" => Some(Self::Gen2),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod function_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Google Cloud Functions is used to deploy functions that are executed by
    /// Google in response to various events. Data connected with that event is
    /// passed to a function as the input data.
    ///
    /// A **function** is a resource which describes a function that should be
    /// executed and how it is triggered.
    #[derive(Debug, Clone)]
    pub struct FunctionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FunctionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FunctionServiceClient<T>
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
        ) -> FunctionServiceClient<InterceptedService<T, F>>
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
            FunctionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Returns a function with the given name from the requested project.
        pub async fn get_function(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFunctionRequest>,
        ) -> std::result::Result<tonic::Response<super::Function>, tonic::Status> {
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
                "/google.cloud.functions.v2alpha.FunctionService/GetFunction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.functions.v2alpha.FunctionService",
                        "GetFunction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a list of functions that belong to the requested project.
        pub async fn list_functions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFunctionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFunctionsResponse>,
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
                "/google.cloud.functions.v2alpha.FunctionService/ListFunctions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.functions.v2alpha.FunctionService",
                        "ListFunctions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new function. If a function with the given name already exists in
        /// the specified project, the long running operation will return
        /// `ALREADY_EXISTS` error.
        pub async fn create_function(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFunctionRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.functions.v2alpha.FunctionService/CreateFunction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.functions.v2alpha.FunctionService",
                        "CreateFunction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates existing function.
        pub async fn update_function(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFunctionRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.functions.v2alpha.FunctionService/UpdateFunction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.functions.v2alpha.FunctionService",
                        "UpdateFunction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a function with the given name from the specified project. If the
        /// given function is used by some trigger, the trigger will be updated to
        /// remove this function.
        pub async fn delete_function(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFunctionRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.functions.v2alpha.FunctionService/DeleteFunction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.functions.v2alpha.FunctionService",
                        "DeleteFunction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a signed URL for uploading a function source code.
        /// For more information about the signed URL usage see:
        /// https://cloud.google.com/storage/docs/access-control/signed-urls.
        /// Once the function source code upload is complete, the used signed
        /// URL should be provided in CreateFunction or UpdateFunction request
        /// as a reference to the function source code.
        ///
        /// When uploading source code to the generated signed URL, please follow
        /// these restrictions:
        ///
        /// * Source file type should be a zip file.
        /// * No credentials should be attached - the signed URLs provide access to the
        ///   target bucket using internal service identity; if credentials were
        ///   attached, the identity from the credentials would be used, but that
        ///   identity does not have permissions to upload files to the URL.
        ///
        /// When making a HTTP PUT request, these two headers need to be specified:
        ///
        /// * `content-type: application/zip`
        ///
        /// And this header SHOULD NOT be specified:
        ///
        /// * `Authorization: Bearer YOUR_TOKEN`
        pub async fn generate_upload_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateUploadUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateUploadUrlResponse>,
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
                "/google.cloud.functions.v2alpha.FunctionService/GenerateUploadUrl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.functions.v2alpha.FunctionService",
                        "GenerateUploadUrl",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a signed URL for downloading deployed function source code.
        /// The URL is only valid for a limited period and should be used within
        /// 30 minutes of generation.
        /// For more information about the signed URL usage see:
        /// https://cloud.google.com/storage/docs/access-control/signed-urls
        pub async fn generate_download_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateDownloadUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateDownloadUrlResponse>,
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
                "/google.cloud.functions.v2alpha.FunctionService/GenerateDownloadUrl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.functions.v2alpha.FunctionService",
                        "GenerateDownloadUrl",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a list of runtimes that are supported for the requested project.
        pub async fn list_runtimes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuntimesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRuntimesResponse>,
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
                "/google.cloud.functions.v2alpha.FunctionService/ListRuntimes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.functions.v2alpha.FunctionService",
                        "ListRuntimes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
