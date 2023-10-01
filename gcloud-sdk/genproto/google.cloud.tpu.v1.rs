/// Sets the scheduling options for this node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulingConfig {
    /// Defines whether the node is preemptible.
    #[prost(bool, tag = "1")]
    pub preemptible: bool,
    /// Whether the node is created under a reservation.
    #[prost(bool, tag = "2")]
    pub reserved: bool,
}
/// A network endpoint over which a TPU worker can be reached.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkEndpoint {
    /// The IP address of this network endpoint.
    #[prost(string, tag = "1")]
    pub ip_address: ::prost::alloc::string::String,
    /// The port of this network endpoint.
    #[prost(int32, tag = "2")]
    pub port: i32,
}
/// A TPU instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// Output only. Immutable. The name of the TPU
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The user-supplied description of the TPU. Maximum of 512 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Required. The type of hardware accelerators associated with this node.
    #[prost(string, tag = "5")]
    pub accelerator_type: ::prost::alloc::string::String,
    /// Output only. DEPRECATED! Use network_endpoints instead.
    /// The network address for the TPU Node as visible to Compute Engine
    /// instances.
    #[deprecated]
    #[prost(string, tag = "8")]
    pub ip_address: ::prost::alloc::string::String,
    /// Output only. DEPRECATED! Use network_endpoints instead.
    /// The network port for the TPU Node as visible to Compute Engine instances.
    #[deprecated]
    #[prost(string, tag = "14")]
    pub port: ::prost::alloc::string::String,
    /// Output only. The current state for the TPU Node.
    #[prost(enumeration = "node::State", tag = "9")]
    pub state: i32,
    /// Output only. If this field is populated, it contains a description of why the TPU Node
    /// is unhealthy.
    #[prost(string, tag = "10")]
    pub health_description: ::prost::alloc::string::String,
    /// Required. The version of Tensorflow running in the Node.
    #[prost(string, tag = "11")]
    pub tensorflow_version: ::prost::alloc::string::String,
    /// The name of a network they wish to peer the TPU node to. It must be a
    /// preexisting Compute Engine network inside of the project on which this API
    /// has been activated. If none is provided, "default" will be used.
    #[prost(string, tag = "12")]
    pub network: ::prost::alloc::string::String,
    /// The CIDR block that the TPU node will use when selecting an IP address.
    /// This CIDR block must be a /29 block; the Compute Engine networks API
    /// forbids a smaller block, and using a larger block would be wasteful (a
    /// node can only consume one IP address). Errors will occur if the CIDR block
    /// has already been used for a currently existing TPU node, the CIDR block
    /// conflicts with any subnetworks in the user's provided network, or the
    /// provided network is peered with another network that is using that CIDR
    /// block.
    #[prost(string, tag = "13")]
    pub cidr_block: ::prost::alloc::string::String,
    /// Output only. The service account used to run the tensor flow services within the node.
    /// To share resources, including Google Cloud Storage data, with the
    /// Tensorflow job running in the Node, this account must have permissions to
    /// that data.
    #[prost(string, tag = "15")]
    pub service_account: ::prost::alloc::string::String,
    /// Output only. The time when the node was created.
    #[prost(message, optional, tag = "16")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The scheduling options for this node.
    #[prost(message, optional, tag = "17")]
    pub scheduling_config: ::core::option::Option<SchedulingConfig>,
    /// Output only. The network endpoints where TPU workers can be accessed and
    /// sent work. It is recommended that Tensorflow clients of the node reach out
    /// to the 0th entry in this map first.
    #[prost(message, repeated, tag = "21")]
    pub network_endpoints: ::prost::alloc::vec::Vec<NetworkEndpoint>,
    /// The health status of the TPU node.
    #[prost(enumeration = "node::Health", tag = "22")]
    pub health: i32,
    /// Resource labels to represent user-provided metadata.
    #[prost(map = "string, string", tag = "24")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Whether the VPC peering for the node is set up through Service Networking
    /// API. The VPC Peering should be set up before provisioning the node.
    /// If this field is set, cidr_block field should not be specified. If the
    /// network, that you want to peer the TPU Node to, is Shared VPC networks,
    /// the node must be created with this this field enabled.
    #[prost(bool, tag = "27")]
    pub use_service_networking: bool,
    /// Output only. The API version that created this Node.
    #[prost(enumeration = "node::ApiVersion", tag = "38")]
    pub api_version: i32,
    /// Output only. The Symptoms that have occurred to the TPU Node.
    #[prost(message, repeated, tag = "39")]
    pub symptoms: ::prost::alloc::vec::Vec<Symptom>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    /// Represents the different states of a TPU node during its lifecycle.
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
        /// TPU node state is not known/set.
        Unspecified = 0,
        /// TPU node is being created.
        Creating = 1,
        /// TPU node has been created and is fully usable.
        Ready = 2,
        /// TPU node is restarting.
        Restarting = 3,
        /// TPU node is undergoing reimaging.
        Reimaging = 4,
        /// TPU node is being deleted.
        Deleting = 5,
        /// TPU node is being repaired and may be unusable. Details can be
        /// found in the `help_description` field.
        Repairing = 6,
        /// TPU node is stopped.
        Stopped = 8,
        /// TPU node is currently stopping.
        Stopping = 9,
        /// TPU node is currently starting.
        Starting = 10,
        /// TPU node has been preempted. Only applies to Preemptible TPU Nodes.
        Preempted = 11,
        /// TPU node has been terminated due to maintenance or has reached the end of
        /// its life cycle (for preemptible nodes).
        Terminated = 12,
        /// TPU node is currently hiding.
        Hiding = 13,
        /// TPU node has been hidden.
        Hidden = 14,
        /// TPU node is currently unhiding.
        Unhiding = 15,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Ready => "READY",
                State::Restarting => "RESTARTING",
                State::Reimaging => "REIMAGING",
                State::Deleting => "DELETING",
                State::Repairing => "REPAIRING",
                State::Stopped => "STOPPED",
                State::Stopping => "STOPPING",
                State::Starting => "STARTING",
                State::Preempted => "PREEMPTED",
                State::Terminated => "TERMINATED",
                State::Hiding => "HIDING",
                State::Hidden => "HIDDEN",
                State::Unhiding => "UNHIDING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "RESTARTING" => Some(Self::Restarting),
                "REIMAGING" => Some(Self::Reimaging),
                "DELETING" => Some(Self::Deleting),
                "REPAIRING" => Some(Self::Repairing),
                "STOPPED" => Some(Self::Stopped),
                "STOPPING" => Some(Self::Stopping),
                "STARTING" => Some(Self::Starting),
                "PREEMPTED" => Some(Self::Preempted),
                "TERMINATED" => Some(Self::Terminated),
                "HIDING" => Some(Self::Hiding),
                "HIDDEN" => Some(Self::Hidden),
                "UNHIDING" => Some(Self::Unhiding),
                _ => None,
            }
        }
    }
    /// Health defines the status of a TPU node as reported by
    /// Health Monitor.
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
    pub enum Health {
        /// Health status is unknown: not initialized or failed to retrieve.
        Unspecified = 0,
        /// The resource is healthy.
        Healthy = 1,
        /// The resource is unhealthy.
        DeprecatedUnhealthy = 2,
        /// The resource is unresponsive.
        Timeout = 3,
        /// The in-guest ML stack is unhealthy.
        UnhealthyTensorflow = 4,
        /// The node is under maintenance/priority boost caused rescheduling and
        /// will resume running once rescheduled.
        UnhealthyMaintenance = 5,
    }
    impl Health {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Health::Unspecified => "HEALTH_UNSPECIFIED",
                Health::Healthy => "HEALTHY",
                Health::DeprecatedUnhealthy => "DEPRECATED_UNHEALTHY",
                Health::Timeout => "TIMEOUT",
                Health::UnhealthyTensorflow => "UNHEALTHY_TENSORFLOW",
                Health::UnhealthyMaintenance => "UNHEALTHY_MAINTENANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HEALTH_UNSPECIFIED" => Some(Self::Unspecified),
                "HEALTHY" => Some(Self::Healthy),
                "DEPRECATED_UNHEALTHY" => Some(Self::DeprecatedUnhealthy),
                "TIMEOUT" => Some(Self::Timeout),
                "UNHEALTHY_TENSORFLOW" => Some(Self::UnhealthyTensorflow),
                "UNHEALTHY_MAINTENANCE" => Some(Self::UnhealthyMaintenance),
                _ => None,
            }
        }
    }
    /// TPU API Version.
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
    pub enum ApiVersion {
        /// API version is unknown.
        Unspecified = 0,
        /// TPU API V1Alpha1 version.
        V1Alpha1 = 1,
        /// TPU API V1 version.
        V1 = 2,
        /// TPU API V2Alpha1 version.
        V2Alpha1 = 3,
    }
    impl ApiVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ApiVersion::Unspecified => "API_VERSION_UNSPECIFIED",
                ApiVersion::V1Alpha1 => "V1_ALPHA1",
                ApiVersion::V1 => "V1",
                ApiVersion::V2Alpha1 => "V2_ALPHA1",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "API_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
                "V1_ALPHA1" => Some(Self::V1Alpha1),
                "V1" => Some(Self::V1),
                "V2_ALPHA1" => Some(Self::V2Alpha1),
                _ => None,
            }
        }
    }
}
/// Request for [ListNodes][google.cloud.tpu.v1.Tpu.ListNodes].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodesRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for [ListNodes][google.cloud.tpu.v1.Tpu.ListNodes].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodesResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<Node>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for [GetNode][google.cloud.tpu.v1.Tpu.GetNode].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for [CreateNode][google.cloud.tpu.v1.Tpu.CreateNode].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNodeRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The unqualified resource name.
    #[prost(string, tag = "2")]
    pub node_id: ::prost::alloc::string::String,
    /// Required. The node.
    #[prost(message, optional, tag = "3")]
    pub node: ::core::option::Option<Node>,
}
/// Request for [DeleteNode][google.cloud.tpu.v1.Tpu.DeleteNode].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNodeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for [ReimageNode][google.cloud.tpu.v1.Tpu.ReimageNode].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReimageNodeRequest {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The version for reimage to create.
    #[prost(string, tag = "2")]
    pub tensorflow_version: ::prost::alloc::string::String,
}
/// Request for [StopNode][google.cloud.tpu.v1.Tpu.StopNode].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopNodeRequest {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for [StartNode][google.cloud.tpu.v1.Tpu.StartNode].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNodeRequest {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A tensorflow version that a Node can be configured with.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorFlowVersion {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// the tensorflow version.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// Request for [GetTensorFlowVersion][google.cloud.tpu.v1.Tpu.GetTensorFlowVersion].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTensorFlowVersionRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for [ListTensorFlowVersions][google.cloud.tpu.v1.Tpu.ListTensorFlowVersions].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTensorFlowVersionsRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Sort results.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for [ListTensorFlowVersions][google.cloud.tpu.v1.Tpu.ListTensorFlowVersions].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTensorFlowVersionsResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub tensorflow_versions: ::prost::alloc::vec::Vec<TensorFlowVersion>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A accelerator type that a Node can be configured with.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceleratorType {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// the accelerator type.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
}
/// Request for [GetAcceleratorType][google.cloud.tpu.v1.Tpu.GetAcceleratorType].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAcceleratorTypeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for [ListAcceleratorTypes][google.cloud.tpu.v1.Tpu.ListAcceleratorTypes].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAcceleratorTypesRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Sort results.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for [ListAcceleratorTypes][google.cloud.tpu.v1.Tpu.ListAcceleratorTypes].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAcceleratorTypesResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub accelerator_types: ::prost::alloc::vec::Vec<AcceleratorType>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata describing an [Operation][google.longrunning.Operation]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Target of the operation - for example
    /// projects/project-1/connectivityTests/test-1
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Specifies if cancellation was requested for the operation.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// A Symptom instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Symptom {
    /// Timestamp when the Symptom is created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Type of the Symptom.
    #[prost(enumeration = "symptom::SymptomType", tag = "2")]
    pub symptom_type: i32,
    /// Detailed information of the current Symptom.
    #[prost(string, tag = "3")]
    pub details: ::prost::alloc::string::String,
    /// A string used to uniquely distinguish a worker within a TPU node.
    #[prost(string, tag = "4")]
    pub worker_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Symptom`.
pub mod symptom {
    /// SymptomType represents the different types of Symptoms that a TPU can be
    /// at.
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
    pub enum SymptomType {
        /// Unspecified symptom.
        Unspecified = 0,
        /// TPU VM memory is low.
        LowMemory = 1,
        /// TPU runtime is out of memory.
        OutOfMemory = 2,
        /// TPU runtime execution has timed out.
        ExecuteTimedOut = 3,
        /// TPU runtime fails to construct a mesh that recognizes each TPU device's
        /// neighbors.
        MeshBuildFail = 4,
        /// TPU HBM is out of memory.
        HbmOutOfMemory = 5,
        /// Abusive behaviors have been identified on the current project.
        ProjectAbuse = 6,
    }
    impl SymptomType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SymptomType::Unspecified => "SYMPTOM_TYPE_UNSPECIFIED",
                SymptomType::LowMemory => "LOW_MEMORY",
                SymptomType::OutOfMemory => "OUT_OF_MEMORY",
                SymptomType::ExecuteTimedOut => "EXECUTE_TIMED_OUT",
                SymptomType::MeshBuildFail => "MESH_BUILD_FAIL",
                SymptomType::HbmOutOfMemory => "HBM_OUT_OF_MEMORY",
                SymptomType::ProjectAbuse => "PROJECT_ABUSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYMPTOM_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "LOW_MEMORY" => Some(Self::LowMemory),
                "OUT_OF_MEMORY" => Some(Self::OutOfMemory),
                "EXECUTE_TIMED_OUT" => Some(Self::ExecuteTimedOut),
                "MESH_BUILD_FAIL" => Some(Self::MeshBuildFail),
                "HBM_OUT_OF_MEMORY" => Some(Self::HbmOutOfMemory),
                "PROJECT_ABUSE" => Some(Self::ProjectAbuse),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod tpu_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages TPU nodes and other resources
    ///
    /// TPU API v1
    #[derive(Debug, Clone)]
    pub struct TpuClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TpuClient<tonic::transport::Channel> {
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
    impl<T> TpuClient<T>
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
        ) -> TpuClient<InterceptedService<T, F>>
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
            TpuClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists nodes.
        pub async fn list_nodes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNodesResponse>,
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
                "/google.cloud.tpu.v1.Tpu/ListNodes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v1.Tpu", "ListNodes"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a node.
        pub async fn get_node(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodeRequest>,
        ) -> std::result::Result<tonic::Response<super::Node>, tonic::Status> {
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
                "/google.cloud.tpu.v1.Tpu/GetNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v1.Tpu", "GetNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates a node.
        pub async fn create_node(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNodeRequest>,
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
                "/google.cloud.tpu.v1.Tpu/CreateNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v1.Tpu", "CreateNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a node.
        pub async fn delete_node(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNodeRequest>,
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
                "/google.cloud.tpu.v1.Tpu/DeleteNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v1.Tpu", "DeleteNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Reimages a node's OS.
        pub async fn reimage_node(
            &mut self,
            request: impl tonic::IntoRequest<super::ReimageNodeRequest>,
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
                "/google.cloud.tpu.v1.Tpu/ReimageNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v1.Tpu", "ReimageNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Stops a node.
        pub async fn stop_node(
            &mut self,
            request: impl tonic::IntoRequest<super::StopNodeRequest>,
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
                "/google.cloud.tpu.v1.Tpu/StopNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v1.Tpu", "StopNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Starts a node.
        pub async fn start_node(
            &mut self,
            request: impl tonic::IntoRequest<super::StartNodeRequest>,
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
                "/google.cloud.tpu.v1.Tpu/StartNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v1.Tpu", "StartNode"));
            self.inner.unary(req, path, codec).await
        }
        /// List TensorFlow versions supported by this API.
        pub async fn list_tensor_flow_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTensorFlowVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTensorFlowVersionsResponse>,
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
                "/google.cloud.tpu.v1.Tpu/ListTensorFlowVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tpu.v1.Tpu", "ListTensorFlowVersions"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets TensorFlow Version.
        pub async fn get_tensor_flow_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTensorFlowVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TensorFlowVersion>,
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
                "/google.cloud.tpu.v1.Tpu/GetTensorFlowVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tpu.v1.Tpu", "GetTensorFlowVersion"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists accelerator types supported by this API.
        pub async fn list_accelerator_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAcceleratorTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAcceleratorTypesResponse>,
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
                "/google.cloud.tpu.v1.Tpu/ListAcceleratorTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tpu.v1.Tpu", "ListAcceleratorTypes"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets AcceleratorType.
        pub async fn get_accelerator_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAcceleratorTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceleratorType>,
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
                "/google.cloud.tpu.v1.Tpu/GetAcceleratorType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tpu.v1.Tpu", "GetAcceleratorType"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
