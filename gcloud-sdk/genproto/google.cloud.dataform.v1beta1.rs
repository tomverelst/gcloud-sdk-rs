/// Represents a Dataform Git repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repository {
    /// Output only. The repository's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The repository's user-friendly name.
    #[prost(string, tag = "8")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. If set, configures this repository to be linked to a Git remote.
    #[prost(message, optional, tag = "2")]
    pub git_remote_settings: ::core::option::Option<repository::GitRemoteSettings>,
    /// Optional. The name of the Secret Manager secret version to be used to
    /// interpolate variables into the .npmrc file for package installation
    /// operations. Must be in the format `projects/*/secrets/*/versions/*`. The
    /// file itself must be in a JSON format.
    #[prost(string, tag = "3")]
    pub npmrc_environment_variables_secret_version: ::prost::alloc::string::String,
    /// Optional. If set, fields of `workspace_compilation_overrides` override the
    /// default compilation settings that are specified in dataform.json when
    /// creating workspace-scoped compilation results. See documentation for
    /// `WorkspaceCompilationOverrides` for more information.
    #[prost(message, optional, tag = "4")]
    pub workspace_compilation_overrides: ::core::option::Option<
        repository::WorkspaceCompilationOverrides,
    >,
    /// Optional. Repository user labels.
    #[prost(map = "string, string", tag = "5")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Input only. If set to true, the authenticated user will be
    /// granted the roles/dataform.admin role on the created repository. To modify
    /// access to the created repository later apply setIamPolicy from
    /// <https://cloud.google.com/dataform/reference/rest#rest-resource:-v1beta1.projects.locations.repositories>
    #[prost(bool, tag = "9")]
    pub set_authenticated_user_admin: bool,
    /// Optional. The service account to run workflow invocations under.
    #[prost(string, tag = "10")]
    pub service_account: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Repository`.
pub mod repository {
    /// Controls Git remote configuration for a repository.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GitRemoteSettings {
        /// Required. The Git remote's URL.
        #[prost(string, tag = "1")]
        pub url: ::prost::alloc::string::String,
        /// Required. The Git remote's default branch name.
        #[prost(string, tag = "2")]
        pub default_branch: ::prost::alloc::string::String,
        /// Optional. The name of the Secret Manager secret version to use as an
        /// authentication token for Git operations. Must be in the format
        /// `projects/*/secrets/*/versions/*`.
        #[prost(string, tag = "3")]
        pub authentication_token_secret_version: ::prost::alloc::string::String,
        /// Optional. Authentication fields for remote uris using SSH protocol.
        #[prost(message, optional, tag = "5")]
        pub ssh_authentication_config: ::core::option::Option<
            git_remote_settings::SshAuthenticationConfig,
        >,
        /// Output only. Deprecated: The field does not contain any token status
        /// information. Instead use
        /// <https://cloud.google.com/dataform/reference/rest/v1beta1/projects.locations.repositories/computeAccessTokenStatus>
        #[deprecated]
        #[prost(enumeration = "git_remote_settings::TokenStatus", tag = "4")]
        pub token_status: i32,
    }
    /// Nested message and enum types in `GitRemoteSettings`.
    pub mod git_remote_settings {
        /// Configures fields for performing SSH authentication.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SshAuthenticationConfig {
            /// Required. The name of the Secret Manager secret version to use as a
            /// ssh private key for Git operations.
            /// Must be in the format `projects/*/secrets/*/versions/*`.
            #[prost(string, tag = "1")]
            pub user_private_key_secret_version: ::prost::alloc::string::String,
            /// Required. Content of a public SSH key to verify an identity of a remote
            /// Git host.
            #[prost(string, tag = "2")]
            pub host_public_key: ::prost::alloc::string::String,
        }
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
        pub enum TokenStatus {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// The token could not be found in Secret Manager (or the Dataform
            /// Service Account did not have permission to access it).
            NotFound = 1,
            /// The token could not be used to authenticate against the Git remote.
            Invalid = 2,
            /// The token was used successfully to authenticate against the Git remote.
            Valid = 3,
        }
        impl TokenStatus {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    TokenStatus::Unspecified => "TOKEN_STATUS_UNSPECIFIED",
                    TokenStatus::NotFound => "NOT_FOUND",
                    TokenStatus::Invalid => "INVALID",
                    TokenStatus::Valid => "VALID",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TOKEN_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                    "NOT_FOUND" => Some(Self::NotFound),
                    "INVALID" => Some(Self::Invalid),
                    "VALID" => Some(Self::Valid),
                    _ => None,
                }
            }
        }
    }
    /// Configures workspace compilation overrides for a repository.
    /// Primarily used by the UI (`console.cloud.google.com`).
    /// `schema_suffix` and `table_prefix` can have a special expression -
    /// `${workspaceName}`, which refers to the workspace name from which the
    /// compilation results will be created. API callers are expected to resolve
    /// the expression in these overrides and provide them explicitly in
    /// `code_compilation_config`
    /// (<https://cloud.google.com/dataform/reference/rest/v1beta1/projects.locations.repositories.compilationResults#codecompilationconfig>)
    /// when creating workspace-scoped compilation results.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WorkspaceCompilationOverrides {
        /// Optional. The default database (Google Cloud project ID).
        #[prost(string, tag = "1")]
        pub default_database: ::prost::alloc::string::String,
        /// Optional. The suffix that should be appended to all schema (BigQuery
        /// dataset ID) names.
        #[prost(string, tag = "2")]
        pub schema_suffix: ::prost::alloc::string::String,
        /// Optional. The prefix that should be prepended to all table names.
        #[prost(string, tag = "3")]
        pub table_prefix: ::prost::alloc::string::String,
    }
}
/// `ListRepositories` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesRequest {
    /// Required. The location in which to list repositories. Must be in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of repositories to return. The server may return
    /// fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListRepositories` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListRepositories`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. This field only supports ordering by `name`. If unspecified, the
    /// server will choose the ordering. If specified, the default order is
    /// ascending for the `name` field.
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Filter for the returned list.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// `ListRepositories` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesResponse {
    /// List of repositories.
    #[prost(message, repeated, tag = "1")]
    pub repositories: ::prost::alloc::vec::Vec<Repository>,
    /// A token which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations which could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `GetRepository` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRepositoryRequest {
    /// Required. The repository's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `CreateRepository` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRepositoryRequest {
    /// Required. The location in which to create the repository. Must be in the
    /// format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The repository to create.
    #[prost(message, optional, tag = "2")]
    pub repository: ::core::option::Option<Repository>,
    /// Required. The ID to use for the repository, which will become the final
    /// component of the repository's resource name.
    #[prost(string, tag = "3")]
    pub repository_id: ::prost::alloc::string::String,
}
/// `UpdateRepository` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRepositoryRequest {
    /// Optional. Specifies the fields to be updated in the repository. If left
    /// unset, all fields will be updated.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The repository to update.
    #[prost(message, optional, tag = "2")]
    pub repository: ::core::option::Option<Repository>,
}
/// `DeleteRepository` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRepositoryRequest {
    /// Required. The repository's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, any child resources of this repository will also be
    /// deleted. (Otherwise, the request will only succeed if the repository has no
    /// child resources.)
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// `CommitRepositoryChanges` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitRepositoryChangesRequest {
    /// Required. The repository's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The changes to commit to the repository.
    #[prost(message, optional, tag = "2")]
    pub commit_metadata: ::core::option::Option<CommitMetadata>,
    /// Optional. The commit SHA which must be the repository's current HEAD before
    /// applying this commit; otherwise this request will fail. If unset, no
    /// validation on the current HEAD commit SHA is performed.
    #[prost(string, tag = "4")]
    pub required_head_commit_sha: ::prost::alloc::string::String,
    /// A map to the path of the file to the operation. The path is the full file
    /// path including filename, from repository root.
    #[prost(map = "string, message", tag = "3")]
    pub file_operations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        commit_repository_changes_request::FileOperation,
    >,
}
/// Nested message and enum types in `CommitRepositoryChangesRequest`.
pub mod commit_repository_changes_request {
    /// Represents a single file operation to the repository.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FileOperation {
        #[prost(oneof = "file_operation::Operation", tags = "1, 2")]
        pub operation: ::core::option::Option<file_operation::Operation>,
    }
    /// Nested message and enum types in `FileOperation`.
    pub mod file_operation {
        /// Represents the write file operation (for files added or modified).
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct WriteFile {
            /// The file's contents.
            #[prost(bytes = "vec", tag = "1")]
            pub contents: ::prost::alloc::vec::Vec<u8>,
        }
        /// Represents the delete file operation.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DeleteFile {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Operation {
            /// Represents the write operation.
            #[prost(message, tag = "1")]
            WriteFile(WriteFile),
            /// Represents the delete operation.
            #[prost(message, tag = "2")]
            DeleteFile(DeleteFile),
        }
    }
}
/// `ReadRepositoryFile` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRepositoryFileRequest {
    /// Required. The repository's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The commit SHA for the commit to read from. If unset, the file
    /// will be read from HEAD.
    #[prost(string, tag = "2")]
    pub commit_sha: ::prost::alloc::string::String,
    /// Required. Full file path to read including filename, from repository root.
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
}
/// `ReadRepositoryFile` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRepositoryFileResponse {
    /// The file's contents.
    #[prost(bytes = "vec", tag = "1")]
    pub contents: ::prost::alloc::vec::Vec<u8>,
}
/// `QueryRepositoryDirectoryContents` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRepositoryDirectoryContentsRequest {
    /// Required. The repository's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The Commit SHA for the commit to query from. If unset, the
    /// directory will be queried from HEAD.
    #[prost(string, tag = "2")]
    pub commit_sha: ::prost::alloc::string::String,
    /// Optional. The directory's full path including directory name, relative to
    /// root. If left unset, the root is used.
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
    /// Optional. Maximum number of paths to return. The server may return fewer
    /// items than requested. If unspecified, the server will pick an appropriate
    /// default.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. Page token received from a previous
    /// `QueryRepositoryDirectoryContents` call. Provide this to retrieve the
    /// subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `QueryRepositoryDirectoryContents` must match the call that provided the
    /// page token.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// `QueryRepositoryDirectoryContents` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRepositoryDirectoryContentsResponse {
    /// List of entries in the directory.
    #[prost(message, repeated, tag = "1")]
    pub directory_entries: ::prost::alloc::vec::Vec<DirectoryEntry>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// `FetchRepositoryHistory` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRepositoryHistoryRequest {
    /// Required. The repository's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Maximum number of commits to return. The server may return fewer
    /// items than requested. If unspecified, the server will pick an appropriate
    /// default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `FetchRepositoryHistory`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `FetchRepositoryHistory`
    /// must match the call that provided the page token.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// `FetchRepositoryHistory` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRepositoryHistoryResponse {
    /// A list of commit logs, ordered by 'git log' default order.
    #[prost(message, repeated, tag = "1")]
    pub commits: ::prost::alloc::vec::Vec<CommitLogEntry>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Represents a single commit log.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitLogEntry {
    /// Commit timestamp.
    #[prost(message, optional, tag = "1")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The commit SHA for this commit log entry.
    #[prost(string, tag = "2")]
    pub commit_sha: ::prost::alloc::string::String,
    /// The commit author for this commit log entry.
    #[prost(message, optional, tag = "3")]
    pub author: ::core::option::Option<CommitAuthor>,
    /// The commit message for this commit log entry.
    #[prost(string, tag = "4")]
    pub commit_message: ::prost::alloc::string::String,
}
/// Represents a Dataform Git commit.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitMetadata {
    /// Required. The commit's author.
    #[prost(message, optional, tag = "1")]
    pub author: ::core::option::Option<CommitAuthor>,
    /// Optional. The commit's message.
    #[prost(string, tag = "2")]
    pub commit_message: ::prost::alloc::string::String,
}
/// `ComputeRepositoryAccessTokenStatus` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRepositoryAccessTokenStatusRequest {
    /// Required. The repository's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `ComputeRepositoryAccessTokenStatus` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRepositoryAccessTokenStatusResponse {
    /// Indicates the status of the Git access token.
    #[prost(
        enumeration = "compute_repository_access_token_status_response::TokenStatus",
        tag = "1"
    )]
    pub token_status: i32,
}
/// Nested message and enum types in `ComputeRepositoryAccessTokenStatusResponse`.
pub mod compute_repository_access_token_status_response {
    /// Indicates the status of a Git authentication token.
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
    pub enum TokenStatus {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The token could not be found in Secret Manager (or the Dataform
        /// Service Account did not have permission to access it).
        NotFound = 1,
        /// The token could not be used to authenticate against the Git remote.
        Invalid = 2,
        /// The token was used successfully to authenticate against the Git remote.
        Valid = 3,
    }
    impl TokenStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TokenStatus::Unspecified => "TOKEN_STATUS_UNSPECIFIED",
                TokenStatus::NotFound => "NOT_FOUND",
                TokenStatus::Invalid => "INVALID",
                TokenStatus::Valid => "VALID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TOKEN_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "NOT_FOUND" => Some(Self::NotFound),
                "INVALID" => Some(Self::Invalid),
                "VALID" => Some(Self::Valid),
                _ => None,
            }
        }
    }
}
/// `FetchRemoteBranches` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRemoteBranchesRequest {
    /// Required. The repository's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `FetchRemoteBranches` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRemoteBranchesResponse {
    /// The remote repository's branch names.
    #[prost(string, repeated, tag = "1")]
    pub branches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a Dataform Git workspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workspace {
    /// Output only. The workspace's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `ListWorkspaces` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkspacesRequest {
    /// Required. The repository in which to list workspaces. Must be in the
    /// format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of workspaces to return. The server may return
    /// fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListWorkspaces` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListWorkspaces`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. This field only supports ordering by `name`. If unspecified, the
    /// server will choose the ordering. If specified, the default order is
    /// ascending for the `name` field.
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Filter for the returned list.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// `ListWorkspaces` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkspacesResponse {
    /// List of workspaces.
    #[prost(message, repeated, tag = "1")]
    pub workspaces: ::prost::alloc::vec::Vec<Workspace>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations which could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `GetWorkspace` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkspaceRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `CreateWorkspace` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkspaceRequest {
    /// Required. The repository in which to create the workspace. Must be in the
    /// format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The workspace to create.
    #[prost(message, optional, tag = "2")]
    pub workspace: ::core::option::Option<Workspace>,
    /// Required. The ID to use for the workspace, which will become the final
    /// component of the workspace's resource name.
    #[prost(string, tag = "3")]
    pub workspace_id: ::prost::alloc::string::String,
}
/// `DeleteWorkspace` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkspaceRequest {
    /// Required. The workspace resource's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the author of a Git commit.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitAuthor {
    /// Required. The commit author's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The commit author's email address.
    #[prost(string, tag = "2")]
    pub email_address: ::prost::alloc::string::String,
}
/// `PullGitCommits` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullGitCommitsRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The name of the branch in the Git remote from which to pull
    /// commits. If left unset, the repository's default branch name will be used.
    #[prost(string, tag = "2")]
    pub remote_branch: ::prost::alloc::string::String,
    /// Required. The author of any merge commit which may be created as a result
    /// of merging fetched Git commits into this workspace.
    #[prost(message, optional, tag = "3")]
    pub author: ::core::option::Option<CommitAuthor>,
}
/// `PushGitCommits` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushGitCommitsRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The name of the branch in the Git remote to which commits should
    /// be pushed. If left unset, the repository's default branch name will be
    /// used.
    #[prost(string, tag = "2")]
    pub remote_branch: ::prost::alloc::string::String,
}
/// `FetchFileGitStatuses` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchFileGitStatusesRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `FetchFileGitStatuses` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchFileGitStatusesResponse {
    /// A list of all files which have uncommitted Git changes. There will only be
    /// a single entry for any given file.
    #[prost(message, repeated, tag = "1")]
    pub uncommitted_file_changes: ::prost::alloc::vec::Vec<
        fetch_file_git_statuses_response::UncommittedFileChange,
    >,
}
/// Nested message and enum types in `FetchFileGitStatusesResponse`.
pub mod fetch_file_git_statuses_response {
    /// Represents the Git state of a file with uncommitted changes.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UncommittedFileChange {
        /// The file's full path including filename, relative to the workspace root.
        #[prost(string, tag = "1")]
        pub path: ::prost::alloc::string::String,
        /// Indicates the status of the file.
        #[prost(enumeration = "uncommitted_file_change::State", tag = "2")]
        pub state: i32,
    }
    /// Nested message and enum types in `UncommittedFileChange`.
    pub mod uncommitted_file_change {
        /// Indicates the status of an uncommitted file change.
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
            /// Default value. This value is unused.
            Unspecified = 0,
            /// The file has been newly added.
            Added = 1,
            /// The file has been deleted.
            Deleted = 2,
            /// The file has been modified.
            Modified = 3,
            /// The file contains merge conflicts.
            HasConflicts = 4,
        }
        impl State {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    State::Unspecified => "STATE_UNSPECIFIED",
                    State::Added => "ADDED",
                    State::Deleted => "DELETED",
                    State::Modified => "MODIFIED",
                    State::HasConflicts => "HAS_CONFLICTS",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "ADDED" => Some(Self::Added),
                    "DELETED" => Some(Self::Deleted),
                    "MODIFIED" => Some(Self::Modified),
                    "HAS_CONFLICTS" => Some(Self::HasConflicts),
                    _ => None,
                }
            }
        }
    }
}
/// `FetchGitAheadBehind` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchGitAheadBehindRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The name of the branch in the Git remote against which this
    /// workspace should be compared. If left unset, the repository's default
    /// branch name will be used.
    #[prost(string, tag = "2")]
    pub remote_branch: ::prost::alloc::string::String,
}
/// `FetchGitAheadBehind` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchGitAheadBehindResponse {
    /// The number of commits in the remote branch that are not in the workspace.
    #[prost(int32, tag = "1")]
    pub commits_ahead: i32,
    /// The number of commits in the workspace that are not in the remote branch.
    #[prost(int32, tag = "2")]
    pub commits_behind: i32,
}
/// `CommitWorkspaceChanges` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitWorkspaceChangesRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The commit's author.
    #[prost(message, optional, tag = "4")]
    pub author: ::core::option::Option<CommitAuthor>,
    /// Optional. The commit's message.
    #[prost(string, tag = "2")]
    pub commit_message: ::prost::alloc::string::String,
    /// Optional. Full file paths to commit including filename, rooted at workspace
    /// root. If left empty, all files will be committed.
    #[prost(string, repeated, tag = "3")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `ResetWorkspaceChanges` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetWorkspaceChangesRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Full file paths to reset back to their committed state including
    /// filename, rooted at workspace root. If left empty, all files will be reset.
    #[prost(string, repeated, tag = "2")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. If set to true, untracked files will be deleted.
    #[prost(bool, tag = "3")]
    pub clean: bool,
}
/// `FetchFileDiff` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchFileDiffRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The file's full path including filename, relative to the
    /// workspace root.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
/// `FetchFileDiff` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchFileDiffResponse {
    /// The raw formatted Git diff for the file.
    #[prost(string, tag = "1")]
    pub formatted_diff: ::prost::alloc::string::String,
}
/// `QueryDirectoryContents` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDirectoryContentsRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub workspace: ::prost::alloc::string::String,
    /// Optional. The directory's full path including directory name, relative to
    /// the workspace root. If left unset, the workspace root is used.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// Optional. Maximum number of paths to return. The server may return fewer
    /// items than requested. If unspecified, the server will pick an appropriate
    /// default.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `QueryDirectoryContents`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `QueryDirectoryContents` must match the call that provided the page
    /// token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// `QueryDirectoryContents` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDirectoryContentsResponse {
    /// List of entries in the directory.
    #[prost(message, repeated, tag = "1")]
    pub directory_entries: ::prost::alloc::vec::Vec<DirectoryEntry>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Represents a single entry in a directory.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectoryEntry {
    #[prost(oneof = "directory_entry::Entry", tags = "1, 2")]
    pub entry: ::core::option::Option<directory_entry::Entry>,
}
/// Nested message and enum types in `DirectoryEntry`.
pub mod directory_entry {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        /// A file in the directory.
        #[prost(string, tag = "1")]
        File(::prost::alloc::string::String),
        /// A child directory in the directory.
        #[prost(string, tag = "2")]
        Directory(::prost::alloc::string::String),
    }
}
/// `MakeDirectory` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakeDirectoryRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The directory's full path including directory name, relative to
    /// the workspace root.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
/// `MakeDirectory` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakeDirectoryResponse {}
/// `RemoveDirectory` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDirectoryRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The directory's full path including directory name, relative to
    /// the workspace root.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
/// `MoveDirectory` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveDirectoryRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The directory's full path including directory name, relative to
    /// the workspace root.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// Required. The new path for the directory including directory name, rooted
    /// at workspace root.
    #[prost(string, tag = "3")]
    pub new_path: ::prost::alloc::string::String,
}
/// `MoveDirectory` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveDirectoryResponse {}
/// `ReadFile` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadFileRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The file's full path including filename, relative to the
    /// workspace root.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
/// `ReadFile` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadFileResponse {
    /// The file's contents.
    #[prost(bytes = "vec", tag = "1")]
    pub file_contents: ::prost::alloc::vec::Vec<u8>,
}
/// `RemoveFile` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFileRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The file's full path including filename, relative to the
    /// workspace root.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
/// `MoveFile` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveFileRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The file's full path including filename, relative to the
    /// workspace root.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// Required. The file's new path including filename, relative to the workspace
    /// root.
    #[prost(string, tag = "3")]
    pub new_path: ::prost::alloc::string::String,
}
/// `MoveFile` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveFileResponse {}
/// `WriteFile` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteFileRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The file.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// Required. The file's contents.
    #[prost(bytes = "vec", tag = "3")]
    pub contents: ::prost::alloc::vec::Vec<u8>,
}
/// `WriteFile` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteFileResponse {}
/// `InstallNpmPackages` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallNpmPackagesRequest {
    /// Required. The workspace's name.
    #[prost(string, tag = "1")]
    pub workspace: ::prost::alloc::string::String,
}
/// `InstallNpmPackages` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallNpmPackagesResponse {}
/// Represents a Dataform release configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseConfig {
    /// Output only. The release config's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Git commit/tag/branch name at which the repository should be
    /// compiled. Must exist in the remote repository. Examples:
    /// - a commit SHA: `12ade345`
    /// - a tag: `tag1`
    /// - a branch name: `branch1`
    #[prost(string, tag = "2")]
    pub git_commitish: ::prost::alloc::string::String,
    /// Optional. If set, fields of `code_compilation_config` override the default
    /// compilation settings that are specified in dataform.json.
    #[prost(message, optional, tag = "3")]
    pub code_compilation_config: ::core::option::Option<CodeCompilationConfig>,
    /// Optional. Optional schedule (in cron format) for automatic creation of
    /// compilation results.
    #[prost(string, tag = "4")]
    pub cron_schedule: ::prost::alloc::string::String,
    /// Optional. Specifies the time zone to be used when interpreting
    /// cron_schedule. Must be a time zone name from the time zone database
    /// (<https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>). If left
    /// unspecified, the default is UTC.
    #[prost(string, tag = "7")]
    pub time_zone: ::prost::alloc::string::String,
    /// Output only. Records of the 10 most recent scheduled release attempts,
    /// ordered in in descending order of `release_time`. Updated whenever
    /// automatic creation of a compilation result is triggered by cron_schedule.
    #[prost(message, repeated, tag = "5")]
    pub recent_scheduled_release_records: ::prost::alloc::vec::Vec<
        release_config::ScheduledReleaseRecord,
    >,
    /// Optional. The name of the currently released compilation result for this
    /// release config. This value is updated when a compilation result is created
    /// from this release config, or when this resource is updated by API call
    /// (perhaps to roll back to an earlier release). The compilation result must
    /// have been created using this release config. Must be in the format
    /// `projects/*/locations/*/repositories/*/compilationResults/*`.
    #[prost(string, tag = "6")]
    pub release_compilation_result: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ReleaseConfig`.
pub mod release_config {
    /// A record of an attempt to create a compilation result for this release
    /// config.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ScheduledReleaseRecord {
        /// The timestamp of this release attempt.
        #[prost(message, optional, tag = "1")]
        pub release_time: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(oneof = "scheduled_release_record::Result", tags = "2, 3")]
        pub result: ::core::option::Option<scheduled_release_record::Result>,
    }
    /// Nested message and enum types in `ScheduledReleaseRecord`.
    pub mod scheduled_release_record {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Result {
            /// The name of the created compilation result, if one was successfully
            /// created. Must be in the format
            /// `projects/*/locations/*/repositories/*/compilationResults/*`.
            #[prost(string, tag = "2")]
            CompilationResult(::prost::alloc::string::String),
            /// The error status encountered upon this attempt to create the
            /// compilation result, if the attempt was unsuccessful.
            #[prost(message, tag = "3")]
            ErrorStatus(super::super::super::super::super::rpc::Status),
        }
    }
}
/// `ListReleaseConfigs` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReleaseConfigsRequest {
    /// Required. The repository in which to list release configs. Must be in the
    /// format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of release configs to return. The server may
    /// return fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListReleaseConfigs` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListReleaseConfigs`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// `ListReleaseConfigs` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReleaseConfigsResponse {
    /// List of release configs.
    #[prost(message, repeated, tag = "1")]
    pub release_configs: ::prost::alloc::vec::Vec<ReleaseConfig>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations which could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `GetReleaseConfig` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReleaseConfigRequest {
    /// Required. The release config's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `CreateReleaseConfig` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReleaseConfigRequest {
    /// Required. The repository in which to create the release config. Must be in
    /// the format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The release config to create.
    #[prost(message, optional, tag = "2")]
    pub release_config: ::core::option::Option<ReleaseConfig>,
    /// Required. The ID to use for the release config, which will become the final
    /// component of the release config's resource name.
    #[prost(string, tag = "3")]
    pub release_config_id: ::prost::alloc::string::String,
}
/// `UpdateReleaseConfig` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReleaseConfigRequest {
    /// Optional. Specifies the fields to be updated in the release config. If left
    /// unset, all fields will be updated.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The release config to update.
    #[prost(message, optional, tag = "2")]
    pub release_config: ::core::option::Option<ReleaseConfig>,
}
/// `DeleteReleaseConfig` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReleaseConfigRequest {
    /// Required. The release config's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the result of compiling a Dataform project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompilationResult {
    /// Output only. The compilation result's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. If set, fields of `code_compilation_config` override the default
    /// compilation settings that are specified in dataform.json.
    #[prost(message, optional, tag = "4")]
    pub code_compilation_config: ::core::option::Option<CodeCompilationConfig>,
    /// Output only. The fully resolved Git commit SHA of the code that was
    /// compiled. Not set for compilation results whose source is a workspace.
    #[prost(string, tag = "8")]
    pub resolved_git_commit_sha: ::prost::alloc::string::String,
    /// Output only. The version of `@dataform/core` that was used for compilation.
    #[prost(string, tag = "5")]
    pub dataform_core_version: ::prost::alloc::string::String,
    /// Output only. Errors encountered during project compilation.
    #[prost(message, repeated, tag = "6")]
    pub compilation_errors: ::prost::alloc::vec::Vec<
        compilation_result::CompilationError,
    >,
    #[prost(oneof = "compilation_result::Source", tags = "2, 3, 7")]
    pub source: ::core::option::Option<compilation_result::Source>,
}
/// Nested message and enum types in `CompilationResult`.
pub mod compilation_result {
    /// An error encountered when attempting to compile a Dataform project.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompilationError {
        /// Output only. The error's top level message.
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
        /// Output only. The error's full stack trace.
        #[prost(string, tag = "2")]
        pub stack: ::prost::alloc::string::String,
        /// Output only. The path of the file where this error occurred, if
        /// available, relative to the project root.
        #[prost(string, tag = "3")]
        pub path: ::prost::alloc::string::String,
        /// Output only. The identifier of the action where this error occurred, if
        /// available.
        #[prost(message, optional, tag = "4")]
        pub action_target: ::core::option::Option<super::Target>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Immutable. Git commit/tag/branch name at which the repository should be
        /// compiled. Must exist in the remote repository. Examples:
        /// - a commit SHA: `12ade345`
        /// - a tag: `tag1`
        /// - a branch name: `branch1`
        #[prost(string, tag = "2")]
        GitCommitish(::prost::alloc::string::String),
        /// Immutable. The name of the workspace to compile. Must be in the format
        /// `projects/*/locations/*/repositories/*/workspaces/*`.
        #[prost(string, tag = "3")]
        Workspace(::prost::alloc::string::String),
        /// Immutable. The name of the release config to compile. The release
        /// config's 'current_compilation_result' field will be updated to this
        /// compilation result. Must be in the format
        /// `projects/*/locations/*/repositories/*/releaseConfigs/*`.
        #[prost(string, tag = "7")]
        ReleaseConfig(::prost::alloc::string::String),
    }
}
/// Configures various aspects of Dataform code compilation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeCompilationConfig {
    /// Optional. The default database (Google Cloud project ID).
    #[prost(string, tag = "1")]
    pub default_database: ::prost::alloc::string::String,
    /// Optional. The default schema (BigQuery dataset ID).
    #[prost(string, tag = "2")]
    pub default_schema: ::prost::alloc::string::String,
    /// Optional. The default BigQuery location to use. Defaults to "US".
    /// See the BigQuery docs for a full list of locations:
    /// <https://cloud.google.com/bigquery/docs/locations.>
    #[prost(string, tag = "8")]
    pub default_location: ::prost::alloc::string::String,
    /// Optional. The default schema (BigQuery dataset ID) for assertions.
    #[prost(string, tag = "3")]
    pub assertion_schema: ::prost::alloc::string::String,
    /// Optional. User-defined variables that are made available to project code
    /// during compilation.
    #[prost(map = "string, string", tag = "4")]
    pub vars: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The suffix that should be appended to all database (Google Cloud
    /// project ID) names.
    #[prost(string, tag = "5")]
    pub database_suffix: ::prost::alloc::string::String,
    /// Optional. The suffix that should be appended to all schema (BigQuery
    /// dataset ID) names.
    #[prost(string, tag = "6")]
    pub schema_suffix: ::prost::alloc::string::String,
    /// Optional. The prefix that should be prepended to all table names.
    #[prost(string, tag = "7")]
    pub table_prefix: ::prost::alloc::string::String,
}
/// `ListCompilationResults` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCompilationResultsRequest {
    /// Required. The repository in which to list compilation results. Must be in
    /// the format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of compilation results to return. The server may
    /// return fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListCompilationResults`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCompilationResults`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// `ListCompilationResults` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCompilationResultsResponse {
    /// List of compilation results.
    #[prost(message, repeated, tag = "1")]
    pub compilation_results: ::prost::alloc::vec::Vec<CompilationResult>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations which could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `GetCompilationResult` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCompilationResultRequest {
    /// Required. The compilation result's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `CreateCompilationResult` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCompilationResultRequest {
    /// Required. The repository in which to create the compilation result. Must be
    /// in the format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The compilation result to create.
    #[prost(message, optional, tag = "2")]
    pub compilation_result: ::core::option::Option<CompilationResult>,
}
/// Represents an action identifier. If the action writes output, the output
/// will be written to the referenced database object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Target {
    /// The action's database (Google Cloud project ID) .
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// The action's schema (BigQuery dataset ID), within `database`.
    #[prost(string, tag = "2")]
    pub schema: ::prost::alloc::string::String,
    /// The action's name, within `database` and `schema`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// Describes a relation and its columns.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationDescriptor {
    /// A text description of the relation.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// A list of descriptions of columns within the relation.
    #[prost(message, repeated, tag = "2")]
    pub columns: ::prost::alloc::vec::Vec<relation_descriptor::ColumnDescriptor>,
    /// A set of BigQuery labels that should be applied to the relation.
    #[prost(map = "string, string", tag = "3")]
    pub bigquery_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `RelationDescriptor`.
pub mod relation_descriptor {
    /// Describes a column.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ColumnDescriptor {
        /// The identifier for the column. Each entry in `path` represents one level
        /// of nesting.
        #[prost(string, repeated, tag = "1")]
        pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// A textual description of the column.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// A list of BigQuery policy tags that will be applied to the column.
        #[prost(string, repeated, tag = "3")]
        pub bigquery_policy_tags: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
}
/// Represents a single Dataform action in a compilation result.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompilationResultAction {
    /// This action's identifier. Unique within the compilation result.
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<Target>,
    /// The action's identifier if the project had been compiled without any
    /// overrides configured. Unique within the compilation result.
    #[prost(message, optional, tag = "2")]
    pub canonical_target: ::core::option::Option<Target>,
    /// The full path including filename in which this action is located, relative
    /// to the workspace root.
    #[prost(string, tag = "3")]
    pub file_path: ::prost::alloc::string::String,
    #[prost(oneof = "compilation_result_action::CompiledObject", tags = "4, 5, 6, 7")]
    pub compiled_object: ::core::option::Option<
        compilation_result_action::CompiledObject,
    >,
}
/// Nested message and enum types in `CompilationResultAction`.
pub mod compilation_result_action {
    /// Represents a database relation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Relation {
        /// A list of actions that this action depends on.
        #[prost(message, repeated, tag = "1")]
        pub dependency_targets: ::prost::alloc::vec::Vec<super::Target>,
        /// Whether this action is disabled (i.e. should not be run).
        #[prost(bool, tag = "2")]
        pub disabled: bool,
        /// Arbitrary, user-defined tags on this action.
        #[prost(string, repeated, tag = "3")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Descriptor for the relation and its columns.
        #[prost(message, optional, tag = "4")]
        pub relation_descriptor: ::core::option::Option<super::RelationDescriptor>,
        /// The type of this relation.
        #[prost(enumeration = "relation::RelationType", tag = "5")]
        pub relation_type: i32,
        /// The SELECT query which returns rows which this relation should contain.
        #[prost(string, tag = "6")]
        pub select_query: ::prost::alloc::string::String,
        /// SQL statements to be executed before creating the relation.
        #[prost(string, repeated, tag = "7")]
        pub pre_operations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// SQL statements to be executed after creating the relation.
        #[prost(string, repeated, tag = "8")]
        pub post_operations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Configures `INCREMENTAL_TABLE` settings for this relation. Only set if
        /// `relation_type` is `INCREMENTAL_TABLE`.
        #[prost(message, optional, tag = "9")]
        pub incremental_table_config: ::core::option::Option<
            relation::IncrementalTableConfig,
        >,
        /// The SQL expression used to partition the relation.
        #[prost(string, tag = "10")]
        pub partition_expression: ::prost::alloc::string::String,
        /// A list of columns or SQL expressions used to cluster the table.
        #[prost(string, repeated, tag = "11")]
        pub cluster_expressions: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Sets the partition expiration in days.
        #[prost(int32, tag = "12")]
        pub partition_expiration_days: i32,
        /// Specifies whether queries on this table must include a predicate filter
        /// that filters on the partitioning column.
        #[prost(bool, tag = "13")]
        pub require_partition_filter: bool,
        /// Additional options that will be provided as key/value pairs into the
        /// options clause of a create table/view statement. See
        /// <https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language>
        /// for more information on which options are supported.
        #[prost(map = "string, string", tag = "14")]
        pub additional_options: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
    /// Nested message and enum types in `Relation`.
    pub mod relation {
        /// Contains settings for relations of type `INCREMENTAL_TABLE`.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IncrementalTableConfig {
            /// The SELECT query which returns rows which should be inserted into the
            /// relation if it already exists and is not being refreshed.
            #[prost(string, tag = "1")]
            pub incremental_select_query: ::prost::alloc::string::String,
            /// Whether this table should be protected from being refreshed.
            #[prost(bool, tag = "2")]
            pub refresh_disabled: bool,
            /// A set of columns or SQL expressions used to define row uniqueness.
            /// If any duplicates are discovered (as defined by `unique_key_parts`),
            /// only the newly selected rows (as defined by `incremental_select_query`)
            /// will be included in the relation.
            #[prost(string, repeated, tag = "3")]
            pub unique_key_parts: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
            /// A SQL expression conditional used to limit the set of existing rows
            /// considered for a merge operation (see `unique_key_parts` for more
            /// information).
            #[prost(string, tag = "4")]
            pub update_partition_filter: ::prost::alloc::string::String,
            /// SQL statements to be executed before inserting new rows into the
            /// relation.
            #[prost(string, repeated, tag = "5")]
            pub incremental_pre_operations: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
            /// SQL statements to be executed after inserting new rows into the
            /// relation.
            #[prost(string, repeated, tag = "6")]
            pub incremental_post_operations: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
        }
        /// Indicates the type of this relation.
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
        pub enum RelationType {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// The relation is a table.
            Table = 1,
            /// The relation is a view.
            View = 2,
            /// The relation is an incrementalized table.
            IncrementalTable = 3,
            /// The relation is a materialized view.
            MaterializedView = 4,
        }
        impl RelationType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    RelationType::Unspecified => "RELATION_TYPE_UNSPECIFIED",
                    RelationType::Table => "TABLE",
                    RelationType::View => "VIEW",
                    RelationType::IncrementalTable => "INCREMENTAL_TABLE",
                    RelationType::MaterializedView => "MATERIALIZED_VIEW",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "RELATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "TABLE" => Some(Self::Table),
                    "VIEW" => Some(Self::View),
                    "INCREMENTAL_TABLE" => Some(Self::IncrementalTable),
                    "MATERIALIZED_VIEW" => Some(Self::MaterializedView),
                    _ => None,
                }
            }
        }
    }
    /// Represents a list of arbitrary database operations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Operations {
        /// A list of actions that this action depends on.
        #[prost(message, repeated, tag = "1")]
        pub dependency_targets: ::prost::alloc::vec::Vec<super::Target>,
        /// Whether this action is disabled (i.e. should not be run).
        #[prost(bool, tag = "2")]
        pub disabled: bool,
        /// Arbitrary, user-defined tags on this action.
        #[prost(string, repeated, tag = "3")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Descriptor for any output relation and its columns. Only set if
        /// `has_output` is true.
        #[prost(message, optional, tag = "6")]
        pub relation_descriptor: ::core::option::Option<super::RelationDescriptor>,
        /// A list of arbitrary SQL statements that will be executed without
        /// alteration.
        #[prost(string, repeated, tag = "4")]
        pub queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Whether these operations produce an output relation.
        #[prost(bool, tag = "5")]
        pub has_output: bool,
    }
    /// Represents an assertion upon a SQL query which is required return zero
    /// rows.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Assertion {
        /// A list of actions that this action depends on.
        #[prost(message, repeated, tag = "1")]
        pub dependency_targets: ::prost::alloc::vec::Vec<super::Target>,
        /// The parent action of this assertion. Only set if this assertion was
        /// automatically generated.
        #[prost(message, optional, tag = "5")]
        pub parent_action: ::core::option::Option<super::Target>,
        /// Whether this action is disabled (i.e. should not be run).
        #[prost(bool, tag = "2")]
        pub disabled: bool,
        /// Arbitrary, user-defined tags on this action.
        #[prost(string, repeated, tag = "3")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The SELECT query which must return zero rows in order for this assertion
        /// to succeed.
        #[prost(string, tag = "4")]
        pub select_query: ::prost::alloc::string::String,
        /// Descriptor for the assertion's automatically-generated view and its
        /// columns.
        #[prost(message, optional, tag = "6")]
        pub relation_descriptor: ::core::option::Option<super::RelationDescriptor>,
    }
    /// Represents a relation which is not managed by Dataform but which may be
    /// referenced by Dataform actions.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Declaration {
        /// Descriptor for the relation and its columns. Used as documentation only,
        /// i.e. values here will result in no changes to the relation's metadata.
        #[prost(message, optional, tag = "1")]
        pub relation_descriptor: ::core::option::Option<super::RelationDescriptor>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CompiledObject {
        /// The database relation created/updated by this action.
        #[prost(message, tag = "4")]
        Relation(Relation),
        /// The database operations executed by this action.
        #[prost(message, tag = "5")]
        Operations(Operations),
        /// The assertion executed by this action.
        #[prost(message, tag = "6")]
        Assertion(Assertion),
        /// The declaration declared by this action.
        #[prost(message, tag = "7")]
        Declaration(Declaration),
    }
}
/// `QueryCompilationResultActions` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCompilationResultActionsRequest {
    /// Required. The compilation result's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Maximum number of compilation results to return. The server may
    /// return fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous
    /// `QueryCompilationResultActions` call. Provide this to retrieve the
    /// subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `QueryCompilationResultActions` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Optional filter for the returned list. Filtering is only
    /// currently supported on the `file_path` field.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// `QueryCompilationResultActions` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCompilationResultActionsResponse {
    /// List of compilation result actions.
    #[prost(message, repeated, tag = "1")]
    pub compilation_result_actions: ::prost::alloc::vec::Vec<CompilationResultAction>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Represents a Dataform workflow configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowConfig {
    /// Output only. The workflow config's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the release config whose release_compilation_result
    /// should be executed. Must be in the format
    /// `projects/*/locations/*/repositories/*/releaseConfigs/*`.
    #[prost(string, tag = "2")]
    pub release_config: ::prost::alloc::string::String,
    /// Optional. If left unset, a default InvocationConfig will be used.
    #[prost(message, optional, tag = "3")]
    pub invocation_config: ::core::option::Option<InvocationConfig>,
    /// Optional. Optional schedule (in cron format) for automatic execution of
    /// this workflow config.
    #[prost(string, tag = "4")]
    pub cron_schedule: ::prost::alloc::string::String,
    /// Optional. Specifies the time zone to be used when interpreting
    /// cron_schedule. Must be a time zone name from the time zone database
    /// (<https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>). If left
    /// unspecified, the default is UTC.
    #[prost(string, tag = "7")]
    pub time_zone: ::prost::alloc::string::String,
    /// Output only. Records of the 10 most recent scheduled execution attempts,
    /// ordered in in descending order of `execution_time`. Updated whenever
    /// automatic creation of a workflow invocation is triggered by cron_schedule.
    #[prost(message, repeated, tag = "5")]
    pub recent_scheduled_execution_records: ::prost::alloc::vec::Vec<
        workflow_config::ScheduledExecutionRecord,
    >,
}
/// Nested message and enum types in `WorkflowConfig`.
pub mod workflow_config {
    /// A record of an attempt to create a workflow invocation for this workflow
    /// config.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ScheduledExecutionRecord {
        /// The timestamp of this execution attempt.
        #[prost(message, optional, tag = "1")]
        pub execution_time: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(oneof = "scheduled_execution_record::Result", tags = "2, 3")]
        pub result: ::core::option::Option<scheduled_execution_record::Result>,
    }
    /// Nested message and enum types in `ScheduledExecutionRecord`.
    pub mod scheduled_execution_record {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Result {
            /// The name of the created workflow invocation, if one was successfully
            /// created. Must be in the format
            /// `projects/*/locations/*/repositories/*/workflowInvocations/*`.
            #[prost(string, tag = "2")]
            WorkflowInvocation(::prost::alloc::string::String),
            /// The error status encountered upon this attempt to create the
            /// workflow invocation, if the attempt was unsuccessful.
            #[prost(message, tag = "3")]
            ErrorStatus(super::super::super::super::super::rpc::Status),
        }
    }
}
/// Includes various configuration options for a workflow invocation.
/// If both `included_targets` and `included_tags` are unset, all actions
/// will be included.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvocationConfig {
    /// Optional. The set of action identifiers to include.
    #[prost(message, repeated, tag = "1")]
    pub included_targets: ::prost::alloc::vec::Vec<Target>,
    /// Optional. The set of tags to include.
    #[prost(string, repeated, tag = "2")]
    pub included_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. When set to true, transitive dependencies of included actions
    /// will be executed.
    #[prost(bool, tag = "3")]
    pub transitive_dependencies_included: bool,
    /// Optional. When set to true, transitive dependents of included actions will
    /// be executed.
    #[prost(bool, tag = "4")]
    pub transitive_dependents_included: bool,
    /// Optional. When set to true, any incremental tables will be fully refreshed.
    #[prost(bool, tag = "5")]
    pub fully_refresh_incremental_tables_enabled: bool,
    /// Optional. The service account to run workflow invocations under.
    #[prost(string, tag = "6")]
    pub service_account: ::prost::alloc::string::String,
}
/// `ListWorkflowConfigs` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkflowConfigsRequest {
    /// Required. The repository in which to list workflow configs. Must be in the
    /// format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of workflow configs to return. The server may
    /// return fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListWorkflowConfigs` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListWorkflowConfigs`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// `ListWorkflowConfigs` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkflowConfigsResponse {
    /// List of workflow configs.
    #[prost(message, repeated, tag = "1")]
    pub workflow_configs: ::prost::alloc::vec::Vec<WorkflowConfig>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations which could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `GetWorkflowConfig` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkflowConfigRequest {
    /// Required. The workflow config's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `CreateWorkflowConfig` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkflowConfigRequest {
    /// Required. The repository in which to create the workflow config. Must be in
    /// the format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The workflow config to create.
    #[prost(message, optional, tag = "2")]
    pub workflow_config: ::core::option::Option<WorkflowConfig>,
    /// Required. The ID to use for the workflow config, which will become the
    /// final component of the workflow config's resource name.
    #[prost(string, tag = "3")]
    pub workflow_config_id: ::prost::alloc::string::String,
}
/// `UpdateWorkflowConfig` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkflowConfigRequest {
    /// Optional. Specifies the fields to be updated in the workflow config. If
    /// left unset, all fields will be updated.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The workflow config to update.
    #[prost(message, optional, tag = "2")]
    pub workflow_config: ::core::option::Option<WorkflowConfig>,
}
/// `DeleteWorkflowConfig` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkflowConfigRequest {
    /// Required. The workflow config's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents a single invocation of a compilation result.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowInvocation {
    /// Output only. The workflow invocation's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. If left unset, a default InvocationConfig will be used.
    #[prost(message, optional, tag = "3")]
    pub invocation_config: ::core::option::Option<InvocationConfig>,
    /// Output only. This workflow invocation's current state.
    #[prost(enumeration = "workflow_invocation::State", tag = "4")]
    pub state: i32,
    /// Output only. This workflow invocation's timing details.
    #[prost(message, optional, tag = "5")]
    pub invocation_timing: ::core::option::Option<super::super::super::r#type::Interval>,
    #[prost(oneof = "workflow_invocation::CompilationSource", tags = "2, 6")]
    pub compilation_source: ::core::option::Option<
        workflow_invocation::CompilationSource,
    >,
}
/// Nested message and enum types in `WorkflowInvocation`.
pub mod workflow_invocation {
    /// Represents the current state of a workflow invocation.
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
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The workflow invocation is currently running.
        Running = 1,
        /// The workflow invocation succeeded. A terminal state.
        Succeeded = 2,
        /// The workflow invocation was cancelled. A terminal state.
        Cancelled = 3,
        /// The workflow invocation failed. A terminal state.
        Failed = 4,
        /// The workflow invocation is being cancelled, but some actions are still
        /// running.
        Canceling = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Cancelled => "CANCELLED",
                State::Failed => "FAILED",
                State::Canceling => "CANCELING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "CANCELLED" => Some(Self::Cancelled),
                "FAILED" => Some(Self::Failed),
                "CANCELING" => Some(Self::Canceling),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CompilationSource {
        /// Immutable. The name of the compilation result to use for this invocation.
        /// Must be in the format
        /// `projects/*/locations/*/repositories/*/compilationResults/*`.
        #[prost(string, tag = "2")]
        CompilationResult(::prost::alloc::string::String),
        /// Immutable. The name of the workflow config to invoke. Must be in the
        /// format `projects/*/locations/*/repositories/*/workflowConfigs/*`.
        #[prost(string, tag = "6")]
        WorkflowConfig(::prost::alloc::string::String),
    }
}
/// `ListWorkflowInvocations` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkflowInvocationsRequest {
    /// Required. The parent resource of the WorkflowInvocation type. Must be in
    /// the format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of workflow invocations to return. The server may
    /// return fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListWorkflowInvocations`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListWorkflowInvocations`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. This field only supports ordering by `name`. If unspecified, the
    /// server will choose the ordering. If specified, the default order is
    /// ascending for the `name` field.
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Filter for the returned list.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// `ListWorkflowInvocations` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkflowInvocationsResponse {
    /// List of workflow invocations.
    #[prost(message, repeated, tag = "1")]
    pub workflow_invocations: ::prost::alloc::vec::Vec<WorkflowInvocation>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations which could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `GetWorkflowInvocation` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkflowInvocationRequest {
    /// Required. The workflow invocation resource's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `CreateWorkflowInvocation` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkflowInvocationRequest {
    /// Required. The repository in which to create the workflow invocation. Must
    /// be in the format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The workflow invocation resource to create.
    #[prost(message, optional, tag = "2")]
    pub workflow_invocation: ::core::option::Option<WorkflowInvocation>,
}
/// `DeleteWorkflowInvocation` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkflowInvocationRequest {
    /// Required. The workflow invocation resource's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// `CancelWorkflowInvocation` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelWorkflowInvocationRequest {
    /// Required. The workflow invocation resource's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents a single action in a workflow invocation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowInvocationAction {
    /// Output only. This action's identifier. Unique within the workflow
    /// invocation.
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<Target>,
    /// Output only. The action's identifier if the project had been compiled
    /// without any overrides configured. Unique within the compilation result.
    #[prost(message, optional, tag = "2")]
    pub canonical_target: ::core::option::Option<Target>,
    /// Output only. This action's current state.
    #[prost(enumeration = "workflow_invocation_action::State", tag = "4")]
    pub state: i32,
    /// Output only. If and only if action's state is FAILED a failure reason is
    /// set.
    #[prost(string, tag = "7")]
    pub failure_reason: ::prost::alloc::string::String,
    /// Output only. This action's timing details.
    /// `start_time` will be set if the action is in [RUNNING, SUCCEEDED,
    /// CANCELLED, FAILED] state.
    /// `end_time` will be set if the action is in \[SUCCEEDED, CANCELLED, FAILED\]
    /// state.
    #[prost(message, optional, tag = "5")]
    pub invocation_timing: ::core::option::Option<super::super::super::r#type::Interval>,
    /// Output only. The workflow action's bigquery action details.
    #[prost(message, optional, tag = "6")]
    pub bigquery_action: ::core::option::Option<
        workflow_invocation_action::BigQueryAction,
    >,
}
/// Nested message and enum types in `WorkflowInvocationAction`.
pub mod workflow_invocation_action {
    /// Represents a workflow action that will run against BigQuery.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryAction {
        /// Output only. The generated BigQuery SQL script that will be executed.
        #[prost(string, tag = "1")]
        pub sql_script: ::prost::alloc::string::String,
    }
    /// Represents the current state of a workflow invocation action.
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
        /// The action has not yet been considered for invocation.
        Pending = 0,
        /// The action is currently running.
        Running = 1,
        /// Execution of the action was skipped because upstream dependencies did not
        /// all complete successfully. A terminal state.
        Skipped = 2,
        /// Execution of the action was disabled as per the configuration of the
        /// corresponding compilation result action. A terminal state.
        Disabled = 3,
        /// The action succeeded. A terminal state.
        Succeeded = 4,
        /// The action was cancelled. A terminal state.
        Cancelled = 5,
        /// The action failed. A terminal state.
        Failed = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Pending => "PENDING",
                State::Running => "RUNNING",
                State::Skipped => "SKIPPED",
                State::Disabled => "DISABLED",
                State::Succeeded => "SUCCEEDED",
                State::Cancelled => "CANCELLED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PENDING" => Some(Self::Pending),
                "RUNNING" => Some(Self::Running),
                "SKIPPED" => Some(Self::Skipped),
                "DISABLED" => Some(Self::Disabled),
                "SUCCEEDED" => Some(Self::Succeeded),
                "CANCELLED" => Some(Self::Cancelled),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// `QueryWorkflowInvocationActions` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWorkflowInvocationActionsRequest {
    /// Required. The workflow invocation's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Maximum number of workflow invocations to return. The server may
    /// return fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous
    /// `QueryWorkflowInvocationActions` call. Provide this to retrieve the
    /// subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `QueryWorkflowInvocationActions` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// `QueryWorkflowInvocationActions` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWorkflowInvocationActionsResponse {
    /// List of workflow invocation actions.
    #[prost(message, repeated, tag = "1")]
    pub workflow_invocation_actions: ::prost::alloc::vec::Vec<WorkflowInvocationAction>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod dataform_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Dataform is a service to develop, create, document, test, and update curated
    /// tables in BigQuery.
    #[derive(Debug, Clone)]
    pub struct DataformClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataformClient<tonic::transport::Channel> {
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
    impl<T> DataformClient<T>
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
        ) -> DataformClient<InterceptedService<T, F>>
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
            DataformClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Repositories in a given project and location.
        pub async fn list_repositories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRepositoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRepositoriesResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/ListRepositories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "ListRepositories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a single Repository.
        pub async fn get_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRepositoryRequest>,
        ) -> std::result::Result<tonic::Response<super::Repository>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/GetRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "GetRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Repository in a given project and location.
        pub async fn create_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRepositoryRequest>,
        ) -> std::result::Result<tonic::Response<super::Repository>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/CreateRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "CreateRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a single Repository.
        pub async fn update_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRepositoryRequest>,
        ) -> std::result::Result<tonic::Response<super::Repository>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/UpdateRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "UpdateRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Repository.
        pub async fn delete_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRepositoryRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/DeleteRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "DeleteRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Applies a Git commit to a Repository. The Repository must not have a value
        /// for `git_remote_settings.url`.
        pub async fn commit_repository_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitRepositoryChangesRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/CommitRepositoryChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "CommitRepositoryChanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the contents of a file (inside a Repository). The Repository
        /// must not have a value for `git_remote_settings.url`.
        pub async fn read_repository_file(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRepositoryFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadRepositoryFileResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/ReadRepositoryFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "ReadRepositoryFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the contents of a given Repository directory. The Repository must
        /// not have a value for `git_remote_settings.url`.
        pub async fn query_repository_directory_contents(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryRepositoryDirectoryContentsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryRepositoryDirectoryContentsResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/QueryRepositoryDirectoryContents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "QueryRepositoryDirectoryContents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a Repository's history of commits.  The Repository must not have a
        /// value for `git_remote_settings.url`.
        pub async fn fetch_repository_history(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchRepositoryHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchRepositoryHistoryResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/FetchRepositoryHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "FetchRepositoryHistory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Computes a Repository's Git access token status.
        pub async fn compute_repository_access_token_status(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ComputeRepositoryAccessTokenStatusRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ComputeRepositoryAccessTokenStatusResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/ComputeRepositoryAccessTokenStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "ComputeRepositoryAccessTokenStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a Repository's remote branches.
        pub async fn fetch_remote_branches(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchRemoteBranchesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchRemoteBranchesResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/FetchRemoteBranches",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "FetchRemoteBranches",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Workspaces in a given Repository.
        pub async fn list_workspaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkspacesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkspacesResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/ListWorkspaces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "ListWorkspaces",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a single Workspace.
        pub async fn get_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkspaceRequest>,
        ) -> std::result::Result<tonic::Response<super::Workspace>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/GetWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "GetWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Workspace in a given Repository.
        pub async fn create_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkspaceRequest>,
        ) -> std::result::Result<tonic::Response<super::Workspace>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/CreateWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "CreateWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Workspace.
        pub async fn delete_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkspaceRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/DeleteWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "DeleteWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Installs dependency NPM packages (inside a Workspace).
        pub async fn install_npm_packages(
            &mut self,
            request: impl tonic::IntoRequest<super::InstallNpmPackagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InstallNpmPackagesResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/InstallNpmPackages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "InstallNpmPackages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Pulls Git commits from the Repository's remote into a Workspace.
        pub async fn pull_git_commits(
            &mut self,
            request: impl tonic::IntoRequest<super::PullGitCommitsRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/PullGitCommits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "PullGitCommits",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Pushes Git commits from a Workspace to the Repository's remote.
        pub async fn push_git_commits(
            &mut self,
            request: impl tonic::IntoRequest<super::PushGitCommitsRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/PushGitCommits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "PushGitCommits",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches Git statuses for the files in a Workspace.
        pub async fn fetch_file_git_statuses(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchFileGitStatusesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchFileGitStatusesResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/FetchFileGitStatuses",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "FetchFileGitStatuses",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches Git ahead/behind against a remote branch.
        pub async fn fetch_git_ahead_behind(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchGitAheadBehindRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchGitAheadBehindResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/FetchGitAheadBehind",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "FetchGitAheadBehind",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Applies a Git commit for uncommitted files in a Workspace.
        pub async fn commit_workspace_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitWorkspaceChangesRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/CommitWorkspaceChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "CommitWorkspaceChanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Performs a Git reset for uncommitted files in a Workspace.
        pub async fn reset_workspace_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetWorkspaceChangesRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/ResetWorkspaceChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "ResetWorkspaceChanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches Git diff for an uncommitted file in a Workspace.
        pub async fn fetch_file_diff(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchFileDiffRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchFileDiffResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/FetchFileDiff",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "FetchFileDiff",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the contents of a given Workspace directory.
        pub async fn query_directory_contents(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDirectoryContentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDirectoryContentsResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/QueryDirectoryContents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "QueryDirectoryContents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a directory inside a Workspace.
        pub async fn make_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::MakeDirectoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MakeDirectoryResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/MakeDirectory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "MakeDirectory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a directory (inside a Workspace) and all of its contents.
        pub async fn remove_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveDirectoryRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/RemoveDirectory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "RemoveDirectory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Moves a directory (inside a Workspace), and all of its contents, to a new
        /// location.
        pub async fn move_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveDirectoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MoveDirectoryResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/MoveDirectory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "MoveDirectory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the contents of a file (inside a Workspace).
        pub async fn read_file(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadFileResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/ReadFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dataform.v1beta1.Dataform", "ReadFile"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a file (inside a Workspace).
        pub async fn remove_file(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFileRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/RemoveFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "RemoveFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Moves a file (inside a Workspace) to a new location.
        pub async fn move_file(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MoveFileResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/MoveFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dataform.v1beta1.Dataform", "MoveFile"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Writes to a file (inside a Workspace).
        pub async fn write_file(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WriteFileResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/WriteFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "WriteFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists ReleaseConfigs in a given Repository.
        pub async fn list_release_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReleaseConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListReleaseConfigsResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/ListReleaseConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "ListReleaseConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a single ReleaseConfig.
        pub async fn get_release_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReleaseConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::ReleaseConfig>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/GetReleaseConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "GetReleaseConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new ReleaseConfig in a given Repository.
        pub async fn create_release_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReleaseConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::ReleaseConfig>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/CreateReleaseConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "CreateReleaseConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a single ReleaseConfig.
        pub async fn update_release_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateReleaseConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::ReleaseConfig>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/UpdateReleaseConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "UpdateReleaseConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single ReleaseConfig.
        pub async fn delete_release_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReleaseConfigRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/DeleteReleaseConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "DeleteReleaseConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists CompilationResults in a given Repository.
        pub async fn list_compilation_results(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCompilationResultsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCompilationResultsResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/ListCompilationResults",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "ListCompilationResults",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a single CompilationResult.
        pub async fn get_compilation_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCompilationResultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompilationResult>,
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
                "/google.cloud.dataform.v1beta1.Dataform/GetCompilationResult",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "GetCompilationResult",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new CompilationResult in a given project and location.
        pub async fn create_compilation_result(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCompilationResultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompilationResult>,
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
                "/google.cloud.dataform.v1beta1.Dataform/CreateCompilationResult",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "CreateCompilationResult",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns CompilationResultActions in a given CompilationResult.
        pub async fn query_compilation_result_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCompilationResultActionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCompilationResultActionsResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/QueryCompilationResultActions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "QueryCompilationResultActions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists WorkflowConfigs in a given Repository.
        pub async fn list_workflow_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkflowConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkflowConfigsResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/ListWorkflowConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "ListWorkflowConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a single WorkflowConfig.
        pub async fn get_workflow_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkflowConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::WorkflowConfig>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/GetWorkflowConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "GetWorkflowConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new WorkflowConfig in a given Repository.
        pub async fn create_workflow_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkflowConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::WorkflowConfig>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/CreateWorkflowConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "CreateWorkflowConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a single WorkflowConfig.
        pub async fn update_workflow_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkflowConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::WorkflowConfig>, tonic::Status> {
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
                "/google.cloud.dataform.v1beta1.Dataform/UpdateWorkflowConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "UpdateWorkflowConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single WorkflowConfig.
        pub async fn delete_workflow_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkflowConfigRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/DeleteWorkflowConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "DeleteWorkflowConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists WorkflowInvocations in a given Repository.
        pub async fn list_workflow_invocations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkflowInvocationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkflowInvocationsResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/ListWorkflowInvocations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "ListWorkflowInvocations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a single WorkflowInvocation.
        pub async fn get_workflow_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkflowInvocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WorkflowInvocation>,
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
                "/google.cloud.dataform.v1beta1.Dataform/GetWorkflowInvocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "GetWorkflowInvocation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new WorkflowInvocation in a given Repository.
        pub async fn create_workflow_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkflowInvocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WorkflowInvocation>,
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
                "/google.cloud.dataform.v1beta1.Dataform/CreateWorkflowInvocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "CreateWorkflowInvocation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single WorkflowInvocation.
        pub async fn delete_workflow_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkflowInvocationRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/DeleteWorkflowInvocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "DeleteWorkflowInvocation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Requests cancellation of a running WorkflowInvocation.
        pub async fn cancel_workflow_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelWorkflowInvocationRequest>,
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
                "/google.cloud.dataform.v1beta1.Dataform/CancelWorkflowInvocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "CancelWorkflowInvocation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns WorkflowInvocationActions in a given WorkflowInvocation.
        pub async fn query_workflow_invocation_actions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryWorkflowInvocationActionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryWorkflowInvocationActionsResponse>,
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
                "/google.cloud.dataform.v1beta1.Dataform/QueryWorkflowInvocationActions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataform.v1beta1.Dataform",
                        "QueryWorkflowInvocationActions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
