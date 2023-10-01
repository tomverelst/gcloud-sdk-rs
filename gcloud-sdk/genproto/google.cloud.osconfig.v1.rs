/// This API resource represents the available inventory data for a
/// Compute Engine virtual machine (VM) instance at a given point in time.
///
/// You can use this API resource to determine the inventory data of your VM.
///
/// For more information, see [Information provided by OS inventory
/// management](<https://cloud.google.com/compute/docs/instances/os-inventory-management#data-collected>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inventory {
    /// Output only. The `Inventory` API resource name.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/instances/{instance_id}/inventory`
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Base level operating system information for the VM.
    #[prost(message, optional, tag = "1")]
    pub os_info: ::core::option::Option<inventory::OsInfo>,
    /// Inventory items related to the VM keyed by an opaque unique identifier for
    /// each inventory item.  The identifier is unique to each distinct and
    /// addressable inventory item and will change, when there is a new package
    /// version.
    #[prost(map = "string, message", tag = "2")]
    pub items: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        inventory::Item,
    >,
    /// Output only. Timestamp of the last reported inventory for the VM.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Inventory`.
pub mod inventory {
    /// Operating system information for the VM.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsInfo {
        /// The VM hostname.
        #[prost(string, tag = "9")]
        pub hostname: ::prost::alloc::string::String,
        /// The operating system long name.
        /// For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019
        /// Datacenter'.
        #[prost(string, tag = "2")]
        pub long_name: ::prost::alloc::string::String,
        /// The operating system short name.
        /// For example, 'windows' or 'debian'.
        #[prost(string, tag = "3")]
        pub short_name: ::prost::alloc::string::String,
        /// The version of the operating system.
        #[prost(string, tag = "4")]
        pub version: ::prost::alloc::string::String,
        /// The system architecture of the operating system.
        #[prost(string, tag = "5")]
        pub architecture: ::prost::alloc::string::String,
        /// The kernel version of the operating system.
        #[prost(string, tag = "6")]
        pub kernel_version: ::prost::alloc::string::String,
        /// The kernel release of the operating system.
        #[prost(string, tag = "7")]
        pub kernel_release: ::prost::alloc::string::String,
        /// The current version of the OS Config agent running on the VM.
        #[prost(string, tag = "8")]
        pub osconfig_agent_version: ::prost::alloc::string::String,
    }
    /// A single piece of inventory on a VM.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        /// Identifier for this item, unique across items for this VM.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The origin of this inventory item.
        #[prost(enumeration = "item::OriginType", tag = "2")]
        pub origin_type: i32,
        /// When this inventory item was first detected.
        #[prost(message, optional, tag = "8")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// When this inventory item was last modified.
        #[prost(message, optional, tag = "9")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The specific type of inventory, correlating to its specific details.
        #[prost(enumeration = "item::Type", tag = "5")]
        pub r#type: i32,
        /// Specific details of this inventory item based on its type.
        #[prost(oneof = "item::Details", tags = "6, 7")]
        pub details: ::core::option::Option<item::Details>,
    }
    /// Nested message and enum types in `Item`.
    pub mod item {
        /// The origin of a specific inventory item.
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
        pub enum OriginType {
            /// Invalid. An origin type must be specified.
            Unspecified = 0,
            /// This inventory item was discovered as the result of the agent
            /// reporting inventory via the reporting API.
            InventoryReport = 1,
        }
        impl OriginType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    OriginType::Unspecified => "ORIGIN_TYPE_UNSPECIFIED",
                    OriginType::InventoryReport => "INVENTORY_REPORT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "ORIGIN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "INVENTORY_REPORT" => Some(Self::InventoryReport),
                    _ => None,
                }
            }
        }
        /// The different types of inventory that are tracked on a VM.
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
        pub enum Type {
            /// Invalid. An type must be specified.
            Unspecified = 0,
            /// This represents a package that is installed on the VM.
            InstalledPackage = 1,
            /// This represents an update that is available for a package.
            AvailablePackage = 2,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::InstalledPackage => "INSTALLED_PACKAGE",
                    Type::AvailablePackage => "AVAILABLE_PACKAGE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "INSTALLED_PACKAGE" => Some(Self::InstalledPackage),
                    "AVAILABLE_PACKAGE" => Some(Self::AvailablePackage),
                    _ => None,
                }
            }
        }
        /// Specific details of this inventory item based on its type.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Details {
            /// Software package present on the VM instance.
            #[prost(message, tag = "6")]
            InstalledPackage(super::SoftwarePackage),
            /// Software package available to be installed on the VM instance.
            #[prost(message, tag = "7")]
            AvailablePackage(super::SoftwarePackage),
        }
    }
    /// Software package information of the operating system.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SoftwarePackage {
        /// Information about the different types of software packages.
        #[prost(oneof = "software_package::Details", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
        pub details: ::core::option::Option<software_package::Details>,
    }
    /// Nested message and enum types in `SoftwarePackage`.
    pub mod software_package {
        /// Information about the different types of software packages.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Details {
            /// Yum package info.
            /// For details about the yum package manager, see
            /// <https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-yum.>
            #[prost(message, tag = "1")]
            YumPackage(super::VersionedPackage),
            /// Details of an APT package.
            /// For details about the apt package manager, see
            /// <https://wiki.debian.org/Apt.>
            #[prost(message, tag = "2")]
            AptPackage(super::VersionedPackage),
            /// Details of a Zypper package.
            /// For details about the Zypper package manager, see
            /// <https://en.opensuse.org/SDB:Zypper_manual.>
            #[prost(message, tag = "3")]
            ZypperPackage(super::VersionedPackage),
            /// Details of a Googet package.
            ///   For details about the googet package manager, see
            ///   <https://github.com/google/googet.>
            #[prost(message, tag = "4")]
            GoogetPackage(super::VersionedPackage),
            /// Details of a Zypper patch.
            /// For details about the Zypper package manager, see
            /// <https://en.opensuse.org/SDB:Zypper_manual.>
            #[prost(message, tag = "5")]
            ZypperPatch(super::ZypperPatch),
            /// Details of a Windows Update package.
            /// See <https://docs.microsoft.com/en-us/windows/win32/api/_wua/> for
            /// information about Windows Update.
            #[prost(message, tag = "6")]
            WuaPackage(super::WindowsUpdatePackage),
            /// Details of a Windows Quick Fix engineering package.
            /// See
            /// <https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering>
            /// for info in Windows Quick Fix Engineering.
            #[prost(message, tag = "7")]
            QfePackage(super::WindowsQuickFixEngineeringPackage),
            /// Details of a COS package.
            #[prost(message, tag = "8")]
            CosPackage(super::VersionedPackage),
            /// Details of Windows Application.
            #[prost(message, tag = "9")]
            WindowsApplication(super::WindowsApplication),
        }
    }
    /// Information related to the a standard versioned package.  This includes
    /// package info for APT, Yum, Zypper, and Googet package managers.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VersionedPackage {
        /// The name of the package.
        #[prost(string, tag = "4")]
        pub package_name: ::prost::alloc::string::String,
        /// The system architecture this package is intended for.
        #[prost(string, tag = "2")]
        pub architecture: ::prost::alloc::string::String,
        /// The version of the package.
        #[prost(string, tag = "3")]
        pub version: ::prost::alloc::string::String,
    }
    /// Details related to a Zypper Patch.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ZypperPatch {
        /// The name of the patch.
        #[prost(string, tag = "5")]
        pub patch_name: ::prost::alloc::string::String,
        /// The category of the patch.
        #[prost(string, tag = "2")]
        pub category: ::prost::alloc::string::String,
        /// The severity specified for this patch
        #[prost(string, tag = "3")]
        pub severity: ::prost::alloc::string::String,
        /// Any summary information provided about this patch.
        #[prost(string, tag = "4")]
        pub summary: ::prost::alloc::string::String,
    }
    /// Details related to a Windows Update package.
    /// Field data and names are taken from Windows Update API IUpdate Interface:
    /// <https://docs.microsoft.com/en-us/windows/win32/api/_wua/>
    /// Descriptive fields like title, and description are localized based on
    /// the locale of the VM being updated.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsUpdatePackage {
        /// The localized title of the update package.
        #[prost(string, tag = "1")]
        pub title: ::prost::alloc::string::String,
        /// The localized description of the update package.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// The categories that are associated with this update package.
        #[prost(message, repeated, tag = "3")]
        pub categories: ::prost::alloc::vec::Vec<
            windows_update_package::WindowsUpdateCategory,
        >,
        /// A collection of Microsoft Knowledge Base article IDs that are associated
        /// with the update package.
        #[prost(string, repeated, tag = "4")]
        pub kb_article_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// A hyperlink to the language-specific support information for the update.
        #[prost(string, tag = "11")]
        pub support_url: ::prost::alloc::string::String,
        /// A collection of URLs that provide more information about the update
        /// package.
        #[prost(string, repeated, tag = "5")]
        pub more_info_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Gets the identifier of an update package.  Stays the same across
        /// revisions.
        #[prost(string, tag = "6")]
        pub update_id: ::prost::alloc::string::String,
        /// The revision number of this update package.
        #[prost(int32, tag = "7")]
        pub revision_number: i32,
        /// The last published date of the update, in (UTC) date and time.
        #[prost(message, optional, tag = "10")]
        pub last_deployment_change_time: ::core::option::Option<
            ::prost_types::Timestamp,
        >,
    }
    /// Nested message and enum types in `WindowsUpdatePackage`.
    pub mod windows_update_package {
        /// Categories specified by the Windows Update.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct WindowsUpdateCategory {
            /// The identifier of the windows update category.
            #[prost(string, tag = "1")]
            pub id: ::prost::alloc::string::String,
            /// The name of the windows update category.
            #[prost(string, tag = "2")]
            pub name: ::prost::alloc::string::String,
        }
    }
    /// Information related to a Quick Fix Engineering package.
    /// Fields are taken from Windows QuickFixEngineering Interface and match
    /// the source names:
    /// <https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsQuickFixEngineeringPackage {
        /// A short textual description of the QFE update.
        #[prost(string, tag = "1")]
        pub caption: ::prost::alloc::string::String,
        /// A textual description of the QFE update.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// Unique identifier associated with a particular QFE update.
        #[prost(string, tag = "3")]
        pub hot_fix_id: ::prost::alloc::string::String,
        /// Date that the QFE update was installed.  Mapped from installed_on field.
        #[prost(message, optional, tag = "5")]
        pub install_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Contains information about a Windows application that is retrieved from the
    /// Windows Registry. For more information about these fields, see:
    /// <https://docs.microsoft.com/en-us/windows/win32/msi/uninstall-registry-key>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsApplication {
        /// The name of the application or product.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// The version of the product or application in string format.
        #[prost(string, tag = "2")]
        pub display_version: ::prost::alloc::string::String,
        /// The name of the manufacturer for the product or application.
        #[prost(string, tag = "3")]
        pub publisher: ::prost::alloc::string::String,
        /// The last time this product received service. The value of this property
        /// is replaced each time a patch is applied or removed from the product or
        /// the command-line option is used to repair the product.
        #[prost(message, optional, tag = "4")]
        pub install_date: ::core::option::Option<
            super::super::super::super::r#type::Date,
        >,
        /// The internet address for technical support.
        #[prost(string, tag = "5")]
        pub help_link: ::prost::alloc::string::String,
    }
}
/// A request message for getting inventory data for the specified VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInventoryRequest {
    /// Required. API resource name for inventory resource.
    ///
    /// Format:
    /// `projects/{project}/locations/{location}/instances/{instance}/inventory`
    ///
    /// For `{project}`, either `project-number` or `project-id` can be provided.
    /// For `{instance}`, either Compute Engine  `instance-id` or `instance-name`
    /// can be provided.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Inventory view indicating what information should be included in the
    /// inventory resource. If unspecified, the default view is BASIC.
    #[prost(enumeration = "InventoryView", tag = "2")]
    pub view: i32,
}
/// A request message for listing inventory data for all VMs in the specified
/// location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInventoriesRequest {
    /// Required. The parent resource name.
    ///
    /// Format: `projects/{project}/locations/{location}/instances/-`
    ///
    /// For `{project}`, either `project-number` or `project-id` can be provided.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Inventory view indicating what information should be included in the
    /// inventory resource. If unspecified, the default view is BASIC.
    #[prost(enumeration = "InventoryView", tag = "2")]
    pub view: i32,
    /// The maximum number of results to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to
    /// `ListInventories` that indicates where this listing
    /// should continue from.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by a
    /// `Inventory` API resource to be included in the response.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing inventory data for all VMs in a specified
/// location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInventoriesResponse {
    /// List of inventory objects.
    #[prost(message, repeated, tag = "1")]
    pub inventories: ::prost::alloc::vec::Vec<Inventory>,
    /// The pagination token to retrieve the next page of inventory objects.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The view for inventory objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InventoryView {
    /// The default value.
    /// The API defaults to the BASIC view.
    Unspecified = 0,
    /// Returns the basic inventory information that includes `os_info`.
    Basic = 1,
    /// Returns all fields.
    Full = 2,
}
impl InventoryView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InventoryView::Unspecified => "INVENTORY_VIEW_UNSPECIFIED",
            InventoryView::Basic => "BASIC",
            InventoryView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVENTORY_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// An OS policy defines the desired state configuration for a VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicy {
    /// Required. The id of the OS policy with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the assignment.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Policy description.
    /// Length of the description is limited to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Policy mode
    #[prost(enumeration = "os_policy::Mode", tag = "3")]
    pub mode: i32,
    /// Required. List of resource groups for the policy.
    /// For a particular VM, resource groups are evaluated in the order specified
    /// and the first resource group that is applicable is selected and the rest
    /// are ignored.
    ///
    /// If none of the resource groups are applicable for a VM, the VM is
    /// considered to be non-compliant w.r.t this policy. This behavior can be
    /// toggled by the flag `allow_no_resource_group_match`
    #[prost(message, repeated, tag = "4")]
    pub resource_groups: ::prost::alloc::vec::Vec<os_policy::ResourceGroup>,
    /// This flag determines the OS policy compliance status when none of the
    /// resource groups within the policy are applicable for a VM. Set this value
    /// to `true` if the policy needs to be reported as compliant even if the
    /// policy has nothing to validate or enforce.
    #[prost(bool, tag = "5")]
    pub allow_no_resource_group_match: bool,
}
/// Nested message and enum types in `OSPolicy`.
pub mod os_policy {
    /// Filtering criteria to select VMs based on inventory details.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InventoryFilter {
        /// Required. The OS short name
        #[prost(string, tag = "1")]
        pub os_short_name: ::prost::alloc::string::String,
        /// The OS version
        ///
        /// Prefix matches are supported if asterisk(*) is provided as the
        /// last character. For example, to match all versions with a major
        /// version of `7`, specify the following value for this field `7.*`
        ///
        /// An empty string matches all OS versions.
        #[prost(string, tag = "2")]
        pub os_version: ::prost::alloc::string::String,
    }
    /// An OS policy resource is used to define the desired state configuration
    /// and provides a specific functionality like installing/removing packages,
    /// executing a script etc.
    ///
    /// The system ensures that resources are always in their desired state by
    /// taking necessary actions if they have drifted from their desired state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        /// Required. The id of the resource with the following restrictions:
        ///
        /// * Must contain only lowercase letters, numbers, and hyphens.
        /// * Must start with a letter.
        /// * Must be between 1-63 characters.
        /// * Must end with a number or a letter.
        /// * Must be unique within the OS policy.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Resource type.
        #[prost(oneof = "resource::ResourceType", tags = "2, 3, 4, 5")]
        pub resource_type: ::core::option::Option<resource::ResourceType>,
    }
    /// Nested message and enum types in `Resource`.
    pub mod resource {
        /// A remote or local file.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct File {
            /// Defaults to false. When false, files are subject to validations
            /// based on the file type:
            ///
            /// Remote: A checksum must be specified.
            /// Cloud Storage: An object generation number must be specified.
            #[prost(bool, tag = "4")]
            pub allow_insecure: bool,
            /// A specific type of file.
            #[prost(oneof = "file::Type", tags = "1, 2, 3")]
            pub r#type: ::core::option::Option<file::Type>,
        }
        /// Nested message and enum types in `File`.
        pub mod file {
            /// Specifies a file available via some URI.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Remote {
                /// Required. URI from which to fetch the object. It should contain both
                /// the protocol and path following the format `{protocol}://{location}`.
                #[prost(string, tag = "1")]
                pub uri: ::prost::alloc::string::String,
                /// SHA256 checksum of the remote file.
                #[prost(string, tag = "2")]
                pub sha256_checksum: ::prost::alloc::string::String,
            }
            /// Specifies a file available as a Cloud Storage Object.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Gcs {
                /// Required. Bucket of the Cloud Storage object.
                #[prost(string, tag = "1")]
                pub bucket: ::prost::alloc::string::String,
                /// Required. Name of the Cloud Storage object.
                #[prost(string, tag = "2")]
                pub object: ::prost::alloc::string::String,
                /// Generation number of the Cloud Storage object.
                #[prost(int64, tag = "3")]
                pub generation: i64,
            }
            /// A specific type of file.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Type {
                /// A generic remote file.
                #[prost(message, tag = "1")]
                Remote(Remote),
                /// A Cloud Storage object.
                #[prost(message, tag = "2")]
                Gcs(Gcs),
                /// A local path within the VM to use.
                #[prost(string, tag = "3")]
                LocalPath(::prost::alloc::string::String),
            }
        }
        /// A resource that manages a system package.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PackageResource {
            /// Required. The desired state the agent should maintain for this package.
            #[prost(enumeration = "package_resource::DesiredState", tag = "1")]
            pub desired_state: i32,
            /// A system package.
            #[prost(
                oneof = "package_resource::SystemPackage",
                tags = "2, 3, 4, 5, 6, 7, 8"
            )]
            pub system_package: ::core::option::Option<package_resource::SystemPackage>,
        }
        /// Nested message and enum types in `PackageResource`.
        pub mod package_resource {
            /// A deb package file. dpkg packages only support INSTALLED state.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Deb {
                /// Required. A deb package.
                #[prost(message, optional, tag = "1")]
                pub source: ::core::option::Option<super::File>,
                /// Whether dependencies should also be installed.
                /// - install when false: `dpkg -i package`
                /// - install when true: `apt-get update && apt-get -y install
                /// package.deb`
                #[prost(bool, tag = "2")]
                pub pull_deps: bool,
            }
            /// A package managed by APT.
            /// - install: `apt-get update && apt-get -y install \[name\]`
            /// - remove: `apt-get -y remove \[name\]`
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Apt {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// An RPM package file. RPM packages only support INSTALLED state.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Rpm {
                /// Required. An rpm package.
                #[prost(message, optional, tag = "1")]
                pub source: ::core::option::Option<super::File>,
                /// Whether dependencies should also be installed.
                /// - install when false: `rpm --upgrade --replacepkgs package.rpm`
                /// - install when true: `yum -y install package.rpm` or
                /// `zypper -y install package.rpm`
                #[prost(bool, tag = "2")]
                pub pull_deps: bool,
            }
            /// A package managed by YUM.
            /// - install: `yum -y install package`
            /// - remove: `yum -y remove package`
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Yum {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// A package managed by Zypper.
            /// - install: `zypper -y install package`
            /// - remove: `zypper -y rm package`
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Zypper {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// A package managed by GooGet.
            /// - install: `googet -noconfirm install package`
            /// - remove: `googet -noconfirm remove package`
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GooGet {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// An MSI package. MSI packages only support INSTALLED state.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Msi {
                /// Required. The MSI package.
                #[prost(message, optional, tag = "1")]
                pub source: ::core::option::Option<super::File>,
                /// Additional properties to use during installation.
                /// This should be in the format of Property=Setting.
                /// Appended to the defaults of `ACTION=INSTALL
                /// REBOOT=ReallySuppress`.
                #[prost(string, repeated, tag = "2")]
                pub properties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// The desired state that the OS Config agent maintains on the VM.
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
            pub enum DesiredState {
                /// Unspecified is invalid.
                Unspecified = 0,
                /// Ensure that the package is installed.
                Installed = 1,
                /// The agent ensures that the package is not installed and
                /// uninstalls it if detected.
                Removed = 2,
            }
            impl DesiredState {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        DesiredState::Unspecified => "DESIRED_STATE_UNSPECIFIED",
                        DesiredState::Installed => "INSTALLED",
                        DesiredState::Removed => "REMOVED",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "DESIRED_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                        "INSTALLED" => Some(Self::Installed),
                        "REMOVED" => Some(Self::Removed),
                        _ => None,
                    }
                }
            }
            /// A system package.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum SystemPackage {
                /// A package managed by Apt.
                #[prost(message, tag = "2")]
                Apt(Apt),
                /// A deb package file.
                #[prost(message, tag = "3")]
                Deb(Deb),
                /// A package managed by YUM.
                #[prost(message, tag = "4")]
                Yum(Yum),
                /// A package managed by Zypper.
                #[prost(message, tag = "5")]
                Zypper(Zypper),
                /// An rpm package file.
                #[prost(message, tag = "6")]
                Rpm(Rpm),
                /// A package managed by GooGet.
                #[prost(message, tag = "7")]
                Googet(GooGet),
                /// An MSI package.
                #[prost(message, tag = "8")]
                Msi(Msi),
            }
        }
        /// A resource that manages a package repository.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RepositoryResource {
            /// A specific type of repository.
            #[prost(oneof = "repository_resource::Repository", tags = "1, 2, 3, 4")]
            pub repository: ::core::option::Option<repository_resource::Repository>,
        }
        /// Nested message and enum types in `RepositoryResource`.
        pub mod repository_resource {
            /// Represents a single apt package repository. These will be added to
            /// a repo file that will be managed at
            /// `/etc/apt/sources.list.d/google_osconfig.list`.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct AptRepository {
                /// Required. Type of archive files in this repository.
                #[prost(enumeration = "apt_repository::ArchiveType", tag = "1")]
                pub archive_type: i32,
                /// Required. URI for this repository.
                #[prost(string, tag = "2")]
                pub uri: ::prost::alloc::string::String,
                /// Required. Distribution of this repository.
                #[prost(string, tag = "3")]
                pub distribution: ::prost::alloc::string::String,
                /// Required. List of components for this repository. Must contain at
                /// least one item.
                #[prost(string, repeated, tag = "4")]
                pub components: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// URI of the key file for this repository. The agent maintains a
                /// keyring at `/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg`.
                #[prost(string, tag = "5")]
                pub gpg_key: ::prost::alloc::string::String,
            }
            /// Nested message and enum types in `AptRepository`.
            pub mod apt_repository {
                /// Type of archive.
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
                pub enum ArchiveType {
                    /// Unspecified is invalid.
                    Unspecified = 0,
                    /// Deb indicates that the archive contains binary files.
                    Deb = 1,
                    /// Deb-src indicates that the archive contains source files.
                    DebSrc = 2,
                }
                impl ArchiveType {
                    /// String value of the enum field names used in the ProtoBuf definition.
                    ///
                    /// The values are not transformed in any way and thus are considered stable
                    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                    pub fn as_str_name(&self) -> &'static str {
                        match self {
                            ArchiveType::Unspecified => "ARCHIVE_TYPE_UNSPECIFIED",
                            ArchiveType::Deb => "DEB",
                            ArchiveType::DebSrc => "DEB_SRC",
                        }
                    }
                    /// Creates an enum from field names used in the ProtoBuf definition.
                    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                        match value {
                            "ARCHIVE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                            "DEB" => Some(Self::Deb),
                            "DEB_SRC" => Some(Self::DebSrc),
                            _ => None,
                        }
                    }
                }
            }
            /// Represents a single yum package repository. These are added to a
            /// repo file that is managed at
            /// `/etc/yum.repos.d/google_osconfig.repo`.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct YumRepository {
                /// Required. A one word, unique name for this repository. This is  the
                /// `repo id` in the yum config file and also the `display_name` if
                /// `display_name` is omitted. This id is also used as the unique
                /// identifier when checking for resource conflicts.
                #[prost(string, tag = "1")]
                pub id: ::prost::alloc::string::String,
                /// The display name of the repository.
                #[prost(string, tag = "2")]
                pub display_name: ::prost::alloc::string::String,
                /// Required. The location of the repository directory.
                #[prost(string, tag = "3")]
                pub base_url: ::prost::alloc::string::String,
                /// URIs of GPG keys.
                #[prost(string, repeated, tag = "4")]
                pub gpg_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// Represents a single zypper package repository. These are added to a
            /// repo file that is managed at
            /// `/etc/zypp/repos.d/google_osconfig.repo`.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ZypperRepository {
                /// Required. A one word, unique name for this repository. This is the
                /// `repo id` in the zypper config file and also the `display_name` if
                /// `display_name` is omitted. This id is also used as the unique
                /// identifier when checking for GuestPolicy conflicts.
                #[prost(string, tag = "1")]
                pub id: ::prost::alloc::string::String,
                /// The display name of the repository.
                #[prost(string, tag = "2")]
                pub display_name: ::prost::alloc::string::String,
                /// Required. The location of the repository directory.
                #[prost(string, tag = "3")]
                pub base_url: ::prost::alloc::string::String,
                /// URIs of GPG keys.
                #[prost(string, repeated, tag = "4")]
                pub gpg_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// Represents a Goo package repository. These are added to a repo file
            /// that is managed at
            /// `C:/ProgramData/GooGet/repos/google_osconfig.repo`.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GooRepository {
                /// Required. The name of the repository.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
                /// Required. The url of the repository.
                #[prost(string, tag = "2")]
                pub url: ::prost::alloc::string::String,
            }
            /// A specific type of repository.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Repository {
                /// An Apt Repository.
                #[prost(message, tag = "1")]
                Apt(AptRepository),
                /// A Yum Repository.
                #[prost(message, tag = "2")]
                Yum(YumRepository),
                /// A Zypper Repository.
                #[prost(message, tag = "3")]
                Zypper(ZypperRepository),
                /// A Goo Repository.
                #[prost(message, tag = "4")]
                Goo(GooRepository),
            }
        }
        /// A resource that allows executing scripts on the VM.
        ///
        /// The `ExecResource` has 2 stages: `validate` and `enforce` and both stages
        /// accept a script as an argument to execute.
        ///
        /// When the `ExecResource` is applied by the agent, it first executes the
        /// script in the `validate` stage. The `validate` stage can signal that the
        /// `ExecResource` is already in the desired state by returning an exit code
        /// of `100`. If the `ExecResource` is not in the desired state, it should
        /// return an exit code of `101`. Any other exit code returned by this stage
        /// is considered an error.
        ///
        /// If the `ExecResource` is not in the desired state based on the exit code
        /// from the `validate` stage, the agent proceeds to execute the script from
        /// the `enforce` stage. If the `ExecResource` is already in the desired
        /// state, the `enforce` stage will not be run.
        /// Similar to `validate` stage, the `enforce` stage should return an exit
        /// code of `100` to indicate that the resource in now in its desired state.
        /// Any other exit code is considered an error.
        ///
        /// NOTE: An exit code of `100` was chosen over `0` (and `101` vs `1`) to
        /// have an explicit indicator of `in desired state`, `not in desired state`
        /// and errors. Because, for example, Powershell will always return an exit
        /// code of `0` unless an `exit` statement is provided in the script. So, for
        /// reasons of consistency and being explicit, exit codes `100` and `101`
        /// were chosen.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExecResource {
            /// Required. What to run to validate this resource is in the desired
            /// state. An exit code of 100 indicates "in desired state", and exit code
            /// of 101 indicates "not in desired state". Any other exit code indicates
            /// a failure running validate.
            #[prost(message, optional, tag = "1")]
            pub validate: ::core::option::Option<exec_resource::Exec>,
            /// What to run to bring this resource into the desired state.
            /// An exit code of 100 indicates "success", any other exit code indicates
            /// a failure running enforce.
            #[prost(message, optional, tag = "2")]
            pub enforce: ::core::option::Option<exec_resource::Exec>,
        }
        /// Nested message and enum types in `ExecResource`.
        pub mod exec_resource {
            /// A file or script to execute.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Exec {
                /// Optional arguments to pass to the source during execution.
                #[prost(string, repeated, tag = "3")]
                pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// Required. The script interpreter to use.
                #[prost(enumeration = "exec::Interpreter", tag = "4")]
                pub interpreter: i32,
                /// Only recorded for enforce Exec.
                /// Path to an output file (that is created by this Exec) whose
                /// content will be recorded in OSPolicyResourceCompliance after a
                /// successful run. Absence or failure to read this file will result in
                /// this ExecResource being non-compliant. Output file size is limited to
                /// 100K bytes.
                #[prost(string, tag = "5")]
                pub output_file_path: ::prost::alloc::string::String,
                /// What to execute.
                #[prost(oneof = "exec::Source", tags = "1, 2")]
                pub source: ::core::option::Option<exec::Source>,
            }
            /// Nested message and enum types in `Exec`.
            pub mod exec {
                /// The interpreter to use.
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
                pub enum Interpreter {
                    /// Invalid value, the request will return validation error.
                    Unspecified = 0,
                    /// If an interpreter is not specified, the
                    /// source is executed directly. This execution, without an
                    /// interpreter, only succeeds for executables and scripts that have <a
                    /// href="<https://en.wikipedia.org/wiki/Shebang_(Unix>)"
                    /// class="external">shebang lines</a>.
                    None = 1,
                    /// Indicates that the script runs with `/bin/sh` on Linux and
                    /// `cmd.exe` on Windows.
                    Shell = 2,
                    /// Indicates that the script runs with PowerShell.
                    Powershell = 3,
                }
                impl Interpreter {
                    /// String value of the enum field names used in the ProtoBuf definition.
                    ///
                    /// The values are not transformed in any way and thus are considered stable
                    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                    pub fn as_str_name(&self) -> &'static str {
                        match self {
                            Interpreter::Unspecified => "INTERPRETER_UNSPECIFIED",
                            Interpreter::None => "NONE",
                            Interpreter::Shell => "SHELL",
                            Interpreter::Powershell => "POWERSHELL",
                        }
                    }
                    /// Creates an enum from field names used in the ProtoBuf definition.
                    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                        match value {
                            "INTERPRETER_UNSPECIFIED" => Some(Self::Unspecified),
                            "NONE" => Some(Self::None),
                            "SHELL" => Some(Self::Shell),
                            "POWERSHELL" => Some(Self::Powershell),
                            _ => None,
                        }
                    }
                }
                /// What to execute.
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Source {
                    /// A remote or local file.
                    #[prost(message, tag = "1")]
                    File(super::super::File),
                    /// An inline script.
                    /// The size of the script is limited to 1024 characters.
                    #[prost(string, tag = "2")]
                    Script(::prost::alloc::string::String),
                }
            }
        }
        /// A resource that manages the state of a file.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FileResource {
            /// Required. The absolute path of the file within the VM.
            #[prost(string, tag = "3")]
            pub path: ::prost::alloc::string::String,
            /// Required. Desired state of the file.
            #[prost(enumeration = "file_resource::DesiredState", tag = "4")]
            pub state: i32,
            /// Consists of three octal digits which represent, in
            /// order, the permissions of the owner, group, and other users for the
            /// file (similarly to the numeric mode used in the linux chmod
            /// utility). Each digit represents a three bit number with the 4 bit
            /// corresponding to the read permissions, the 2 bit corresponds to the
            /// write bit, and the one bit corresponds to the execute permission.
            /// Default behavior is 755.
            ///
            /// Below are some examples of permissions and their associated values:
            /// read, write, and execute: 7
            /// read and execute: 5
            /// read and write: 6
            /// read only: 4
            #[prost(string, tag = "5")]
            pub permissions: ::prost::alloc::string::String,
            /// The source for the contents of the file.
            #[prost(oneof = "file_resource::Source", tags = "1, 2")]
            pub source: ::core::option::Option<file_resource::Source>,
        }
        /// Nested message and enum types in `FileResource`.
        pub mod file_resource {
            /// Desired state of the file.
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
            pub enum DesiredState {
                /// Unspecified is invalid.
                Unspecified = 0,
                /// Ensure file at path is present.
                Present = 1,
                /// Ensure file at path is absent.
                Absent = 2,
                /// Ensure the contents of the file at path matches. If the file does
                /// not exist it will be created.
                ContentsMatch = 3,
            }
            impl DesiredState {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        DesiredState::Unspecified => "DESIRED_STATE_UNSPECIFIED",
                        DesiredState::Present => "PRESENT",
                        DesiredState::Absent => "ABSENT",
                        DesiredState::ContentsMatch => "CONTENTS_MATCH",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "DESIRED_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                        "PRESENT" => Some(Self::Present),
                        "ABSENT" => Some(Self::Absent),
                        "CONTENTS_MATCH" => Some(Self::ContentsMatch),
                        _ => None,
                    }
                }
            }
            /// The source for the contents of the file.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Source {
                /// A remote or local source.
                #[prost(message, tag = "1")]
                File(super::File),
                /// A a file with this content.
                /// The size of the content is limited to 1024 characters.
                #[prost(string, tag = "2")]
                Content(::prost::alloc::string::String),
            }
        }
        /// Resource type.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ResourceType {
            /// Package resource
            #[prost(message, tag = "2")]
            Pkg(PackageResource),
            /// Package repository resource
            #[prost(message, tag = "3")]
            Repository(RepositoryResource),
            /// Exec resource
            #[prost(message, tag = "4")]
            Exec(ExecResource),
            /// File resource
            #[prost(message, tag = "5")]
            File(FileResource),
        }
    }
    /// Resource groups provide a mechanism to group OS policy resources.
    ///
    /// Resource groups enable OS policy authors to create a single OS policy
    /// to be applied to VMs running different operating Systems.
    ///
    /// When the OS policy is applied to a target VM, the appropriate resource
    /// group within the OS policy is selected based on the `OSFilter` specified
    /// within the resource group.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceGroup {
        /// List of inventory filters for the resource group.
        ///
        /// The resources in this resource group are applied to the target VM if it
        /// satisfies at least one of the following inventory filters.
        ///
        /// For example, to apply this resource group to VMs running either `RHEL` or
        /// `CentOS` operating systems, specify 2 items for the list with following
        /// values:
        /// inventory_filters\[0\].os_short_name='rhel' and
        /// inventory_filters\[1\].os_short_name='centos'
        ///
        /// If the list is empty, this resource group will be applied to the target
        /// VM unconditionally.
        #[prost(message, repeated, tag = "1")]
        pub inventory_filters: ::prost::alloc::vec::Vec<InventoryFilter>,
        /// Required. List of resources configured for this resource group.
        /// The resources are executed in the exact order specified here.
        #[prost(message, repeated, tag = "2")]
        pub resources: ::prost::alloc::vec::Vec<Resource>,
    }
    /// Policy mode
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
    pub enum Mode {
        /// Invalid mode
        Unspecified = 0,
        /// This mode checks if the configuration resources in the policy are in
        /// their desired state. No actions are performed if they are not in the
        /// desired state. This mode is used for reporting purposes.
        Validation = 1,
        /// This mode checks if the configuration resources in the policy are in
        /// their desired state, and if not, enforces the desired state.
        Enforcement = 2,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Unspecified => "MODE_UNSPECIFIED",
                Mode::Validation => "VALIDATION",
                Mode::Enforcement => "ENFORCEMENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "VALIDATION" => Some(Self::Validation),
                "ENFORCEMENT" => Some(Self::Enforcement),
                _ => None,
            }
        }
    }
}
/// Get a report of the OS policy assignment for a VM instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOsPolicyAssignmentReportRequest {
    /// Required. API resource name for OS policy assignment report.
    ///
    /// Format:
    /// `/projects/{project}/locations/{location}/instances/{instance}/osPolicyAssignments/{assignment}/report`
    ///
    /// For `{project}`, either `project-number` or `project-id` can be provided.
    /// For `{instance_id}`, either Compute Engine `instance-id` or `instance-name`
    /// can be provided.
    /// For `{assignment_id}`, the OSPolicyAssignment id must be provided.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// List the OS policy assignment reports for VM instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsPolicyAssignmentReportsRequest {
    /// Required. The parent resource name.
    ///
    /// Format:
    /// `projects/{project}/locations/{location}/instances/{instance}/osPolicyAssignments/{assignment}/reports`
    ///
    /// For `{project}`, either `project-number` or `project-id` can be provided.
    /// For `{instance}`, either `instance-name`, `instance-id`, or `-` can be
    /// provided. If '-' is provided, the response will include
    /// OSPolicyAssignmentReports for all instances in the project/location.
    /// For `{assignment}`, either `assignment-id` or `-` can be provided. If '-'
    /// is provided, the response will include OSPolicyAssignmentReports for all
    /// OSPolicyAssignments in the project/location.
    /// Either {instance} or {assignment} must be `-`.
    ///
    /// For example:
    /// `projects/{project}/locations/{location}/instances/{instance}/osPolicyAssignments/-/reports`
    ///   returns all reports for the instance
    /// `projects/{project}/locations/{location}/instances/-/osPolicyAssignments/{assignment-id}/reports`
    ///   returns all the reports for the given assignment across all instances.
    /// `projects/{project}/locations/{location}/instances/-/osPolicyAssignments/-/reports`
    ///   returns all the reports for all assignments across all instances.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If provided, this field specifies the criteria that must be met by the
    /// `OSPolicyAssignmentReport` API resource that is included in the response.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// A pagination token returned from a previous call to the
    /// `ListOSPolicyAssignmentReports` method that indicates where this listing
    /// should continue from.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response message for listing OS Policy assignment reports including the
/// page of results and page token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsPolicyAssignmentReportsResponse {
    /// List of OS policy assignment reports.
    #[prost(message, repeated, tag = "1")]
    pub os_policy_assignment_reports: ::prost::alloc::vec::Vec<OsPolicyAssignmentReport>,
    /// The pagination token to retrieve the next page of OS policy assignment
    /// report objects.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A report of the OS policy assignment status for a given instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicyAssignmentReport {
    /// The `OSPolicyAssignmentReport` API resource name.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/instances/{instance_id}/osPolicyAssignments/{os_policy_assignment_id}/report`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The Compute Engine VM instance name.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Reference to the `OSPolicyAssignment` API resource that the `OSPolicy`
    /// belongs to.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}`
    #[prost(string, tag = "3")]
    pub os_policy_assignment: ::prost::alloc::string::String,
    /// Compliance data for each `OSPolicy` that is applied to the VM.
    #[prost(message, repeated, tag = "4")]
    pub os_policy_compliances: ::prost::alloc::vec::Vec<
        os_policy_assignment_report::OsPolicyCompliance,
    >,
    /// Timestamp for when the report was last generated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Unique identifier of the last attempted run to apply the OS policies
    /// associated with this assignment on the VM.
    ///
    /// This ID is logged by the OS Config agent while applying the OS
    /// policies associated with this assignment on the VM.
    /// NOTE: If the service is unable to successfully connect to the agent for
    /// this run, then this id will not be available in the agent logs.
    #[prost(string, tag = "6")]
    pub last_run_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `OSPolicyAssignmentReport`.
pub mod os_policy_assignment_report {
    /// Compliance data for an OS policy
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsPolicyCompliance {
        /// The OS policy id
        #[prost(string, tag = "1")]
        pub os_policy_id: ::prost::alloc::string::String,
        /// The compliance state of the OS policy.
        #[prost(enumeration = "os_policy_compliance::ComplianceState", tag = "2")]
        pub compliance_state: i32,
        /// The reason for the OS policy to be in an unknown compliance state.
        /// This field is always populated when `compliance_state` is `UNKNOWN`.
        ///
        /// If populated, the field can contain one of the following values:
        ///
        /// * `vm-not-running`: The VM was not running.
        /// * `os-policies-not-supported-by-agent`: The version of the OS Config
        /// agent running on the VM does not support running OS policies.
        /// * `no-agent-detected`: The OS Config agent is not detected for the VM.
        /// * `resource-execution-errors`: The OS Config agent encountered errors
        /// while executing one or more resources in the policy. See
        /// `os_policy_resource_compliances` for details.
        /// * `task-timeout`: The task sent to the agent to apply the policy timed
        /// out.
        /// * `unexpected-agent-state`: The OS Config agent did not report the final
        /// status of the task that attempted to apply the policy. Instead, the agent
        /// unexpectedly started working on a different task. This mostly happens
        /// when the agent or VM unexpectedly restarts while applying OS policies.
        /// * `internal-service-errors`: Internal service errors were encountered
        /// while attempting to apply the policy.
        #[prost(string, tag = "3")]
        pub compliance_state_reason: ::prost::alloc::string::String,
        /// Compliance data for each resource within the policy that is applied to
        /// the VM.
        #[prost(message, repeated, tag = "4")]
        pub os_policy_resource_compliances: ::prost::alloc::vec::Vec<
            os_policy_compliance::OsPolicyResourceCompliance,
        >,
    }
    /// Nested message and enum types in `OSPolicyCompliance`.
    pub mod os_policy_compliance {
        /// Compliance data for an OS policy resource.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct OsPolicyResourceCompliance {
            /// The ID of the OS policy resource.
            #[prost(string, tag = "1")]
            pub os_policy_resource_id: ::prost::alloc::string::String,
            /// Ordered list of configuration completed by the agent for the OS policy
            /// resource.
            #[prost(message, repeated, tag = "2")]
            pub config_steps: ::prost::alloc::vec::Vec<
                os_policy_resource_compliance::OsPolicyResourceConfigStep,
            >,
            /// The compliance state of the resource.
            #[prost(
                enumeration = "os_policy_resource_compliance::ComplianceState",
                tag = "3"
            )]
            pub compliance_state: i32,
            /// A reason for the resource to be in the given compliance state.
            /// This field is always populated when `compliance_state` is `UNKNOWN`.
            ///
            /// The following values are supported when `compliance_state == UNKNOWN`
            ///
            /// * `execution-errors`: Errors were encountered by the agent while
            /// executing the resource and the compliance state couldn't be
            /// determined.
            /// * `execution-skipped-by-agent`: Resource execution was skipped by the
            /// agent because errors were encountered while executing prior resources
            /// in the OS policy.
            /// * `os-policy-execution-attempt-failed`: The execution of the OS policy
            /// containing this resource failed and the compliance state couldn't be
            /// determined.
            #[prost(string, tag = "4")]
            pub compliance_state_reason: ::prost::alloc::string::String,
            /// Resource specific output.
            #[prost(oneof = "os_policy_resource_compliance::Output", tags = "5")]
            pub output: ::core::option::Option<os_policy_resource_compliance::Output>,
        }
        /// Nested message and enum types in `OSPolicyResourceCompliance`.
        pub mod os_policy_resource_compliance {
            /// Step performed by the OS Config agent for configuring an
            /// `OSPolicy` resource to its desired state.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct OsPolicyResourceConfigStep {
                /// Configuration step type.
                #[prost(enumeration = "os_policy_resource_config_step::Type", tag = "1")]
                pub r#type: i32,
                /// An error message recorded during the execution of this step.
                /// Only populated if errors were encountered during this step execution.
                #[prost(string, tag = "2")]
                pub error_message: ::prost::alloc::string::String,
            }
            /// Nested message and enum types in `OSPolicyResourceConfigStep`.
            pub mod os_policy_resource_config_step {
                /// Supported configuration step types
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
                pub enum Type {
                    /// Default value. This value is unused.
                    Unspecified = 0,
                    /// Checks for resource conflicts such as schema errors.
                    Validation = 1,
                    /// Checks the current status of the desired state for a resource.
                    DesiredStateCheck = 2,
                    /// Enforces the desired state for a resource that is not in desired
                    /// state.
                    DesiredStateEnforcement = 3,
                    /// Re-checks the status of the desired state. This check is done
                    /// for a resource after the enforcement of all OS policies.
                    ///
                    /// This step is used to determine the final desired state status for
                    /// the resource. It accounts for any resources that might have drifted
                    /// from their desired state due to side effects from executing other
                    /// resources.
                    DesiredStateCheckPostEnforcement = 4,
                }
                impl Type {
                    /// String value of the enum field names used in the ProtoBuf definition.
                    ///
                    /// The values are not transformed in any way and thus are considered stable
                    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                    pub fn as_str_name(&self) -> &'static str {
                        match self {
                            Type::Unspecified => "TYPE_UNSPECIFIED",
                            Type::Validation => "VALIDATION",
                            Type::DesiredStateCheck => "DESIRED_STATE_CHECK",
                            Type::DesiredStateEnforcement => "DESIRED_STATE_ENFORCEMENT",
                            Type::DesiredStateCheckPostEnforcement => {
                                "DESIRED_STATE_CHECK_POST_ENFORCEMENT"
                            }
                        }
                    }
                    /// Creates an enum from field names used in the ProtoBuf definition.
                    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                        match value {
                            "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                            "VALIDATION" => Some(Self::Validation),
                            "DESIRED_STATE_CHECK" => Some(Self::DesiredStateCheck),
                            "DESIRED_STATE_ENFORCEMENT" => {
                                Some(Self::DesiredStateEnforcement)
                            }
                            "DESIRED_STATE_CHECK_POST_ENFORCEMENT" => {
                                Some(Self::DesiredStateCheckPostEnforcement)
                            }
                            _ => None,
                        }
                    }
                }
            }
            /// ExecResource specific output.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ExecResourceOutput {
                /// Output from enforcement phase output file (if run).
                /// Output size is limited to 100K bytes.
                #[prost(bytes = "vec", tag = "2")]
                pub enforcement_output: ::prost::alloc::vec::Vec<u8>,
            }
            /// Possible compliance states for a resource.
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
            pub enum ComplianceState {
                /// The resource is in an unknown compliance state.
                ///
                /// To get more details about why the policy is in this state, review
                /// the output of the `compliance_state_reason` field.
                Unknown = 0,
                /// Resource is compliant.
                Compliant = 1,
                /// Resource is non-compliant.
                NonCompliant = 2,
            }
            impl ComplianceState {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        ComplianceState::Unknown => "UNKNOWN",
                        ComplianceState::Compliant => "COMPLIANT",
                        ComplianceState::NonCompliant => "NON_COMPLIANT",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "UNKNOWN" => Some(Self::Unknown),
                        "COMPLIANT" => Some(Self::Compliant),
                        "NON_COMPLIANT" => Some(Self::NonCompliant),
                        _ => None,
                    }
                }
            }
            /// Resource specific output.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Output {
                /// ExecResource specific output.
                #[prost(message, tag = "5")]
                ExecResourceOutput(ExecResourceOutput),
            }
        }
        /// Possible compliance states for an os policy.
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
        pub enum ComplianceState {
            /// The policy is in an unknown compliance state.
            ///
            /// Refer to the field `compliance_state_reason` to learn the exact reason
            /// for the policy to be in this compliance state.
            Unknown = 0,
            /// Policy is compliant.
            ///
            /// The policy is compliant if all the underlying resources are also
            /// compliant.
            Compliant = 1,
            /// Policy is non-compliant.
            ///
            /// The policy is non-compliant if one or more underlying resources are
            /// non-compliant.
            NonCompliant = 2,
        }
        impl ComplianceState {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ComplianceState::Unknown => "UNKNOWN",
                    ComplianceState::Compliant => "COMPLIANT",
                    ComplianceState::NonCompliant => "NON_COMPLIANT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "COMPLIANT" => Some(Self::Compliant),
                    "NON_COMPLIANT" => Some(Self::NonCompliant),
                    _ => None,
                }
            }
        }
    }
}
/// Message encapsulating a value that can be either absolute ("fixed") or
/// relative ("percent") to a value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedOrPercent {
    /// Type of the value.
    #[prost(oneof = "fixed_or_percent::Mode", tags = "1, 2")]
    pub mode: ::core::option::Option<fixed_or_percent::Mode>,
}
/// Nested message and enum types in `FixedOrPercent`.
pub mod fixed_or_percent {
    /// Type of the value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// Specifies a fixed value.
        #[prost(int32, tag = "1")]
        Fixed(i32),
        /// Specifies the relative value defined as a percentage, which will be
        /// multiplied by a reference value.
        #[prost(int32, tag = "2")]
        Percent(i32),
    }
}
/// OS policy assignment is an API resource that is used to
/// apply a set of OS policies to a dynamically targeted group of Compute Engine
/// VM instances.
///
/// An OS policy is used to define the desired state configuration for a
/// Compute Engine VM instance through a set of configuration resources that
/// provide capabilities such as installing or removing software packages, or
/// executing a script.
///
/// For more information, see [OS policy and OS policy
/// assignment](<https://cloud.google.com/compute/docs/os-configuration-management/working-with-os-policies>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicyAssignment {
    /// Resource name.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}`
    ///
    /// This field is ignored when you create an OS policy assignment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// OS policy assignment description.
    /// Length of the description is limited to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. List of OS policies to be applied to the VMs.
    #[prost(message, repeated, tag = "3")]
    pub os_policies: ::prost::alloc::vec::Vec<OsPolicy>,
    /// Required. Filter to select VMs.
    #[prost(message, optional, tag = "4")]
    pub instance_filter: ::core::option::Option<os_policy_assignment::InstanceFilter>,
    /// Required. Rollout to deploy the OS policy assignment.
    /// A rollout is triggered in the following situations:
    /// 1) OSPolicyAssignment is created.
    /// 2) OSPolicyAssignment is updated and the update contains changes to one of
    /// the following fields:
    ///     - instance_filter
    ///     - os_policies
    /// 3) OSPolicyAssignment is deleted.
    #[prost(message, optional, tag = "5")]
    pub rollout: ::core::option::Option<os_policy_assignment::Rollout>,
    /// Output only. The assignment revision ID
    /// A new revision is committed whenever a rollout is triggered for a OS policy
    /// assignment
    #[prost(string, tag = "6")]
    pub revision_id: ::prost::alloc::string::String,
    /// Output only. The timestamp that the revision was created.
    #[prost(message, optional, tag = "7")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The etag for this OS policy assignment.
    /// If this is provided on update, it must match the server's etag.
    #[prost(string, tag = "8")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. OS policy assignment rollout state
    #[prost(enumeration = "os_policy_assignment::RolloutState", tag = "9")]
    pub rollout_state: i32,
    /// Output only. Indicates that this revision has been successfully rolled out
    /// in this zone and new VMs will be assigned OS policies from this revision.
    ///
    /// For a given OS policy assignment, there is only one revision with a value
    /// of `true` for this field.
    #[prost(bool, tag = "10")]
    pub baseline: bool,
    /// Output only. Indicates that this revision deletes the OS policy assignment.
    #[prost(bool, tag = "11")]
    pub deleted: bool,
    /// Output only. Indicates that reconciliation is in progress for the revision.
    /// This value is `true` when the `rollout_state` is one of:
    /// * IN_PROGRESS
    /// * CANCELLING
    #[prost(bool, tag = "12")]
    pub reconciling: bool,
    /// Output only. Server generated unique id for the OS policy assignment
    /// resource.
    #[prost(string, tag = "13")]
    pub uid: ::prost::alloc::string::String,
}
/// Nested message and enum types in `OSPolicyAssignment`.
pub mod os_policy_assignment {
    /// Message representing label set.
    /// * A label is a key value pair set for a VM.
    /// * A LabelSet is a set of labels.
    /// * Labels within a LabelSet are ANDed. In other words, a LabelSet is
    ///    applicable for a VM only if it matches all the labels in the
    ///    LabelSet.
    /// * Example: A LabelSet with 2 labels: `env=prod` and `type=webserver` will
    ///             only be applicable for those VMs with both labels
    ///             present.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LabelSet {
        /// Labels are identified by key/value pairs in this map.
        /// A VM should contain all the key/value pairs specified in this
        /// map to be selected.
        #[prost(map = "string, string", tag = "1")]
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
    /// Filters to select target VMs for an assignment.
    ///
    /// If more than one filter criteria is specified below, a VM will be selected
    /// if and only if it satisfies all of them.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceFilter {
        /// Target all VMs in the project. If true, no other criteria is
        /// permitted.
        #[prost(bool, tag = "1")]
        pub all: bool,
        /// List of label sets used for VM inclusion.
        ///
        /// If the list has more than one `LabelSet`, the VM is included if any
        /// of the label sets are applicable for the VM.
        #[prost(message, repeated, tag = "2")]
        pub inclusion_labels: ::prost::alloc::vec::Vec<LabelSet>,
        /// List of label sets used for VM exclusion.
        ///
        /// If the list has more than one label set, the VM is excluded if any
        /// of the label sets are applicable for the VM.
        #[prost(message, repeated, tag = "3")]
        pub exclusion_labels: ::prost::alloc::vec::Vec<LabelSet>,
        /// List of inventories to select VMs.
        ///
        /// A VM is selected if its inventory data matches at least one of the
        /// following inventories.
        #[prost(message, repeated, tag = "4")]
        pub inventories: ::prost::alloc::vec::Vec<instance_filter::Inventory>,
    }
    /// Nested message and enum types in `InstanceFilter`.
    pub mod instance_filter {
        /// VM inventory details.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Inventory {
            /// Required. The OS short name
            #[prost(string, tag = "1")]
            pub os_short_name: ::prost::alloc::string::String,
            /// The OS version
            ///
            /// Prefix matches are supported if asterisk(*) is provided as the
            /// last character. For example, to match all versions with a major
            /// version of `7`, specify the following value for this field `7.*`
            ///
            /// An empty string matches all OS versions.
            #[prost(string, tag = "2")]
            pub os_version: ::prost::alloc::string::String,
        }
    }
    /// Message to configure the rollout at the zonal level for the OS policy
    /// assignment.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rollout {
        /// Required. The maximum number (or percentage) of VMs per zone to disrupt
        /// at any given moment.
        #[prost(message, optional, tag = "1")]
        pub disruption_budget: ::core::option::Option<super::FixedOrPercent>,
        /// Required. This determines the minimum duration of time to wait after the
        /// configuration changes are applied through the current rollout. A
        /// VM continues to count towards the `disruption_budget` at least
        /// until this duration of time has passed after configuration changes are
        /// applied.
        #[prost(message, optional, tag = "2")]
        pub min_wait_duration: ::core::option::Option<::prost_types::Duration>,
    }
    /// OS policy assignment rollout state
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
    pub enum RolloutState {
        /// Invalid value
        Unspecified = 0,
        /// The rollout is in progress.
        InProgress = 1,
        /// The rollout is being cancelled.
        Cancelling = 2,
        /// The rollout is cancelled.
        Cancelled = 3,
        /// The rollout has completed successfully.
        Succeeded = 4,
    }
    impl RolloutState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RolloutState::Unspecified => "ROLLOUT_STATE_UNSPECIFIED",
                RolloutState::InProgress => "IN_PROGRESS",
                RolloutState::Cancelling => "CANCELLING",
                RolloutState::Cancelled => "CANCELLED",
                RolloutState::Succeeded => "SUCCEEDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLLOUT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "CANCELLING" => Some(Self::Cancelling),
                "CANCELLED" => Some(Self::Cancelled),
                "SUCCEEDED" => Some(Self::Succeeded),
                _ => None,
            }
        }
    }
}
/// OS policy assignment operation metadata provided by OS policy assignment API
/// methods that return long running operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicyAssignmentOperationMetadata {
    /// Reference to the `OSPolicyAssignment` API resource.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}`
    #[prost(string, tag = "1")]
    pub os_policy_assignment: ::prost::alloc::string::String,
    /// The OS policy assignment API method.
    #[prost(
        enumeration = "os_policy_assignment_operation_metadata::ApiMethod",
        tag = "2"
    )]
    pub api_method: i32,
    /// State of the rollout
    #[prost(
        enumeration = "os_policy_assignment_operation_metadata::RolloutState",
        tag = "3"
    )]
    pub rollout_state: i32,
    /// Rollout start time
    #[prost(message, optional, tag = "4")]
    pub rollout_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Rollout update time
    #[prost(message, optional, tag = "5")]
    pub rollout_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `OSPolicyAssignmentOperationMetadata`.
pub mod os_policy_assignment_operation_metadata {
    /// The OS policy assignment API method.
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
    pub enum ApiMethod {
        /// Invalid value
        Unspecified = 0,
        /// Create OS policy assignment API method
        Create = 1,
        /// Update OS policy assignment API method
        Update = 2,
        /// Delete OS policy assignment API method
        Delete = 3,
    }
    impl ApiMethod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ApiMethod::Unspecified => "API_METHOD_UNSPECIFIED",
                ApiMethod::Create => "CREATE",
                ApiMethod::Update => "UPDATE",
                ApiMethod::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "API_METHOD_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATE" => Some(Self::Create),
                "UPDATE" => Some(Self::Update),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
    /// State of the rollout
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
    pub enum RolloutState {
        /// Invalid value
        Unspecified = 0,
        /// The rollout is in progress.
        InProgress = 1,
        /// The rollout is being cancelled.
        Cancelling = 2,
        /// The rollout is cancelled.
        Cancelled = 3,
        /// The rollout has completed successfully.
        Succeeded = 4,
    }
    impl RolloutState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RolloutState::Unspecified => "ROLLOUT_STATE_UNSPECIFIED",
                RolloutState::InProgress => "IN_PROGRESS",
                RolloutState::Cancelling => "CANCELLING",
                RolloutState::Cancelled => "CANCELLED",
                RolloutState::Succeeded => "SUCCEEDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLLOUT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "CANCELLING" => Some(Self::Cancelling),
                "CANCELLED" => Some(Self::Cancelled),
                "SUCCEEDED" => Some(Self::Succeeded),
                _ => None,
            }
        }
    }
}
/// A request message to create an OS policy assignment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOsPolicyAssignmentRequest {
    /// Required. The parent resource name in the form:
    /// projects/{project}/locations/{location}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The OS policy assignment to be created.
    #[prost(message, optional, tag = "2")]
    pub os_policy_assignment: ::core::option::Option<OsPolicyAssignment>,
    /// Required. The logical name of the OS policy assignment in the project
    /// with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the project.
    #[prost(string, tag = "3")]
    pub os_policy_assignment_id: ::prost::alloc::string::String,
}
/// A request message to update an OS policy assignment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOsPolicyAssignmentRequest {
    /// Required. The updated OS policy assignment.
    #[prost(message, optional, tag = "1")]
    pub os_policy_assignment: ::core::option::Option<OsPolicyAssignment>,
    /// Optional. Field mask that controls which fields of the assignment should be
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request message to get an OS policy assignment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOsPolicyAssignmentRequest {
    /// Required. The resource name of OS policy assignment.
    ///
    /// Format:
    /// `projects/{project}/locations/{location}/osPolicyAssignments/{os_policy_assignment}@{revisionId}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message to list OS policy assignments for a parent resource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsPolicyAssignmentsRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of assignments to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to
    /// `ListOSPolicyAssignments` that indicates where this listing should continue
    /// from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response message for listing all assignments under given parent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsPolicyAssignmentsResponse {
    /// The list of assignments
    #[prost(message, repeated, tag = "1")]
    pub os_policy_assignments: ::prost::alloc::vec::Vec<OsPolicyAssignment>,
    /// The pagination token to retrieve the next page of OS policy assignments.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request message to list revisions for a OS policy assignment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsPolicyAssignmentRevisionsRequest {
    /// Required. The name of the OS policy assignment to list revisions for.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of revisions to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to
    /// `ListOSPolicyAssignmentRevisions` that indicates where this listing should
    /// continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response message for listing all revisions for a OS policy assignment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsPolicyAssignmentRevisionsResponse {
    /// The OS policy assignment revisions
    #[prost(message, repeated, tag = "1")]
    pub os_policy_assignments: ::prost::alloc::vec::Vec<OsPolicyAssignment>,
    /// The pagination token to retrieve the next page of OS policy assignment
    /// revisions.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request message for deleting a OS policy assignment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOsPolicyAssignmentRequest {
    /// Required. The name of the OS policy assignment to be deleted
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message to initiate patching across Compute Engine
/// instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutePatchJobRequest {
    /// Required. The project in which to run this patch in the form `projects/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Description of the patch job. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Instances to patch, either explicitly or filtered by some
    /// criteria such as zone or labels.
    #[prost(message, optional, tag = "7")]
    pub instance_filter: ::core::option::Option<PatchInstanceFilter>,
    /// Patch configuration being applied. If omitted, instances are
    /// patched using the default configurations.
    #[prost(message, optional, tag = "4")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// Duration of the patch job. After the duration ends, the patch job
    /// times out.
    #[prost(message, optional, tag = "5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// If this patch is a dry-run only, instances are contacted but
    /// will do nothing.
    #[prost(bool, tag = "6")]
    pub dry_run: bool,
    /// Display name for this patch job. This does not have to be unique.
    #[prost(string, tag = "8")]
    pub display_name: ::prost::alloc::string::String,
    /// Rollout strategy of the patch job.
    #[prost(message, optional, tag = "9")]
    pub rollout: ::core::option::Option<PatchRollout>,
}
/// Request to get an active or completed patch job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPatchJobRequest {
    /// Required. Name of the patch in the form `projects/*/patchJobs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to list details for all instances that are part of a patch job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobInstanceDetailsRequest {
    /// Required. The parent for the instances are in the form of
    /// `projects/*/patchJobs/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of instance details records to return.  Default is 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters results listed in the response. This
    /// field supports filtering results by instance zone, name, state, or
    /// `failure_reason`.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing the instances details for a patch job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobInstanceDetailsResponse {
    /// A list of instance status.
    #[prost(message, repeated, tag = "1")]
    pub patch_job_instance_details: ::prost::alloc::vec::Vec<PatchJobInstanceDetails>,
    /// A pagination token that can be used to get the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Patch details for a VM instance. For more information about reviewing VM
/// instance details, see
/// [Listing all VM instance details for a specific patch
/// job](<https://cloud.google.com/compute/docs/os-patch-management/manage-patch-jobs#list-instance-details>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchJobInstanceDetails {
    /// The instance name in the form `projects/*/zones/*/instances/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The unique identifier for the instance. This identifier is
    /// defined by the server.
    #[prost(string, tag = "2")]
    pub instance_system_id: ::prost::alloc::string::String,
    /// Current state of instance patch.
    #[prost(enumeration = "instance::PatchState", tag = "3")]
    pub state: i32,
    /// If the patch fails, this field provides the reason.
    #[prost(string, tag = "4")]
    pub failure_reason: ::prost::alloc::string::String,
    /// The number of times the agent that the agent attempts to apply the patch.
    #[prost(int64, tag = "5")]
    pub attempt_count: i64,
}
/// A request message for listing patch jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobsRequest {
    /// Required. In the form of `projects/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of instance status to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by patch
    /// jobs to be included in the response.
    /// Currently, filtering is only available on the patch_deployment field.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing patch jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobsResponse {
    /// The list of patch jobs.
    #[prost(message, repeated, tag = "1")]
    pub patch_jobs: ::prost::alloc::vec::Vec<PatchJob>,
    /// A pagination token that can be used to get the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A high level representation of a patch job that is either in progress
/// or has completed.
///
/// Instance details are not included in the job. To paginate through instance
/// details, use ListPatchJobInstanceDetails.
///
/// For more information about patch jobs, see
/// [Creating patch
/// jobs](<https://cloud.google.com/compute/docs/os-patch-management/create-patch-job>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchJob {
    /// Unique identifier for this patch job in the form
    /// `projects/*/patchJobs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Display name for this patch job. This is not a unique identifier.
    #[prost(string, tag = "14")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the patch job. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Time this patch job was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Last time this patch job was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The current state of the PatchJob.
    #[prost(enumeration = "patch_job::State", tag = "5")]
    pub state: i32,
    /// Instances to patch.
    #[prost(message, optional, tag = "13")]
    pub instance_filter: ::core::option::Option<PatchInstanceFilter>,
    /// Patch configuration being applied.
    #[prost(message, optional, tag = "7")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// Duration of the patch job. After the duration ends, the
    /// patch job times out.
    #[prost(message, optional, tag = "8")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Summary of instance details.
    #[prost(message, optional, tag = "9")]
    pub instance_details_summary: ::core::option::Option<
        patch_job::InstanceDetailsSummary,
    >,
    /// If this patch job is a dry run, the agent reports that it has
    /// finished without running any updates on the VM instance.
    #[prost(bool, tag = "10")]
    pub dry_run: bool,
    /// If this patch job failed, this message provides information about the
    /// failure.
    #[prost(string, tag = "11")]
    pub error_message: ::prost::alloc::string::String,
    /// Reflects the overall progress of the patch job in the range of
    /// 0.0 being no progress to 100.0 being complete.
    #[prost(double, tag = "12")]
    pub percent_complete: f64,
    /// Output only. Name of the patch deployment that created this patch job.
    #[prost(string, tag = "15")]
    pub patch_deployment: ::prost::alloc::string::String,
    /// Rollout strategy being applied.
    #[prost(message, optional, tag = "16")]
    pub rollout: ::core::option::Option<PatchRollout>,
}
/// Nested message and enum types in `PatchJob`.
pub mod patch_job {
    /// A summary of the current patch state across all instances that this patch
    /// job affects. Contains counts of instances in different states. These states
    /// map to `InstancePatchState`. List patch job instance details to see the
    /// specific states of each instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceDetailsSummary {
        /// Number of instances pending patch job.
        #[prost(int64, tag = "1")]
        pub pending_instance_count: i64,
        /// Number of instances that are inactive.
        #[prost(int64, tag = "2")]
        pub inactive_instance_count: i64,
        /// Number of instances notified about patch job.
        #[prost(int64, tag = "3")]
        pub notified_instance_count: i64,
        /// Number of instances that have started.
        #[prost(int64, tag = "4")]
        pub started_instance_count: i64,
        /// Number of instances that are downloading patches.
        #[prost(int64, tag = "5")]
        pub downloading_patches_instance_count: i64,
        /// Number of instances that are applying patches.
        #[prost(int64, tag = "6")]
        pub applying_patches_instance_count: i64,
        /// Number of instances rebooting.
        #[prost(int64, tag = "7")]
        pub rebooting_instance_count: i64,
        /// Number of instances that have completed successfully.
        #[prost(int64, tag = "8")]
        pub succeeded_instance_count: i64,
        /// Number of instances that require reboot.
        #[prost(int64, tag = "9")]
        pub succeeded_reboot_required_instance_count: i64,
        /// Number of instances that failed.
        #[prost(int64, tag = "10")]
        pub failed_instance_count: i64,
        /// Number of instances that have acked and will start shortly.
        #[prost(int64, tag = "11")]
        pub acked_instance_count: i64,
        /// Number of instances that exceeded the time out while applying the patch.
        #[prost(int64, tag = "12")]
        pub timed_out_instance_count: i64,
        /// Number of instances that are running the pre-patch step.
        #[prost(int64, tag = "13")]
        pub pre_patch_step_instance_count: i64,
        /// Number of instances that are running the post-patch step.
        #[prost(int64, tag = "14")]
        pub post_patch_step_instance_count: i64,
        /// Number of instances that do not appear to be running the agent. Check to
        /// ensure that the agent is installed, running, and able to communicate with
        /// the service.
        #[prost(int64, tag = "15")]
        pub no_agent_detected_instance_count: i64,
    }
    /// Enumeration of the various states a patch job passes through as it
    /// executes.
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
        /// State must be specified.
        Unspecified = 0,
        /// The patch job was successfully initiated.
        Started = 1,
        /// The patch job is looking up instances to run the patch on.
        InstanceLookup = 2,
        /// Instances are being patched.
        Patching = 3,
        /// Patch job completed successfully.
        Succeeded = 4,
        /// Patch job completed but there were errors.
        CompletedWithErrors = 5,
        /// The patch job was canceled.
        Canceled = 6,
        /// The patch job timed out.
        TimedOut = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Started => "STARTED",
                State::InstanceLookup => "INSTANCE_LOOKUP",
                State::Patching => "PATCHING",
                State::Succeeded => "SUCCEEDED",
                State::CompletedWithErrors => "COMPLETED_WITH_ERRORS",
                State::Canceled => "CANCELED",
                State::TimedOut => "TIMED_OUT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STARTED" => Some(Self::Started),
                "INSTANCE_LOOKUP" => Some(Self::InstanceLookup),
                "PATCHING" => Some(Self::Patching),
                "SUCCEEDED" => Some(Self::Succeeded),
                "COMPLETED_WITH_ERRORS" => Some(Self::CompletedWithErrors),
                "CANCELED" => Some(Self::Canceled),
                "TIMED_OUT" => Some(Self::TimedOut),
                _ => None,
            }
        }
    }
}
/// Patch configuration specifications. Contains details on how to apply the
/// patch(es) to a VM instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchConfig {
    /// Post-patch reboot settings.
    #[prost(enumeration = "patch_config::RebootConfig", tag = "1")]
    pub reboot_config: i32,
    /// Apt update settings. Use this setting to override the default `apt` patch
    /// rules.
    #[prost(message, optional, tag = "3")]
    pub apt: ::core::option::Option<AptSettings>,
    /// Yum update settings. Use this setting to override the default `yum` patch
    /// rules.
    #[prost(message, optional, tag = "4")]
    pub yum: ::core::option::Option<YumSettings>,
    /// Goo update settings. Use this setting to override the default `goo` patch
    /// rules.
    #[prost(message, optional, tag = "5")]
    pub goo: ::core::option::Option<GooSettings>,
    /// Zypper update settings. Use this setting to override the default `zypper`
    /// patch rules.
    #[prost(message, optional, tag = "6")]
    pub zypper: ::core::option::Option<ZypperSettings>,
    /// Windows update settings. Use this override the default windows patch rules.
    #[prost(message, optional, tag = "7")]
    pub windows_update: ::core::option::Option<WindowsUpdateSettings>,
    /// The `ExecStep` to run before the patch update.
    #[prost(message, optional, tag = "8")]
    pub pre_step: ::core::option::Option<ExecStep>,
    /// The `ExecStep` to run after the patch update.
    #[prost(message, optional, tag = "9")]
    pub post_step: ::core::option::Option<ExecStep>,
    /// Allows the patch job to run on Managed instance groups (MIGs).
    #[prost(bool, tag = "10")]
    pub mig_instances_allowed: bool,
}
/// Nested message and enum types in `PatchConfig`.
pub mod patch_config {
    /// Post-patch reboot settings.
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
    pub enum RebootConfig {
        /// The default behavior is DEFAULT.
        Unspecified = 0,
        /// The agent decides if a reboot is necessary by checking signals such as
        /// registry keys on Windows or `/var/run/reboot-required` on APT based
        /// systems. On RPM based systems, a set of core system package install times
        /// are compared with system boot time.
        Default = 1,
        /// Always reboot the machine after the update completes.
        Always = 2,
        /// Never reboot the machine after the update completes.
        Never = 3,
    }
    impl RebootConfig {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RebootConfig::Unspecified => "REBOOT_CONFIG_UNSPECIFIED",
                RebootConfig::Default => "DEFAULT",
                RebootConfig::Always => "ALWAYS",
                RebootConfig::Never => "NEVER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REBOOT_CONFIG_UNSPECIFIED" => Some(Self::Unspecified),
                "DEFAULT" => Some(Self::Default),
                "ALWAYS" => Some(Self::Always),
                "NEVER" => Some(Self::Never),
                _ => None,
            }
        }
    }
}
/// Namespace for instance state enums.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Patch state of an instance.
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
    pub enum PatchState {
        /// Unspecified.
        Unspecified = 0,
        /// The instance is not yet notified.
        Pending = 1,
        /// Instance is inactive and cannot be patched.
        Inactive = 2,
        /// The instance is notified that it should be patched.
        Notified = 3,
        /// The instance has started the patching process.
        Started = 4,
        /// The instance is downloading patches.
        DownloadingPatches = 5,
        /// The instance is applying patches.
        ApplyingPatches = 6,
        /// The instance is rebooting.
        Rebooting = 7,
        /// The instance has completed applying patches.
        Succeeded = 8,
        /// The instance has completed applying patches but a reboot is required.
        SucceededRebootRequired = 9,
        /// The instance has failed to apply the patch.
        Failed = 10,
        /// The instance acked the notification and will start shortly.
        Acked = 11,
        /// The instance exceeded the time out while applying the patch.
        TimedOut = 12,
        /// The instance is running the pre-patch step.
        RunningPrePatchStep = 13,
        /// The instance is running the post-patch step.
        RunningPostPatchStep = 14,
        /// The service could not detect the presence of the agent. Check to ensure
        /// that the agent is installed, running, and able to communicate with the
        /// service.
        NoAgentDetected = 15,
    }
    impl PatchState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PatchState::Unspecified => "PATCH_STATE_UNSPECIFIED",
                PatchState::Pending => "PENDING",
                PatchState::Inactive => "INACTIVE",
                PatchState::Notified => "NOTIFIED",
                PatchState::Started => "STARTED",
                PatchState::DownloadingPatches => "DOWNLOADING_PATCHES",
                PatchState::ApplyingPatches => "APPLYING_PATCHES",
                PatchState::Rebooting => "REBOOTING",
                PatchState::Succeeded => "SUCCEEDED",
                PatchState::SucceededRebootRequired => "SUCCEEDED_REBOOT_REQUIRED",
                PatchState::Failed => "FAILED",
                PatchState::Acked => "ACKED",
                PatchState::TimedOut => "TIMED_OUT",
                PatchState::RunningPrePatchStep => "RUNNING_PRE_PATCH_STEP",
                PatchState::RunningPostPatchStep => "RUNNING_POST_PATCH_STEP",
                PatchState::NoAgentDetected => "NO_AGENT_DETECTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PATCH_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "INACTIVE" => Some(Self::Inactive),
                "NOTIFIED" => Some(Self::Notified),
                "STARTED" => Some(Self::Started),
                "DOWNLOADING_PATCHES" => Some(Self::DownloadingPatches),
                "APPLYING_PATCHES" => Some(Self::ApplyingPatches),
                "REBOOTING" => Some(Self::Rebooting),
                "SUCCEEDED" => Some(Self::Succeeded),
                "SUCCEEDED_REBOOT_REQUIRED" => Some(Self::SucceededRebootRequired),
                "FAILED" => Some(Self::Failed),
                "ACKED" => Some(Self::Acked),
                "TIMED_OUT" => Some(Self::TimedOut),
                "RUNNING_PRE_PATCH_STEP" => Some(Self::RunningPrePatchStep),
                "RUNNING_POST_PATCH_STEP" => Some(Self::RunningPostPatchStep),
                "NO_AGENT_DETECTED" => Some(Self::NoAgentDetected),
                _ => None,
            }
        }
    }
}
/// Message for canceling a patch job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelPatchJobRequest {
    /// Required. Name of the patch in the form `projects/*/patchJobs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Apt patching is completed by executing `apt-get update && apt-get
/// upgrade`. Additional options can be set to control how this is executed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AptSettings {
    /// By changing the type to DIST, the patching is performed
    /// using `apt-get dist-upgrade` instead.
    #[prost(enumeration = "apt_settings::Type", tag = "1")]
    pub r#type: i32,
    /// List of packages to exclude from update. These packages will be excluded
    #[prost(string, repeated, tag = "2")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field cannot be specified with any other patch configuration
    /// fields.
    #[prost(string, repeated, tag = "3")]
    pub exclusive_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `AptSettings`.
pub mod apt_settings {
    /// Apt patch type.
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
    pub enum Type {
        /// By default, upgrade will be performed.
        Unspecified = 0,
        /// Runs `apt-get dist-upgrade`.
        Dist = 1,
        /// Runs `apt-get upgrade`.
        Upgrade = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Dist => "DIST",
                Type::Upgrade => "UPGRADE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DIST" => Some(Self::Dist),
                "UPGRADE" => Some(Self::Upgrade),
                _ => None,
            }
        }
    }
}
/// Yum patching is performed by executing `yum update`. Additional options
/// can be set to control how this is executed.
///
/// Note that not all settings are supported on all platforms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YumSettings {
    /// Adds the `--security` flag to `yum update`. Not supported on
    /// all platforms.
    #[prost(bool, tag = "1")]
    pub security: bool,
    /// Will cause patch to run `yum update-minimal` instead.
    #[prost(bool, tag = "2")]
    pub minimal: bool,
    /// List of packages to exclude from update. These packages are excluded by
    /// using the yum `--exclude` flag.
    #[prost(string, repeated, tag = "3")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field must not be specified with any other patch
    /// configuration fields.
    #[prost(string, repeated, tag = "4")]
    pub exclusive_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Googet patching is performed by running `googet update`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GooSettings {}
/// Zypper patching is performed by running `zypper patch`.
/// See also <https://en.opensuse.org/SDB:Zypper_manual.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZypperSettings {
    /// Adds the `--with-optional` flag to `zypper patch`.
    #[prost(bool, tag = "1")]
    pub with_optional: bool,
    /// Adds the `--with-update` flag, to `zypper patch`.
    #[prost(bool, tag = "2")]
    pub with_update: bool,
    /// Install only patches with these categories.
    /// Common categories include security, recommended, and feature.
    #[prost(string, repeated, tag = "3")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Install only patches with these severities.
    /// Common severities include critical, important, moderate, and low.
    #[prost(string, repeated, tag = "4")]
    pub severities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of patches to exclude from update.
    #[prost(string, repeated, tag = "5")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of patches to be updated. These are the only patches
    /// that will be installed using 'zypper patch patch:<patch_name>' command.
    /// This field must not be used with any other patch configuration fields.
    #[prost(string, repeated, tag = "6")]
    pub exclusive_patches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Windows patching is performed using the Windows Update Agent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowsUpdateSettings {
    /// Only apply updates of these windows update classifications. If empty, all
    /// updates are applied.
    #[prost(
        enumeration = "windows_update_settings::Classification",
        repeated,
        tag = "1"
    )]
    pub classifications: ::prost::alloc::vec::Vec<i32>,
    /// List of KBs to exclude from update.
    #[prost(string, repeated, tag = "2")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of kbs to be updated. These are the only patches
    /// that will be updated. This field must not be used with other
    /// patch configurations.
    #[prost(string, repeated, tag = "3")]
    pub exclusive_patches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `WindowsUpdateSettings`.
pub mod windows_update_settings {
    /// Microsoft Windows update classifications as defined in
    /// \[1\]
    /// <https://support.microsoft.com/en-us/help/824684/description-of-the-standard-terminology-that-is-used-to-describe-micro>
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
    pub enum Classification {
        /// Invalid. If classifications are included, they must be specified.
        Unspecified = 0,
        /// "A widely released fix for a specific problem that addresses a critical,
        /// non-security-related bug." \[1\]
        Critical = 1,
        /// "A widely released fix for a product-specific, security-related
        /// vulnerability. Security vulnerabilities are rated by their severity. The
        /// severity rating is indicated in the Microsoft security bulletin as
        /// critical, important, moderate, or low." \[1\]
        Security = 2,
        /// "A widely released and frequent software update that contains additions
        /// to a product's definition database. Definition databases are often used
        /// to detect objects that have specific attributes, such as malicious code,
        /// phishing websites, or junk mail." \[1\]
        Definition = 3,
        /// "Software that controls the input and output of a device." \[1\]
        Driver = 4,
        /// "New product functionality that is first distributed outside the context
        /// of a product release and that is typically included in the next full
        /// product release." \[1\]
        FeaturePack = 5,
        /// "A tested, cumulative set of all hotfixes, security updates, critical
        /// updates, and updates. Additionally, service packs may contain additional
        /// fixes for problems that are found internally since the release of the
        /// product. Service packs my also contain a limited number of
        /// customer-requested design changes or features." \[1\]
        ServicePack = 6,
        /// "A utility or feature that helps complete a task or set of tasks." \[1\]
        Tool = 7,
        /// "A tested, cumulative set of hotfixes, security updates, critical
        /// updates, and updates that are packaged together for easy deployment. A
        /// rollup generally targets a specific area, such as security, or a
        /// component of a product, such as Internet Information Services (IIS)." \[1\]
        UpdateRollup = 8,
        /// "A widely released fix for a specific problem. An update addresses a
        /// noncritical, non-security-related bug." \[1\]
        Update = 9,
    }
    impl Classification {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Classification::Unspecified => "CLASSIFICATION_UNSPECIFIED",
                Classification::Critical => "CRITICAL",
                Classification::Security => "SECURITY",
                Classification::Definition => "DEFINITION",
                Classification::Driver => "DRIVER",
                Classification::FeaturePack => "FEATURE_PACK",
                Classification::ServicePack => "SERVICE_PACK",
                Classification::Tool => "TOOL",
                Classification::UpdateRollup => "UPDATE_ROLLUP",
                Classification::Update => "UPDATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLASSIFICATION_UNSPECIFIED" => Some(Self::Unspecified),
                "CRITICAL" => Some(Self::Critical),
                "SECURITY" => Some(Self::Security),
                "DEFINITION" => Some(Self::Definition),
                "DRIVER" => Some(Self::Driver),
                "FEATURE_PACK" => Some(Self::FeaturePack),
                "SERVICE_PACK" => Some(Self::ServicePack),
                "TOOL" => Some(Self::Tool),
                "UPDATE_ROLLUP" => Some(Self::UpdateRollup),
                "UPDATE" => Some(Self::Update),
                _ => None,
            }
        }
    }
}
/// A step that runs an executable for a PatchJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStep {
    /// The ExecStepConfig for all Linux VMs targeted by the PatchJob.
    #[prost(message, optional, tag = "1")]
    pub linux_exec_step_config: ::core::option::Option<ExecStepConfig>,
    /// The ExecStepConfig for all Windows VMs targeted by the PatchJob.
    #[prost(message, optional, tag = "2")]
    pub windows_exec_step_config: ::core::option::Option<ExecStepConfig>,
}
/// Common configurations for an ExecStep.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepConfig {
    /// Defaults to \[0\]. A list of possible return values that the
    /// execution can return to indicate a success.
    #[prost(int32, repeated, tag = "3")]
    pub allowed_success_codes: ::prost::alloc::vec::Vec<i32>,
    /// The script interpreter to use to run the script. If no interpreter is
    /// specified the script will be executed directly, which will likely
    /// only succeed for scripts with \[shebang lines\]
    /// (<https://en.wikipedia.org/wiki/Shebang_\(Unix\>)).
    #[prost(enumeration = "exec_step_config::Interpreter", tag = "4")]
    pub interpreter: i32,
    /// Location of the executable.
    #[prost(oneof = "exec_step_config::Executable", tags = "1, 2")]
    pub executable: ::core::option::Option<exec_step_config::Executable>,
}
/// Nested message and enum types in `ExecStepConfig`.
pub mod exec_step_config {
    /// The interpreter used to execute the a file.
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
    pub enum Interpreter {
        /// Invalid for a Windows ExecStepConfig. For a Linux ExecStepConfig, the
        /// interpreter will be parsed from the shebang line of the script if
        /// unspecified.
        Unspecified = 0,
        /// Indicates that the script is run with `/bin/sh` on Linux and `cmd`
        /// on Windows.
        Shell = 1,
        /// Indicates that the file is run with PowerShell flags
        /// `-NonInteractive`, `-NoProfile`, and `-ExecutionPolicy Bypass`.
        Powershell = 2,
    }
    impl Interpreter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Interpreter::Unspecified => "INTERPRETER_UNSPECIFIED",
                Interpreter::Shell => "SHELL",
                Interpreter::Powershell => "POWERSHELL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTERPRETER_UNSPECIFIED" => Some(Self::Unspecified),
                "SHELL" => Some(Self::Shell),
                "POWERSHELL" => Some(Self::Powershell),
                _ => None,
            }
        }
    }
    /// Location of the executable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Executable {
        /// An absolute path to the executable on the VM.
        #[prost(string, tag = "1")]
        LocalPath(::prost::alloc::string::String),
        /// A Cloud Storage object containing the executable.
        #[prost(message, tag = "2")]
        GcsObject(super::GcsObject),
    }
}
/// Cloud Storage object representation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsObject {
    /// Required. Bucket of the Cloud Storage object.
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. Name of the Cloud Storage object.
    #[prost(string, tag = "2")]
    pub object: ::prost::alloc::string::String,
    /// Required. Generation number of the Cloud Storage object. This is used to
    /// ensure that the ExecStep specified by this PatchJob does not change.
    #[prost(int64, tag = "3")]
    pub generation_number: i64,
}
/// A filter to target VM instances for patching. The targeted
/// VMs must meet all criteria specified. So if both labels and zones are
/// specified, the patch job targets only VMs with those labels and in those
/// zones.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchInstanceFilter {
    /// Target all VM instances in the project. If true, no other criteria is
    /// permitted.
    #[prost(bool, tag = "1")]
    pub all: bool,
    /// Targets VM instances matching ANY of these GroupLabels. This allows
    /// targeting of disparate groups of VM instances.
    #[prost(message, repeated, tag = "2")]
    pub group_labels: ::prost::alloc::vec::Vec<patch_instance_filter::GroupLabel>,
    /// Targets VM instances in ANY of these zones. Leave empty to target VM
    /// instances in any zone.
    #[prost(string, repeated, tag = "3")]
    pub zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Targets any of the VM instances specified. Instances are specified by their
    /// URI in the form `zones/\[ZONE\]/instances/\[INSTANCE_NAME\]`,
    /// `projects/\[PROJECT_ID\]/zones/\[ZONE\]/instances/\[INSTANCE_NAME\]`, or
    /// `<https://www.googleapis.com/compute/v1/projects/\[PROJECT_ID\]/zones/\[ZONE\]/instances/\[INSTANCE_NAME\]`>
    #[prost(string, repeated, tag = "4")]
    pub instances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Targets VMs whose name starts with one of these prefixes. Similar to
    /// labels, this is another way to group VMs when targeting configs, for
    /// example prefix="prod-".
    #[prost(string, repeated, tag = "5")]
    pub instance_name_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `PatchInstanceFilter`.
pub mod patch_instance_filter {
    /// Targets a group of VM instances by using their [assigned
    /// labels](<https://cloud.google.com/compute/docs/labeling-resources>). Labels
    /// are key-value pairs. A `GroupLabel` is a combination of labels
    /// that is used to target VMs for a patch job.
    ///
    /// For example, a patch job can target VMs that have the following
    /// `GroupLabel`: `{"env":"test", "app":"web"}`. This means that the patch job
    /// is applied to VMs that have both the labels `env=test` and `app=web`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupLabel {
        /// Compute Engine instance labels that must be present for a VM
        /// instance to be targeted by this filter.
        #[prost(map = "string, string", tag = "1")]
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
}
/// Patch rollout configuration specifications. Contains details on the
/// concurrency control when applying patch(es) to all targeted VMs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchRollout {
    /// Mode of the patch rollout.
    #[prost(enumeration = "patch_rollout::Mode", tag = "1")]
    pub mode: i32,
    /// The maximum number (or percentage) of VMs per zone to disrupt at any given
    /// moment. The number of VMs calculated from multiplying the percentage by the
    /// total number of VMs in a zone is rounded up.
    ///
    /// During patching, a VM is considered disrupted from the time the agent is
    /// notified to begin until patching has completed. This disruption time
    /// includes the time to complete reboot and any post-patch steps.
    ///
    /// A VM contributes to the disruption budget if its patching operation fails
    /// either when applying the patches, running pre or post patch steps, or if it
    /// fails to respond with a success notification before timing out. VMs that
    /// are not running or do not have an active agent do not count toward this
    /// disruption budget.
    ///
    /// For zone-by-zone rollouts, if the disruption budget in a zone is exceeded,
    /// the patch job stops, because continuing to the next zone requires
    /// completion of the patch process in the previous zone.
    ///
    /// For example, if the disruption budget has a fixed value of `10`, and 8 VMs
    /// fail to patch in the current zone, the patch job continues to patch 2 VMs
    /// at a time until the zone is completed. When that zone is completed
    /// successfully, patching begins with 10 VMs at a time in the next zone. If 10
    /// VMs in the next zone fail to patch, the patch job stops.
    #[prost(message, optional, tag = "2")]
    pub disruption_budget: ::core::option::Option<FixedOrPercent>,
}
/// Nested message and enum types in `PatchRollout`.
pub mod patch_rollout {
    /// Type of the rollout.
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
    pub enum Mode {
        /// Mode must be specified.
        Unspecified = 0,
        /// Patches are applied one zone at a time. The patch job begins in the
        /// region with the lowest number of targeted VMs. Within the region,
        /// patching begins in the zone with the lowest number of targeted VMs. If
        /// multiple regions (or zones within a region) have the same number of
        /// targeted VMs, a tie-breaker is achieved by sorting the regions or zones
        /// in alphabetical order.
        ZoneByZone = 1,
        /// Patches are applied to VMs in all zones at the same time.
        ConcurrentZones = 2,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Unspecified => "MODE_UNSPECIFIED",
                Mode::ZoneByZone => "ZONE_BY_ZONE",
                Mode::ConcurrentZones => "CONCURRENT_ZONES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "ZONE_BY_ZONE" => Some(Self::ZoneByZone),
                "CONCURRENT_ZONES" => Some(Self::ConcurrentZones),
                _ => None,
            }
        }
    }
}
/// Patch deployments are configurations that individual patch jobs use to
/// complete a patch. These configurations include instance filter, package
/// repository settings, and a schedule. For more information about creating and
/// managing patch deployments, see [Scheduling patch
/// jobs](<https://cloud.google.com/compute/docs/os-patch-management/schedule-patch-jobs>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchDeployment {
    /// Unique name for the patch deployment resource in a project. The patch
    /// deployment name is in the form:
    /// `projects/{project_id}/patchDeployments/{patch_deployment_id}`.
    /// This field is ignored when you create a new patch deployment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Description of the patch deployment. Length of the description is
    /// limited to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. VM instances to patch.
    #[prost(message, optional, tag = "3")]
    pub instance_filter: ::core::option::Option<PatchInstanceFilter>,
    /// Optional. Patch configuration that is applied.
    #[prost(message, optional, tag = "4")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// Optional. Duration of the patch. After the duration ends, the patch times
    /// out.
    #[prost(message, optional, tag = "5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Output only. Time the patch deployment was created. Timestamp is in
    /// [RFC3339](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the patch deployment was last updated. Timestamp is in
    /// [RFC3339](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time a patch job was started by this deployment.
    /// Timestamp is in [RFC3339](<https://www.ietf.org/rfc/rfc3339.txt>) text
    /// format.
    #[prost(message, optional, tag = "10")]
    pub last_execute_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Rollout strategy of the patch job.
    #[prost(message, optional, tag = "11")]
    pub rollout: ::core::option::Option<PatchRollout>,
    /// Output only. Current state of the patch deployment.
    #[prost(enumeration = "patch_deployment::State", tag = "12")]
    pub state: i32,
    /// Schedule for the patch.
    #[prost(oneof = "patch_deployment::Schedule", tags = "6, 7")]
    pub schedule: ::core::option::Option<patch_deployment::Schedule>,
}
/// Nested message and enum types in `PatchDeployment`.
pub mod patch_deployment {
    /// Represents state of patch peployment.
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
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// Active value means that patch deployment generates Patch Jobs.
        Active = 1,
        /// Paused value means that patch deployment does not generate
        /// Patch jobs. Requires user action to move in and out from this state.
        Paused = 2,
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
                State::Paused => "PAUSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "PAUSED" => Some(Self::Paused),
                _ => None,
            }
        }
    }
    /// Schedule for the patch.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schedule {
        /// Required. Schedule a one-time execution.
        #[prost(message, tag = "6")]
        OneTimeSchedule(super::OneTimeSchedule),
        /// Required. Schedule recurring executions.
        #[prost(message, tag = "7")]
        RecurringSchedule(super::RecurringSchedule),
    }
}
/// Sets the time for a one time patch deployment. Timestamp is in
/// [RFC3339](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneTimeSchedule {
    /// Required. The desired patch job execution time.
    #[prost(message, optional, tag = "1")]
    pub execute_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Sets the time for recurring patch deployments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecurringSchedule {
    /// Required. Defines the time zone that `time_of_day` is relative to.
    /// The rules for daylight saving time are determined by the chosen time zone.
    #[prost(message, optional, tag = "1")]
    pub time_zone: ::core::option::Option<super::super::super::r#type::TimeZone>,
    /// Optional. The time that the recurring schedule becomes effective.
    /// Defaults to `create_time` of the patch deployment.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The end time at which a recurring patch deployment schedule is no
    /// longer active.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Time of the day to run a recurring deployment.
    #[prost(message, optional, tag = "4")]
    pub time_of_day: ::core::option::Option<super::super::super::r#type::TimeOfDay>,
    /// Required. The frequency unit of this recurring schedule.
    #[prost(enumeration = "recurring_schedule::Frequency", tag = "5")]
    pub frequency: i32,
    /// Output only. The time the last patch job ran successfully.
    #[prost(message, optional, tag = "9")]
    pub last_execute_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the next patch job is scheduled to run.
    #[prost(message, optional, tag = "10")]
    pub next_execute_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Configurations for this recurring schedule.
    /// Configurations must match frequency.
    #[prost(oneof = "recurring_schedule::ScheduleConfig", tags = "6, 7")]
    pub schedule_config: ::core::option::Option<recurring_schedule::ScheduleConfig>,
}
/// Nested message and enum types in `RecurringSchedule`.
pub mod recurring_schedule {
    /// Specifies the frequency of the recurring patch deployments.
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
    pub enum Frequency {
        /// Invalid. A frequency must be specified.
        Unspecified = 0,
        /// Indicates that the frequency of recurrence should be expressed in terms
        /// of weeks.
        Weekly = 1,
        /// Indicates that the frequency of recurrence should be expressed in terms
        /// of months.
        Monthly = 2,
        /// Indicates that the frequency of recurrence should be expressed in terms
        /// of days.
        Daily = 3,
    }
    impl Frequency {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Frequency::Unspecified => "FREQUENCY_UNSPECIFIED",
                Frequency::Weekly => "WEEKLY",
                Frequency::Monthly => "MONTHLY",
                Frequency::Daily => "DAILY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FREQUENCY_UNSPECIFIED" => Some(Self::Unspecified),
                "WEEKLY" => Some(Self::Weekly),
                "MONTHLY" => Some(Self::Monthly),
                "DAILY" => Some(Self::Daily),
                _ => None,
            }
        }
    }
    /// Configurations for this recurring schedule.
    /// Configurations must match frequency.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ScheduleConfig {
        /// Required. Schedule with weekly executions.
        #[prost(message, tag = "6")]
        Weekly(super::WeeklySchedule),
        /// Required. Schedule with monthly executions.
        #[prost(message, tag = "7")]
        Monthly(super::MonthlySchedule),
    }
}
/// Represents a weekly schedule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeeklySchedule {
    /// Required. Day of the week.
    #[prost(enumeration = "super::super::super::r#type::DayOfWeek", tag = "1")]
    pub day_of_week: i32,
}
/// Represents a monthly schedule. An example of a valid monthly schedule is
/// "on the third Tuesday of the month" or "on the 15th of the month".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlySchedule {
    /// One day in a month.
    #[prost(oneof = "monthly_schedule::DayOfMonth", tags = "1, 2")]
    pub day_of_month: ::core::option::Option<monthly_schedule::DayOfMonth>,
}
/// Nested message and enum types in `MonthlySchedule`.
pub mod monthly_schedule {
    /// One day in a month.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DayOfMonth {
        /// Required. Week day in a month.
        #[prost(message, tag = "1")]
        WeekDayOfMonth(super::WeekDayOfMonth),
        /// Required. One day of the month. 1-31 indicates the 1st to the 31st day.
        /// -1 indicates the last day of the month. Months without the target day
        /// will be skipped. For example, a schedule to run "every month on the 31st"
        /// will not run in February, April, June, etc.
        #[prost(int32, tag = "2")]
        MonthDay(i32),
    }
}
/// Represents one week day in a month. An example is "the 4th Sunday".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeekDayOfMonth {
    /// Required. Week number in a month. 1-4 indicates the 1st to 4th week of the
    /// month. -1 indicates the last week of the month.
    #[prost(int32, tag = "1")]
    pub week_ordinal: i32,
    /// Required. A day of the week.
    #[prost(enumeration = "super::super::super::r#type::DayOfWeek", tag = "2")]
    pub day_of_week: i32,
    /// Optional. Represents the number of days before or after the given week day
    /// of month that the patch deployment is scheduled for. For example if
    /// `week_ordinal` and `day_of_week` values point to the second day of the
    /// month and this `day_offset` value is set to `3`, the patch deployment takes
    /// place three days after the second Tuesday of the month. If this value is
    /// negative, for example -5, the patches are deployed five days before before
    /// the second Tuesday of the month. Allowed values are in range \[-30, 30\].
    #[prost(int32, tag = "3")]
    pub day_offset: i32,
}
/// A request message for creating a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePatchDeploymentRequest {
    /// Required. The project to apply this patch deployment to in the form
    /// `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A name for the patch deployment in the project. When creating a
    /// name the following rules apply:
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the project.
    #[prost(string, tag = "2")]
    pub patch_deployment_id: ::prost::alloc::string::String,
    /// Required. The patch deployment to create.
    #[prost(message, optional, tag = "3")]
    pub patch_deployment: ::core::option::Option<PatchDeployment>,
}
/// A request message for retrieving a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for listing patch deployments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchDeploymentsRequest {
    /// Required. The resource name of the parent in the form `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of patch deployments to return. Default is
    /// 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to
    /// ListPatchDeployments that indicates where this listing should continue
    /// from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response message for listing patch deployments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchDeploymentsResponse {
    /// The list of patch deployments.
    #[prost(message, repeated, tag = "1")]
    pub patch_deployments: ::prost::alloc::vec::Vec<PatchDeployment>,
    /// A pagination token that can be used to get the next page of patch
    /// deployments.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request message for deleting a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for updating a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePatchDeploymentRequest {
    /// Required. The patch deployment to Update.
    #[prost(message, optional, tag = "1")]
    pub patch_deployment: ::core::option::Option<PatchDeployment>,
    /// Optional. Field mask that controls which fields of the patch deployment
    /// should be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request message for pausing a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PausePatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for resuming a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumePatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod os_config_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// OS Config API
    ///
    /// The OS Config service is a server-side component that you can use to
    /// manage package installations and patch jobs for virtual machine instances.
    #[derive(Debug, Clone)]
    pub struct OsConfigServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OsConfigServiceClient<tonic::transport::Channel> {
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
    impl<T> OsConfigServiceClient<T>
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
        ) -> OsConfigServiceClient<InterceptedService<T, F>>
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
            OsConfigServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Patch VM instances by creating and running a patch job.
        pub async fn execute_patch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecutePatchJobRequest>,
        ) -> std::result::Result<tonic::Response<super::PatchJob>, tonic::Status> {
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
                "/google.cloud.osconfig.v1.OsConfigService/ExecutePatchJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "ExecutePatchJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the patch job. This can be used to track the progress of an
        /// ongoing patch job or review the details of completed jobs.
        pub async fn get_patch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPatchJobRequest>,
        ) -> std::result::Result<tonic::Response<super::PatchJob>, tonic::Status> {
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
                "/google.cloud.osconfig.v1.OsConfigService/GetPatchJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "GetPatchJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancel a patch job. The patch job must be active. Canceled patch jobs
        /// cannot be restarted.
        pub async fn cancel_patch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelPatchJobRequest>,
        ) -> std::result::Result<tonic::Response<super::PatchJob>, tonic::Status> {
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
                "/google.cloud.osconfig.v1.OsConfigService/CancelPatchJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "CancelPatchJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get a list of patch jobs.
        pub async fn list_patch_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPatchJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPatchJobsResponse>,
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
                "/google.cloud.osconfig.v1.OsConfigService/ListPatchJobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "ListPatchJobs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get a list of instance details for a given patch job.
        pub async fn list_patch_job_instance_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPatchJobInstanceDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPatchJobInstanceDetailsResponse>,
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
                "/google.cloud.osconfig.v1.OsConfigService/ListPatchJobInstanceDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "ListPatchJobInstanceDetails",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create an OS Config patch deployment.
        pub async fn create_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePatchDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchDeployment>,
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
                "/google.cloud.osconfig.v1.OsConfigService/CreatePatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "CreatePatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get an OS Config patch deployment.
        pub async fn get_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPatchDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchDeployment>,
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
                "/google.cloud.osconfig.v1.OsConfigService/GetPatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "GetPatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get a page of OS Config patch deployments.
        pub async fn list_patch_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPatchDeploymentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPatchDeploymentsResponse>,
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
                "/google.cloud.osconfig.v1.OsConfigService/ListPatchDeployments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "ListPatchDeployments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete an OS Config patch deployment.
        pub async fn delete_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePatchDeploymentRequest>,
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
                "/google.cloud.osconfig.v1.OsConfigService/DeletePatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "DeletePatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update an OS Config patch deployment.
        pub async fn update_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePatchDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchDeployment>,
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
                "/google.cloud.osconfig.v1.OsConfigService/UpdatePatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "UpdatePatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Change state of patch deployment to "PAUSED".
        /// Patch deployment in paused state doesn't generate patch jobs.
        pub async fn pause_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::PausePatchDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchDeployment>,
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
                "/google.cloud.osconfig.v1.OsConfigService/PausePatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "PausePatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Change state of patch deployment back to "ACTIVE".
        /// Patch deployment in active state continues to generate patch jobs.
        pub async fn resume_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumePatchDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchDeployment>,
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
                "/google.cloud.osconfig.v1.OsConfigService/ResumePatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigService",
                        "ResumePatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// This API resource represents the vulnerability report for a specified
/// Compute Engine virtual machine (VM) instance at a given point in time.
///
/// For more information, see [Vulnerability
/// reports](<https://cloud.google.com/compute/docs/instances/os-inventory-management#vulnerability-reports>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerabilityReport {
    /// Output only. The `vulnerabilityReport` API resource name.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/instances/{instance_id}/vulnerabilityReport`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. List of vulnerabilities affecting the VM.
    #[prost(message, repeated, tag = "2")]
    pub vulnerabilities: ::prost::alloc::vec::Vec<vulnerability_report::Vulnerability>,
    /// Output only. The timestamp for when the last vulnerability report was generated for the
    /// VM.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `VulnerabilityReport`.
pub mod vulnerability_report {
    /// A vulnerability affecting the VM instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Vulnerability {
        /// Contains metadata as per the upstream feed of the operating system and
        /// NVD.
        #[prost(message, optional, tag = "1")]
        pub details: ::core::option::Option<vulnerability::Details>,
        /// Corresponds to the `INSTALLED_PACKAGE` inventory item on the VM.
        /// This field displays the inventory items affected by this vulnerability.
        /// If the vulnerability report was not updated after the VM inventory
        /// update, these values might not display in VM inventory. For some distros,
        /// this field may be empty.
        #[deprecated]
        #[prost(string, repeated, tag = "2")]
        pub installed_inventory_item_ids: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Corresponds to the `AVAILABLE_PACKAGE` inventory item on the VM.
        /// If the vulnerability report was not updated after the VM inventory
        /// update, these values might not display in VM inventory. If there is no
        /// available fix, the field is empty. The `inventory_item` value specifies
        /// the latest `SoftwarePackage` available to the VM that fixes the
        /// vulnerability.
        #[deprecated]
        #[prost(string, repeated, tag = "3")]
        pub available_inventory_item_ids: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// The timestamp for when the vulnerability was first detected.
        #[prost(message, optional, tag = "4")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The timestamp for when the vulnerability was last modified.
        #[prost(message, optional, tag = "5")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// List of items affected by the vulnerability.
        #[prost(message, repeated, tag = "6")]
        pub items: ::prost::alloc::vec::Vec<vulnerability::Item>,
    }
    /// Nested message and enum types in `Vulnerability`.
    pub mod vulnerability {
        /// Contains metadata information for the vulnerability. This information is
        /// collected from the upstream feed of the operating system.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Details {
            /// The CVE of the vulnerability. CVE cannot be
            /// empty and the combination of <cve, classification> should be unique
            /// across vulnerabilities for a VM.
            #[prost(string, tag = "1")]
            pub cve: ::prost::alloc::string::String,
            /// The CVSS V2 score of this vulnerability. CVSS V2 score is on a scale of
            /// 0 - 10 where 0 indicates low severity and 10 indicates high severity.
            #[prost(float, tag = "2")]
            pub cvss_v2_score: f32,
            /// The full description of the CVSSv3 for this vulnerability from NVD.
            #[prost(message, optional, tag = "3")]
            pub cvss_v3: ::core::option::Option<super::super::CvsSv3>,
            /// Assigned severity/impact ranking from the distro.
            #[prost(string, tag = "4")]
            pub severity: ::prost::alloc::string::String,
            /// The note or description describing the vulnerability from the distro.
            #[prost(string, tag = "5")]
            pub description: ::prost::alloc::string::String,
            /// Corresponds to the references attached to the `VulnerabilityDetails`.
            #[prost(message, repeated, tag = "6")]
            pub references: ::prost::alloc::vec::Vec<details::Reference>,
        }
        /// Nested message and enum types in `Details`.
        pub mod details {
            /// A reference for this vulnerability.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Reference {
                /// The url of the reference.
                #[prost(string, tag = "1")]
                pub url: ::prost::alloc::string::String,
                /// The source of the reference e.g. NVD.
                #[prost(string, tag = "2")]
                pub source: ::prost::alloc::string::String,
            }
        }
        /// OS inventory item that is affected by a vulnerability or fixed as a
        /// result of a vulnerability.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Item {
            /// Corresponds to the `INSTALLED_PACKAGE` inventory item on the VM.
            /// This field displays the inventory items affected by this vulnerability.
            /// If the vulnerability report was not updated after the VM inventory
            /// update, these values might not display in VM inventory. For some
            /// operating systems, this field might be empty.
            #[prost(string, tag = "1")]
            pub installed_inventory_item_id: ::prost::alloc::string::String,
            /// Corresponds to the `AVAILABLE_PACKAGE` inventory item on the VM.
            /// If the vulnerability report was not updated after the VM inventory
            /// update, these values might not display in VM inventory. If there is no
            /// available fix, the field is empty. The `inventory_item` value specifies
            /// the latest `SoftwarePackage` available to the VM that fixes the
            /// vulnerability.
            #[prost(string, tag = "2")]
            pub available_inventory_item_id: ::prost::alloc::string::String,
            /// The recommended [CPE URI](<https://cpe.mitre.org/specification/>) update
            /// that contains a fix for this vulnerability.
            #[prost(string, tag = "3")]
            pub fixed_cpe_uri: ::prost::alloc::string::String,
            /// The upstream OS patch, packages or KB that fixes the vulnerability.
            #[prost(string, tag = "4")]
            pub upstream_fix: ::prost::alloc::string::String,
        }
    }
}
/// A request message for getting the vulnerability report for the specified VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVulnerabilityReportRequest {
    /// Required. API resource name for vulnerability resource.
    ///
    /// Format:
    /// `projects/{project}/locations/{location}/instances/{instance}/vulnerabilityReport`
    ///
    /// For `{project}`, either `project-number` or `project-id` can be provided.
    /// For `{instance}`, either Compute Engine `instance-id` or `instance-name`
    /// can be provided.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for listing vulnerability reports for all VM instances in
/// the specified location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVulnerabilityReportsRequest {
    /// Required. The parent resource name.
    ///
    /// Format: `projects/{project}/locations/{location}/instances/-`
    ///
    /// For `{project}`, either `project-number` or `project-id` can be provided.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to
    /// `ListVulnerabilityReports` that indicates where this listing
    /// should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by a
    /// `vulnerabilityReport` API resource to be included in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing vulnerability reports for all VM instances in
/// the specified location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVulnerabilityReportsResponse {
    /// List of vulnerabilityReport objects.
    #[prost(message, repeated, tag = "1")]
    pub vulnerability_reports: ::prost::alloc::vec::Vec<VulnerabilityReport>,
    /// The pagination token to retrieve the next page of vulnerabilityReports
    /// object.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Common Vulnerability Scoring System version 3.
/// For details, see <https://www.first.org/cvss/specification-document>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CvsSv3 {
    /// The base score is a function of the base metric scores.
    /// <https://www.first.org/cvss/specification-document#Base-Metrics>
    #[prost(float, tag = "1")]
    pub base_score: f32,
    /// The Exploitability sub-score equation is derived from the Base
    /// Exploitability metrics.
    /// <https://www.first.org/cvss/specification-document#2-1-Exploitability-Metrics>
    #[prost(float, tag = "2")]
    pub exploitability_score: f32,
    /// The Impact sub-score equation is derived from the Base Impact metrics.
    #[prost(float, tag = "3")]
    pub impact_score: f32,
    /// This metric reflects the context by which vulnerability exploitation is
    /// possible.
    #[prost(enumeration = "cvs_sv3::AttackVector", tag = "5")]
    pub attack_vector: i32,
    /// This metric describes the conditions beyond the attacker's control that
    /// must exist in order to exploit the vulnerability.
    #[prost(enumeration = "cvs_sv3::AttackComplexity", tag = "6")]
    pub attack_complexity: i32,
    /// This metric describes the level of privileges an attacker must possess
    /// before successfully exploiting the vulnerability.
    #[prost(enumeration = "cvs_sv3::PrivilegesRequired", tag = "7")]
    pub privileges_required: i32,
    /// This metric captures the requirement for a human user, other than the
    /// attacker, to participate in the successful compromise of the vulnerable
    /// component.
    #[prost(enumeration = "cvs_sv3::UserInteraction", tag = "8")]
    pub user_interaction: i32,
    /// The Scope metric captures whether a vulnerability in one vulnerable
    /// component impacts resources in components beyond its security scope.
    #[prost(enumeration = "cvs_sv3::Scope", tag = "9")]
    pub scope: i32,
    /// This metric measures the impact to the confidentiality of the information
    /// resources managed by a software component due to a successfully exploited
    /// vulnerability.
    #[prost(enumeration = "cvs_sv3::Impact", tag = "10")]
    pub confidentiality_impact: i32,
    /// This metric measures the impact to integrity of a successfully exploited
    /// vulnerability.
    #[prost(enumeration = "cvs_sv3::Impact", tag = "11")]
    pub integrity_impact: i32,
    /// This metric measures the impact to the availability of the impacted
    /// component resulting from a successfully exploited vulnerability.
    #[prost(enumeration = "cvs_sv3::Impact", tag = "12")]
    pub availability_impact: i32,
}
/// Nested message and enum types in `CVSSv3`.
pub mod cvs_sv3 {
    /// This metric reflects the context by which vulnerability exploitation is
    /// possible.
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
    pub enum AttackVector {
        /// Invalid value.
        Unspecified = 0,
        /// The vulnerable component is bound to the network stack and the set of
        /// possible attackers extends beyond the other options listed below, up to
        /// and including the entire Internet.
        Network = 1,
        /// The vulnerable component is bound to the network stack, but the attack is
        /// limited at the protocol level to a logically adjacent topology.
        Adjacent = 2,
        /// The vulnerable component is not bound to the network stack and the
        /// attacker's path is via read/write/execute capabilities.
        Local = 3,
        /// The attack requires the attacker to physically touch or manipulate the
        /// vulnerable component.
        Physical = 4,
    }
    impl AttackVector {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AttackVector::Unspecified => "ATTACK_VECTOR_UNSPECIFIED",
                AttackVector::Network => "ATTACK_VECTOR_NETWORK",
                AttackVector::Adjacent => "ATTACK_VECTOR_ADJACENT",
                AttackVector::Local => "ATTACK_VECTOR_LOCAL",
                AttackVector::Physical => "ATTACK_VECTOR_PHYSICAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ATTACK_VECTOR_UNSPECIFIED" => Some(Self::Unspecified),
                "ATTACK_VECTOR_NETWORK" => Some(Self::Network),
                "ATTACK_VECTOR_ADJACENT" => Some(Self::Adjacent),
                "ATTACK_VECTOR_LOCAL" => Some(Self::Local),
                "ATTACK_VECTOR_PHYSICAL" => Some(Self::Physical),
                _ => None,
            }
        }
    }
    /// This metric describes the conditions beyond the attacker's control that
    /// must exist in order to exploit the vulnerability.
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
    pub enum AttackComplexity {
        /// Invalid value.
        Unspecified = 0,
        /// Specialized access conditions or extenuating circumstances do not exist.
        /// An attacker can expect repeatable success when attacking the vulnerable
        /// component.
        Low = 1,
        /// A successful attack depends on conditions beyond the attacker's control.
        /// That is, a successful attack cannot be accomplished at will, but requires
        /// the attacker to invest in some measurable amount of effort in preparation
        /// or execution against the vulnerable component before a successful attack
        /// can be expected.
        High = 2,
    }
    impl AttackComplexity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AttackComplexity::Unspecified => "ATTACK_COMPLEXITY_UNSPECIFIED",
                AttackComplexity::Low => "ATTACK_COMPLEXITY_LOW",
                AttackComplexity::High => "ATTACK_COMPLEXITY_HIGH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ATTACK_COMPLEXITY_UNSPECIFIED" => Some(Self::Unspecified),
                "ATTACK_COMPLEXITY_LOW" => Some(Self::Low),
                "ATTACK_COMPLEXITY_HIGH" => Some(Self::High),
                _ => None,
            }
        }
    }
    /// This metric describes the level of privileges an attacker must possess
    /// before successfully exploiting the vulnerability.
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
    pub enum PrivilegesRequired {
        /// Invalid value.
        Unspecified = 0,
        /// The attacker is unauthorized prior to attack, and therefore does not
        /// require any access to settings or files of the vulnerable system to
        /// carry out an attack.
        None = 1,
        /// The attacker requires privileges that provide basic user capabilities
        /// that could normally affect only settings and files owned by a user.
        /// Alternatively, an attacker with Low privileges has the ability to access
        /// only non-sensitive resources.
        Low = 2,
        /// The attacker requires privileges that provide significant (e.g.,
        /// administrative) control over the vulnerable component allowing access to
        /// component-wide settings and files.
        High = 3,
    }
    impl PrivilegesRequired {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PrivilegesRequired::Unspecified => "PRIVILEGES_REQUIRED_UNSPECIFIED",
                PrivilegesRequired::None => "PRIVILEGES_REQUIRED_NONE",
                PrivilegesRequired::Low => "PRIVILEGES_REQUIRED_LOW",
                PrivilegesRequired::High => "PRIVILEGES_REQUIRED_HIGH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIVILEGES_REQUIRED_NONE" => Some(Self::None),
                "PRIVILEGES_REQUIRED_LOW" => Some(Self::Low),
                "PRIVILEGES_REQUIRED_HIGH" => Some(Self::High),
                _ => None,
            }
        }
    }
    /// This metric captures the requirement for a human user, other than the
    /// attacker, to participate in the successful compromise of the vulnerable
    /// component.
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
    pub enum UserInteraction {
        /// Invalid value.
        Unspecified = 0,
        /// The vulnerable system can be exploited without interaction from any user.
        None = 1,
        /// Successful exploitation of this vulnerability requires a user to take
        /// some action before the vulnerability can be exploited.
        Required = 2,
    }
    impl UserInteraction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserInteraction::Unspecified => "USER_INTERACTION_UNSPECIFIED",
                UserInteraction::None => "USER_INTERACTION_NONE",
                UserInteraction::Required => "USER_INTERACTION_REQUIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_INTERACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "USER_INTERACTION_NONE" => Some(Self::None),
                "USER_INTERACTION_REQUIRED" => Some(Self::Required),
                _ => None,
            }
        }
    }
    /// The Scope metric captures whether a vulnerability in one vulnerable
    /// component impacts resources in components beyond its security scope.
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
    pub enum Scope {
        /// Invalid value.
        Unspecified = 0,
        /// An exploited vulnerability can only affect resources managed by the same
        /// security authority.
        Unchanged = 1,
        /// An exploited vulnerability can affect resources beyond the security scope
        /// managed by the security authority of the vulnerable component.
        Changed = 2,
    }
    impl Scope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Scope::Unspecified => "SCOPE_UNSPECIFIED",
                Scope::Unchanged => "SCOPE_UNCHANGED",
                Scope::Changed => "SCOPE_CHANGED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SCOPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SCOPE_UNCHANGED" => Some(Self::Unchanged),
                "SCOPE_CHANGED" => Some(Self::Changed),
                _ => None,
            }
        }
    }
    /// The Impact metrics capture the effects of a successfully exploited
    /// vulnerability on the component that suffers the worst outcome that is most
    /// directly and predictably associated with the attack.
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
    pub enum Impact {
        /// Invalid value.
        Unspecified = 0,
        /// High impact.
        High = 1,
        /// Low impact.
        Low = 2,
        /// No impact.
        None = 3,
    }
    impl Impact {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Impact::Unspecified => "IMPACT_UNSPECIFIED",
                Impact::High => "IMPACT_HIGH",
                Impact::Low => "IMPACT_LOW",
                Impact::None => "IMPACT_NONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IMPACT_UNSPECIFIED" => Some(Self::Unspecified),
                "IMPACT_HIGH" => Some(Self::High),
                "IMPACT_LOW" => Some(Self::Low),
                "IMPACT_NONE" => Some(Self::None),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod os_config_zonal_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Zonal OS Config API
    ///
    /// The OS Config service is the server-side component that allows users to
    /// manage package installations and patch jobs for Compute Engine VM instances.
    #[derive(Debug, Clone)]
    pub struct OsConfigZonalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OsConfigZonalServiceClient<tonic::transport::Channel> {
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
    impl<T> OsConfigZonalServiceClient<T>
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
        ) -> OsConfigZonalServiceClient<InterceptedService<T, F>>
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
            OsConfigZonalServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Create an OS policy assignment.
        ///
        /// This method also creates the first revision of the OS policy assignment.
        ///
        /// This method returns a long running operation (LRO) that contains the
        /// rollout details. The rollout can be cancelled by cancelling the LRO.
        ///
        /// For more information, see [Method:
        /// projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel).
        pub async fn create_os_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOsPolicyAssignmentRequest>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/CreateOSPolicyAssignment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "CreateOSPolicyAssignment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update an existing OS policy assignment.
        ///
        /// This method creates a new revision of the OS policy assignment.
        ///
        /// This method returns a long running operation (LRO) that contains the
        /// rollout details. The rollout can be cancelled by cancelling the LRO.
        ///
        /// For more information, see [Method:
        /// projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel).
        pub async fn update_os_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOsPolicyAssignmentRequest>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/UpdateOSPolicyAssignment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "UpdateOSPolicyAssignment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieve an existing OS policy assignment.
        ///
        /// This method always returns the latest revision. In order to retrieve a
        /// previous revision of the assignment, also provide the revision ID in the
        /// `name` parameter.
        pub async fn get_os_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOsPolicyAssignmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OsPolicyAssignment>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/GetOSPolicyAssignment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "GetOSPolicyAssignment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List the OS policy assignments under the parent resource.
        ///
        /// For each OS policy assignment, the latest revision is returned.
        pub async fn list_os_policy_assignments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOsPolicyAssignmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOsPolicyAssignmentsResponse>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/ListOSPolicyAssignments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "ListOSPolicyAssignments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List the OS policy assignment revisions for a given OS policy assignment.
        pub async fn list_os_policy_assignment_revisions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListOsPolicyAssignmentRevisionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListOsPolicyAssignmentRevisionsResponse>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/ListOSPolicyAssignmentRevisions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "ListOSPolicyAssignmentRevisions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete the OS policy assignment.
        ///
        /// This method creates a new revision of the OS policy assignment.
        ///
        /// This method returns a long running operation (LRO) that contains the
        /// rollout details. The rollout can be cancelled by cancelling the LRO.
        ///
        /// If the LRO completes and is not cancelled, all revisions associated with
        /// the OS policy assignment are deleted.
        ///
        /// For more information, see [Method:
        /// projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel).
        pub async fn delete_os_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOsPolicyAssignmentRequest>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/DeleteOSPolicyAssignment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "DeleteOSPolicyAssignment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the OS policy asssignment report for the specified Compute Engine VM
        /// instance.
        pub async fn get_os_policy_assignment_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOsPolicyAssignmentReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OsPolicyAssignmentReport>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/GetOSPolicyAssignmentReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "GetOSPolicyAssignmentReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List OS policy asssignment reports for all Compute Engine VM instances in
        /// the specified zone.
        pub async fn list_os_policy_assignment_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOsPolicyAssignmentReportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOsPolicyAssignmentReportsResponse>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/ListOSPolicyAssignmentReports",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "ListOSPolicyAssignmentReports",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get inventory data for the specified VM instance. If the VM has no
        /// associated inventory, the message `NOT_FOUND` is returned.
        pub async fn get_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInventoryRequest>,
        ) -> std::result::Result<tonic::Response<super::Inventory>, tonic::Status> {
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/GetInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "GetInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List inventory data for all VM instances in the specified zone.
        pub async fn list_inventories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInventoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInventoriesResponse>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/ListInventories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "ListInventories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the vulnerability report for the specified VM instance. Only VMs with
        /// inventory data have vulnerability reports associated with them.
        pub async fn get_vulnerability_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVulnerabilityReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VulnerabilityReport>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/GetVulnerabilityReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "GetVulnerabilityReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List vulnerability reports for all VM instances in the specified zone.
        pub async fn list_vulnerability_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVulnerabilityReportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVulnerabilityReportsResponse>,
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
                "/google.cloud.osconfig.v1.OsConfigZonalService/ListVulnerabilityReports",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1.OsConfigZonalService",
                        "ListVulnerabilityReports",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
