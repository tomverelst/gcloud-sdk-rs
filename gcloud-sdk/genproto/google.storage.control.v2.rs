// This file is @generated by prost-build.
/// Contains information about a pending rename operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingRenameInfo {
    /// Output only. The name of the rename operation.
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
}
/// A folder resource. This resource can only exist in a hierarchical namespace
/// enabled bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Folder {
    /// Identifier. The name of this folder.
    /// Format: `projects/{project}/buckets/{bucket}/folders/{folder}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The version of the metadata for this folder. Used for
    /// preconditions and for detecting changes in metadata.
    #[prost(int64, tag = "3")]
    pub metageneration: i64,
    /// Output only. The creation time of the folder.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The modification time of the folder.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Only present if the folder is part of an ongoing RenameFolder
    /// operation. Contains information which can be used to query the operation
    /// status. The presence of this field also indicates all write operations are
    /// blocked for this folder, including folder, managed folder, and object
    /// operations.
    #[prost(message, optional, tag = "7")]
    pub pending_rename_info: ::core::option::Option<PendingRenameInfo>,
}
/// Request message for GetFolder. This operation is only applicable to a
/// hierarchical namespace enabled bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFolderRequest {
    /// Required. Name of the folder.
    /// Format: `projects/{project}/buckets/{bucket}/folders/{folder}`
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// Makes the operation only succeed conditional on whether the folder's
    /// current metageneration matches the given value.
    #[prost(int64, optional, tag = "3")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation only succeed conditional on whether the folder's
    /// current metageneration does not match the given value.
    #[prost(int64, optional, tag = "4")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Optional. A unique identifier for this request. UUID is the recommended
    /// format, but other formats are still accepted.
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for CreateFolder. This operation is only applicable to a
/// hierarchical namespace enabled bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFolderRequest {
    /// Required. Name of the bucket in which the folder will reside. The bucket
    /// must be a hierarchical namespace enabled bucket.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Properties of the new folder being created.
    /// The bucket and name of the folder are specified in the parent and folder_id
    /// fields, respectively. Populating those fields in `folder` will result in an
    /// error.
    #[prost(message, optional, tag = "2")]
    pub folder: ::core::option::Option<Folder>,
    /// Required. The full name of a folder, including all its parent folders.
    /// Folders use single '/' characters as a delimiter.
    /// The folder_id must end with a slash.
    /// For example, the folder_id of "books/biographies/" would create a new
    /// "biographies/" folder under the "books/" folder.
    #[prost(string, tag = "3")]
    pub folder_id: ::prost::alloc::string::String,
    /// Optional. If true, parent folder doesn't have to be present and all missing
    /// ancestor folders will be created atomically.
    #[prost(bool, tag = "4")]
    pub recursive: bool,
    /// Optional. A unique identifier for this request. UUID is the recommended
    /// format, but other formats are still accepted.
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for DeleteFolder. This operation is only applicable to a
/// hierarchical namespace enabled bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFolderRequest {
    /// Required. Name of the folder.
    /// Format: `projects/{project}/buckets/{bucket}/folders/{folder}`
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// Makes the operation only succeed conditional on whether the folder's
    /// current metageneration matches the given value.
    #[prost(int64, optional, tag = "3")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation only succeed conditional on whether the folder's
    /// current metageneration does not match the given value.
    #[prost(int64, optional, tag = "4")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Optional. A unique identifier for this request. UUID is the recommended
    /// format, but other formats are still accepted.
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for ListFolders. This operation is only applicable to a
/// hierarchical namespace enabled bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFoldersRequest {
    /// Required. Name of the bucket in which to look for folders. The bucket must
    /// be a hierarchical namespace enabled bucket.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of folders to return in a single response. The
    /// service will use this parameter or 1,000 items, whichever is smaller.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A previously-returned page token representing part of the larger
    /// set of results to view.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter results to folders whose names begin with this prefix.
    /// If set, the value must either be an empty string or end with a '/'.
    #[prost(string, tag = "4")]
    pub prefix: ::prost::alloc::string::String,
    /// Optional. If set, returns results in a directory-like mode. The results
    /// will only include folders that either exactly match the above prefix, or
    /// are one level below the prefix. The only supported value is '/'.
    #[prost(string, tag = "8")]
    pub delimiter: ::prost::alloc::string::String,
    /// Optional. Filter results to folders whose names are lexicographically equal
    /// to or after lexicographic_start. If lexicographic_end is also set, the
    /// folders listed have names between lexicographic_start (inclusive) and
    /// lexicographic_end (exclusive).
    #[prost(string, tag = "6")]
    pub lexicographic_start: ::prost::alloc::string::String,
    /// Optional. Filter results to folders whose names are lexicographically
    /// before lexicographic_end. If lexicographic_start is also set, the folders
    /// listed have names between lexicographic_start (inclusive) and
    /// lexicographic_end (exclusive).
    #[prost(string, tag = "7")]
    pub lexicographic_end: ::prost::alloc::string::String,
    /// Optional. A unique identifier for this request. UUID is the recommended
    /// format, but other formats are still accepted.
    #[prost(string, tag = "9")]
    pub request_id: ::prost::alloc::string::String,
}
/// Response message for ListFolders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFoldersResponse {
    /// The list of child folders
    #[prost(message, repeated, tag = "1")]
    pub folders: ::prost::alloc::vec::Vec<Folder>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for RenameFolder. This operation is only applicable to a
/// hierarchical namespace enabled bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameFolderRequest {
    /// Required. Name of the source folder being renamed.
    /// Format: `projects/{project}/buckets/{bucket}/folders/{folder}`
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// Required. The destination folder ID, e.g. `foo/bar/`.
    #[prost(string, tag = "8")]
    pub destination_folder_id: ::prost::alloc::string::String,
    /// Makes the operation only succeed conditional on whether the source
    /// folder's current metageneration matches the given value.
    #[prost(int64, optional, tag = "4")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation only succeed conditional on whether the source
    /// folder's current metageneration does not match the given value.
    #[prost(int64, optional, tag = "5")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Optional. A unique identifier for this request. UUID is the recommended
    /// format, but other formats are still accepted. This request is only
    /// idempotent if a `request_id` is provided.
    #[prost(string, tag = "6")]
    pub request_id: ::prost::alloc::string::String,
}
/// The message contains metadata that is common to all Storage Control
/// long-running operations, present in its `google.longrunning.Operation`
/// messages, and accessible via `metadata.common_metadata`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonLongRunningOperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation was last modified.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The type of operation invoked.
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation.
    #[prost(bool, tag = "5")]
    pub requested_cancellation: bool,
    /// Output only. The estimated progress of the operation in percentage [0,
    /// 100]. The value -1 means the progress is unknown.
    #[prost(int32, tag = "6")]
    pub progress_percent: i32,
}
/// Message returned in the metadata field of the Operation resource for
/// RenameFolder operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameFolderMetadata {
    /// Generic metadata for the long running operation.
    #[prost(message, optional, tag = "1")]
    pub common_metadata: ::core::option::Option<CommonLongRunningOperationMetadata>,
    /// The path of the source folder.
    #[prost(string, tag = "2")]
    pub source_folder_id: ::prost::alloc::string::String,
    /// The path of the destination folder.
    #[prost(string, tag = "3")]
    pub destination_folder_id: ::prost::alloc::string::String,
}
/// The storage layout configuration of a bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageLayout {
    /// Output only. The name of the StorageLayout resource.
    /// Format: `projects/{project}/buckets/{bucket}/storageLayout`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The location of the bucket.
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
    /// Output only. The location type of the bucket (region, dual-region,
    /// multi-region, etc).
    #[prost(string, tag = "3")]
    pub location_type: ::prost::alloc::string::String,
    /// Output only. The data placement configuration for custom dual region. If
    /// there is no configuration, this is not a custom dual region bucket.
    #[prost(message, optional, tag = "4")]
    pub custom_placement_config: ::core::option::Option<
        storage_layout::CustomPlacementConfig,
    >,
    /// Output only. The bucket's hierarchical namespace configuration. If there is
    /// no configuration, the hierarchical namespace is disabled.
    #[prost(message, optional, tag = "5")]
    pub hierarchical_namespace: ::core::option::Option<
        storage_layout::HierarchicalNamespace,
    >,
}
/// Nested message and enum types in `StorageLayout`.
pub mod storage_layout {
    /// Configuration for Custom Dual Regions.  It should specify precisely two
    /// eligible regions within the same Multiregion. More information on regions
    /// may be found [<https://cloud.google.com/storage/docs/locations][here].>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomPlacementConfig {
        /// List of locations to use for data placement.
        #[prost(string, repeated, tag = "1")]
        pub data_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Configuration for a bucket's hierarchical namespace feature.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct HierarchicalNamespace {
        /// Enables the hierarchical namespace feature.
        #[prost(bool, tag = "1")]
        pub enabled: bool,
    }
}
/// Request message for GetStorageLayout.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStorageLayoutRequest {
    /// Required. The name of the StorageLayout resource.
    /// Format: `projects/{project}/buckets/{bucket}/storageLayout`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An optional prefix used for permission check. It is useful when the caller
    /// only has limited permissions under a specific prefix.
    #[prost(string, tag = "2")]
    pub prefix: ::prost::alloc::string::String,
    /// Optional. A unique identifier for this request. UUID is the recommended
    /// format, but other formats are still accepted.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// A managed folder.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedFolder {
    /// Identifier. The name of this managed folder.
    /// Format:
    /// `projects/{project}/buckets/{bucket}/managedFolders/{managedFolder}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The metadata version of this managed folder. It increases
    /// whenever the metadata is updated. Used for preconditions and for detecting
    /// changes in metadata. Managed folders don't have a generation number.
    #[prost(int64, tag = "3")]
    pub metageneration: i64,
    /// Output only. The creation time of the managed folder.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The modification time of the managed folder.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for GetManagedFolder.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagedFolderRequest {
    /// Required. Name of the managed folder.
    /// Format:
    /// `projects/{project}/buckets/{bucket}/managedFolders/{managedFolder}`
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// The operation succeeds conditional on the managed folder's current
    /// metageneration matching the value here specified.
    #[prost(int64, optional, tag = "3")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// The operation succeeds conditional on the managed folder's current
    /// metageneration NOT matching the value here specified.
    #[prost(int64, optional, tag = "4")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Optional. A unique identifier for this request. UUID is the recommended
    /// format, but other formats are still accepted.
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for CreateManagedFolder.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateManagedFolderRequest {
    /// Required. Name of the bucket this managed folder belongs to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Properties of the managed folder being created.
    /// The bucket and managed folder names are specified in the `parent` and
    /// `managed_folder_id` fields. Populating these fields in `managed_folder`
    /// will result in an error.
    #[prost(message, optional, tag = "2")]
    pub managed_folder: ::core::option::Option<ManagedFolder>,
    /// Required. The name of the managed folder. It uses a single `/` as delimiter
    /// and leading and trailing `/` are allowed.
    #[prost(string, tag = "3")]
    pub managed_folder_id: ::prost::alloc::string::String,
    /// Optional. A unique identifier for this request. UUID is the recommended
    /// format, but other formats are still accepted.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// DeleteManagedFolder RPC request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteManagedFolderRequest {
    /// Required. Name of the managed folder.
    /// Format:
    /// `projects/{project}/buckets/{bucket}/managedFolders/{managedFolder}`
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// The operation succeeds conditional on the managed folder's current
    /// metageneration matching the value here specified.
    #[prost(int64, optional, tag = "3")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// The operation succeeds conditional on the managed folder's current
    /// metageneration NOT matching the value here specified.
    #[prost(int64, optional, tag = "4")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Allows deletion of a managed folder even if it is not empty.
    /// A managed folder is empty if it manages no child managed folders or
    /// objects. Caller must have permission for
    /// storage.managedFolders.setIamPolicy.
    #[prost(bool, tag = "5")]
    pub allow_non_empty: bool,
    /// Optional. A unique identifier for this request. UUID is the recommended
    /// format, but other formats are still accepted.
    #[prost(string, tag = "6")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for ListManagedFolders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListManagedFoldersRequest {
    /// Required. Name of the bucket this managed folder belongs to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of managed folders to return in a single response.
    /// The service will use this parameter or 1,000 items, whichever is smaller.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A previously-returned page token representing part of the larger
    /// set of results to view.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter results to match managed folders with name starting with
    /// this prefix.
    #[prost(string, tag = "4")]
    pub prefix: ::prost::alloc::string::String,
    /// Optional. A unique identifier for this request. UUID is the recommended
    /// format, but other formats are still accepted.
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
}
/// Response message for ListManagedFolders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListManagedFoldersResponse {
    /// The list of matching managed folders
    #[prost(message, repeated, tag = "1")]
    pub managed_folders: ::prost::alloc::vec::Vec<ManagedFolder>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod storage_control_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// StorageControl service includes selected control plane operations.
    #[derive(Debug, Clone)]
    pub struct StorageControlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageControlClient<tonic::transport::Channel> {
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
    impl<T> StorageControlClient<T>
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
        ) -> StorageControlClient<InterceptedService<T, F>>
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
            StorageControlClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new folder. This operation is only applicable to a hierarchical
        /// namespace enabled bucket.
        pub async fn create_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFolderRequest>,
        ) -> std::result::Result<tonic::Response<super::Folder>, tonic::Status> {
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
                "/google.storage.control.v2.StorageControl/CreateFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.storage.control.v2.StorageControl",
                        "CreateFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Permanently deletes an empty folder. This operation is only applicable to a
        /// hierarchical namespace enabled bucket.
        pub async fn delete_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFolderRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.storage.control.v2.StorageControl/DeleteFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.storage.control.v2.StorageControl",
                        "DeleteFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns metadata for the specified folder. This operation is only
        /// applicable to a hierarchical namespace enabled bucket.
        pub async fn get_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderRequest>,
        ) -> std::result::Result<tonic::Response<super::Folder>, tonic::Status> {
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
                "/google.storage.control.v2.StorageControl/GetFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.storage.control.v2.StorageControl",
                        "GetFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a list of folders. This operation is only applicable to a
        /// hierarchical namespace enabled bucket.
        pub async fn list_folders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFoldersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFoldersResponse>,
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
                "/google.storage.control.v2.StorageControl/ListFolders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.storage.control.v2.StorageControl",
                        "ListFolders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Renames a source folder to a destination folder. This operation is only
        /// applicable to a hierarchical namespace enabled bucket. During a rename, the
        /// source and destination folders are locked until the long running operation
        /// completes.
        pub async fn rename_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameFolderRequest>,
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
                "/google.storage.control.v2.StorageControl/RenameFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.storage.control.v2.StorageControl",
                        "RenameFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the storage layout configuration for a given bucket.
        pub async fn get_storage_layout(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStorageLayoutRequest>,
        ) -> std::result::Result<tonic::Response<super::StorageLayout>, tonic::Status> {
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
                "/google.storage.control.v2.StorageControl/GetStorageLayout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.storage.control.v2.StorageControl",
                        "GetStorageLayout",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new managed folder.
        pub async fn create_managed_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateManagedFolderRequest>,
        ) -> std::result::Result<tonic::Response<super::ManagedFolder>, tonic::Status> {
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
                "/google.storage.control.v2.StorageControl/CreateManagedFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.storage.control.v2.StorageControl",
                        "CreateManagedFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Permanently deletes an empty managed folder.
        pub async fn delete_managed_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteManagedFolderRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.storage.control.v2.StorageControl/DeleteManagedFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.storage.control.v2.StorageControl",
                        "DeleteManagedFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns metadata for the specified managed folder.
        pub async fn get_managed_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::GetManagedFolderRequest>,
        ) -> std::result::Result<tonic::Response<super::ManagedFolder>, tonic::Status> {
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
                "/google.storage.control.v2.StorageControl/GetManagedFolder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.storage.control.v2.StorageControl",
                        "GetManagedFolder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a list of managed folders for a given bucket.
        pub async fn list_managed_folders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListManagedFoldersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListManagedFoldersResponse>,
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
                "/google.storage.control.v2.StorageControl/ListManagedFolders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.storage.control.v2.StorageControl",
                        "ListManagedFolders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
