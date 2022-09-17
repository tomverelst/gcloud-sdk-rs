/// Information about time ranges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeRange {
    /// The start of the time range.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end of the time range.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Information about a group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// The email address of the group.
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    /// The title of the group.
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
}
/// Information about a domain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domain {
    /// The name of the domain, e.g. `google.com`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// An opaque string used to identify this domain.
    #[prost(string, tag="3")]
    pub legacy_id: ::prost::alloc::string::String,
}
/// The actor of a Drive activity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actor {
    /// The type of actor.
    #[prost(oneof="actor::Type", tags="1, 2, 3, 4, 5")]
    pub r#type: ::core::option::Option<actor::Type>,
}
/// Nested message and enum types in `Actor`.
pub mod actor {
    /// The type of actor.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// An end user.
        #[prost(message, tag="1")]
        User(super::User),
        /// An anonymous user.
        #[prost(message, tag="2")]
        Anonymous(super::AnonymousUser),
        /// An account acting on behalf of another.
        #[prost(message, tag="3")]
        Impersonation(super::Impersonation),
        /// A non-user actor (i.e. system triggered).
        #[prost(message, tag="4")]
        System(super::SystemEvent),
        /// An administrator.
        #[prost(message, tag="5")]
        Administrator(super::Administrator),
    }
}
/// Information about an end user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// The type of user, such as known, unknown, and deleted.
    #[prost(oneof="user::Type", tags="2, 3, 4")]
    pub r#type: ::core::option::Option<user::Type>,
}
/// Nested message and enum types in `User`.
pub mod user {
    /// A known user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KnownUser {
        /// The identifier for this user that can be used with the People API to get
        /// more information. The format is `people/ACCOUNT_ID`. See
        /// <https://developers.google.com/people/.>
        #[prost(string, tag="1")]
        pub person_name: ::prost::alloc::string::String,
        /// True if this is the user making the request.
        #[prost(bool, tag="2")]
        pub is_current_user: bool,
    }
    /// A user whose account has since been deleted.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeletedUser {
    }
    /// A user about whom nothing is currently known.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnknownUser {
    }
    /// The type of user, such as known, unknown, and deleted.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// A known user.
        #[prost(message, tag="2")]
        KnownUser(KnownUser),
        /// A user whose account has since been deleted.
        #[prost(message, tag="3")]
        DeletedUser(DeletedUser),
        /// A user about whom nothing is currently known.
        #[prost(message, tag="4")]
        UnknownUser(UnknownUser),
    }
}
/// Empty message representing an anonymous user or indicating the authenticated
/// user should be anonymized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnonymousUser {
}
/// Information about an impersonation, where an admin acts on behalf of an end
/// user. Information about the acting admin is not currently available.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Impersonation {
    /// The impersonated user.
    #[prost(message, optional, tag="1")]
    pub impersonated_user: ::core::option::Option<User>,
}
/// Event triggered by system operations instead of end users.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemEvent {
    /// The type of the system event that may triggered activity.
    #[prost(enumeration="system_event::Type", tag="1")]
    pub r#type: i32,
}
/// Nested message and enum types in `SystemEvent`.
pub mod system_event {
    /// The types of system events that may trigger activity.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// The event type is unspecified.
        Unspecified = 0,
        /// The event is a consequence of a user account being deleted.
        UserDeletion = 1,
        /// The event is due to the system automatically purging trash.
        TrashAutoPurge = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::UserDeletion => "USER_DELETION",
                Type::TrashAutoPurge => "TRASH_AUTO_PURGE",
            }
        }
    }
}
/// Empty message representing an administrator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Administrator {
}
/// Information about the target of activity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Target {
    /// This field is deprecated; please use the `drive` field instead.
    #[deprecated]
    #[prost(message, optional, tag="2")]
    pub team_drive: ::core::option::Option<TeamDrive>,
    /// The type of target object.
    #[prost(oneof="target::Object", tags="1, 5, 3")]
    pub object: ::core::option::Option<target::Object>,
}
/// Nested message and enum types in `Target`.
pub mod target {
    /// The type of target object.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Object {
        /// The target is a Drive item.
        #[prost(message, tag="1")]
        DriveItem(super::DriveItem),
        /// The target is a shared drive.
        #[prost(message, tag="5")]
        Drive(super::Drive),
        /// The target is a comment on a Drive file.
        #[prost(message, tag="3")]
        FileComment(super::FileComment),
    }
}
/// A lightweight reference to the target of activity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetReference {
    /// This field is deprecated; please use the `drive` field instead.
    #[deprecated]
    #[prost(message, optional, tag="2")]
    pub team_drive: ::core::option::Option<TeamDriveReference>,
    /// The type of target object.
    #[prost(oneof="target_reference::Object", tags="1, 3")]
    pub object: ::core::option::Option<target_reference::Object>,
}
/// Nested message and enum types in `TargetReference`.
pub mod target_reference {
    /// The type of target object.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Object {
        /// The target is a Drive item.
        #[prost(message, tag="1")]
        DriveItem(super::DriveItemReference),
        /// The target is a shared drive.
        #[prost(message, tag="3")]
        Drive(super::DriveReference),
    }
}
/// A comment on a file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileComment {
    /// The comment in the discussion thread. This identifier is an opaque string
    /// compatible with the Drive API; see
    /// <https://developers.google.com/drive/v3/reference/comments/get>
    #[prost(string, tag="1")]
    pub legacy_comment_id: ::prost::alloc::string::String,
    /// The discussion thread to which the comment was added. This identifier is an
    /// opaque string compatible with the Drive API and references the first
    /// comment in a discussion; see
    /// <https://developers.google.com/drive/v3/reference/comments/get>
    #[prost(string, tag="2")]
    pub legacy_discussion_id: ::prost::alloc::string::String,
    /// The link to the discussion thread containing this comment, for example,
    /// `<https://docs.google.com/DOCUMENT_ID/edit?disco=THREAD_ID`.>
    #[prost(string, tag="3")]
    pub link_to_discussion: ::prost::alloc::string::String,
    /// The Drive item containing this comment.
    #[prost(message, optional, tag="4")]
    pub parent: ::core::option::Option<DriveItem>,
}
/// A Drive item, such as a file or folder.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriveItem {
    /// The target Drive item. The format is `items/ITEM_ID`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The title of the Drive item.
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    /// This field is deprecated; please use the `driveFile` field instead.
    #[deprecated]
    #[prost(message, optional, tag="3")]
    pub file: ::core::option::Option<drive_item::File>,
    /// This field is deprecated; please use the `driveFolder` field instead.
    #[deprecated]
    #[prost(message, optional, tag="4")]
    pub folder: ::core::option::Option<drive_item::Folder>,
    /// The MIME type of the Drive item.  See
    /// <https://developers.google.com/drive/v3/web/mime-types.>
    #[prost(string, tag="6")]
    pub mime_type: ::prost::alloc::string::String,
    /// Information about the owner of this Drive item.
    #[prost(message, optional, tag="7")]
    pub owner: ::core::option::Option<Owner>,
    /// If present, this describes the type of the Drive item.
    #[prost(oneof="drive_item::ItemType", tags="8, 9")]
    pub item_type: ::core::option::Option<drive_item::ItemType>,
}
/// Nested message and enum types in `DriveItem`.
pub mod drive_item {
    /// This item is deprecated; please see `DriveFile` instead.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct File {
    }
    /// This item is deprecated; please see `DriveFolder` instead.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Folder {
        /// This field is deprecated; please see `DriveFolder.type` instead.
        #[prost(enumeration="folder::Type", tag="6")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `Folder`.
    pub mod folder {
        /// This item is deprecated; please see `DriveFolder.Type` instead.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Type {
            /// This item is deprecated; please see `DriveFolder.Type` instead.
            Unspecified = 0,
            /// This item is deprecated; please see `DriveFolder.Type` instead.
            MyDriveRoot = 1,
            /// This item is deprecated; please see `DriveFolder.Type` instead.
            TeamDriveRoot = 2,
            /// This item is deprecated; please see `DriveFolder.Type` instead.
            StandardFolder = 3,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::MyDriveRoot => "MY_DRIVE_ROOT",
                    Type::TeamDriveRoot => "TEAM_DRIVE_ROOT",
                    Type::StandardFolder => "STANDARD_FOLDER",
                }
            }
        }
    }
    /// A Drive item which is a file.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DriveFile {
    }
    /// A Drive item which is a folder.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DriveFolder {
        /// The type of Drive folder.
        #[prost(enumeration="drive_folder::Type", tag="6")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `DriveFolder`.
    pub mod drive_folder {
        /// The type of a Drive folder.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Type {
            /// The folder type is unknown.
            Unspecified = 0,
            /// The folder is the root of a user's MyDrive.
            MyDriveRoot = 1,
            /// The folder is the root of a shared drive.
            SharedDriveRoot = 2,
            /// The folder is a standard, non-root, folder.
            StandardFolder = 3,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::MyDriveRoot => "MY_DRIVE_ROOT",
                    Type::SharedDriveRoot => "SHARED_DRIVE_ROOT",
                    Type::StandardFolder => "STANDARD_FOLDER",
                }
            }
        }
    }
    /// If present, this describes the type of the Drive item.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ItemType {
        /// The Drive item is a file.
        #[prost(message, tag="8")]
        DriveFile(DriveFile),
        /// The Drive item is a folder. Includes information about the type of
        /// folder.
        #[prost(message, tag="9")]
        DriveFolder(DriveFolder),
    }
}
/// Information about the owner of a Drive item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Owner {
    /// This field is deprecated; please use the `drive` field instead.
    #[deprecated]
    #[prost(message, optional, tag="2")]
    pub team_drive: ::core::option::Option<TeamDriveReference>,
    /// The domain of the Drive item owner.
    #[prost(message, optional, tag="3")]
    pub domain: ::core::option::Option<Domain>,
    /// The owner of the Drive item.
    #[prost(oneof="owner::Owner", tags="1, 4")]
    pub owner: ::core::option::Option<owner::Owner>,
}
/// Nested message and enum types in `Owner`.
pub mod owner {
    /// The owner of the Drive item.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Owner {
        /// The user that owns the Drive item.
        #[prost(message, tag="1")]
        User(super::User),
        /// The drive that owns the item.
        #[prost(message, tag="4")]
        Drive(super::DriveReference),
    }
}
/// This item is deprecated; please see `Drive` instead.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeamDrive {
    /// This field is deprecated; please see `Drive.name` instead.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// This field is deprecated; please see `Drive.title` instead.
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    /// This field is deprecated; please see `Drive.root` instead.
    #[prost(message, optional, tag="3")]
    pub root: ::core::option::Option<DriveItem>,
}
/// Information about a shared drive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Drive {
    /// The resource name of the shared drive. The format is
    /// `COLLECTION_ID/DRIVE_ID`. Clients should not assume a specific collection
    /// ID for this resource name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The title of the shared drive.
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    /// The root of this shared drive.
    #[prost(message, optional, tag="3")]
    pub root: ::core::option::Option<DriveItem>,
}
/// A lightweight reference to a Drive item, such as a file or folder.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriveItemReference {
    /// The target Drive item. The format is `items/ITEM_ID`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The title of the Drive item.
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    /// This field is deprecated; please use the `driveFile` field instead.
    #[deprecated]
    #[prost(message, optional, tag="3")]
    pub file: ::core::option::Option<drive_item::File>,
    /// This field is deprecated; please use the `driveFolder` field instead.
    #[deprecated]
    #[prost(message, optional, tag="4")]
    pub folder: ::core::option::Option<drive_item::Folder>,
    /// If present, this describes the type of the Drive item.
    #[prost(oneof="drive_item_reference::ItemType", tags="8, 9")]
    pub item_type: ::core::option::Option<drive_item_reference::ItemType>,
}
/// Nested message and enum types in `DriveItemReference`.
pub mod drive_item_reference {
    /// If present, this describes the type of the Drive item.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ItemType {
        /// The Drive item is a file.
        #[prost(message, tag="8")]
        DriveFile(super::drive_item::DriveFile),
        /// The Drive item is a folder. Includes information about the type of
        /// folder.
        #[prost(message, tag="9")]
        DriveFolder(super::drive_item::DriveFolder),
    }
}
/// This item is deprecated; please see `DriveReference` instead.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeamDriveReference {
    /// This field is deprecated; please see `DriveReference.name` instead.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// This field is deprecated; please see `DriveReference.title` instead.
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
}
/// A lightweight reference to a shared drive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriveReference {
    /// The resource name of the shared drive. The format is
    /// `COLLECTION_ID/DRIVE_ID`. Clients should not assume a specific collection
    /// ID for this resource name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The title of the shared drive.
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
}
/// Information about the action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    /// The type and detailed information about the action.
    #[prost(message, optional, tag="1")]
    pub detail: ::core::option::Option<ActionDetail>,
    /// The actor responsible for this action (or empty if all actors are
    /// responsible).
    #[prost(message, optional, tag="3")]
    pub actor: ::core::option::Option<Actor>,
    /// The target this action affects (or empty if affecting all targets). This
    /// represents the state of the target immediately after this action occurred.
    #[prost(message, optional, tag="4")]
    pub target: ::core::option::Option<Target>,
    /// When the action occurred (or empty if same time as entire activity).
    #[prost(oneof="action::Time", tags="5, 6")]
    pub time: ::core::option::Option<action::Time>,
}
/// Nested message and enum types in `Action`.
pub mod action {
    /// When the action occurred (or empty if same time as entire activity).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Time {
        /// The action occurred at this specific time.
        #[prost(message, tag="5")]
        Timestamp(::prost_types::Timestamp),
        /// The action occurred over this time range.
        #[prost(message, tag="6")]
        TimeRange(super::TimeRange),
    }
}
/// Data describing the type and additional information of an action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionDetail {
    /// Data describing the type and additional information of an action.
    #[prost(oneof="action_detail::ActionDetail", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 13")]
    pub action_detail: ::core::option::Option<action_detail::ActionDetail>,
}
/// Nested message and enum types in `ActionDetail`.
pub mod action_detail {
    /// Data describing the type and additional information of an action.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ActionDetail {
        /// An object was created.
        #[prost(message, tag="1")]
        Create(super::Create),
        /// An object was edited.
        #[prost(message, tag="2")]
        Edit(super::Edit),
        /// An object was moved.
        #[prost(message, tag="3")]
        Move(super::Move),
        /// An object was renamed.
        #[prost(message, tag="4")]
        Rename(super::Rename),
        /// An object was deleted.
        #[prost(message, tag="5")]
        Delete(super::Delete),
        /// A deleted object was restored.
        #[prost(message, tag="6")]
        Restore(super::Restore),
        /// The permission on an object was changed.
        #[prost(message, tag="7")]
        PermissionChange(super::PermissionChange),
        /// A change about comments was made.
        #[prost(message, tag="8")]
        Comment(super::Comment),
        /// A change happened in data leak prevention status.
        #[prost(message, tag="9")]
        DlpChange(super::DataLeakPreventionChange),
        /// An object was referenced in an application outside of Drive/Docs.
        #[prost(message, tag="12")]
        Reference(super::ApplicationReference),
        /// Settings were changed.
        #[prost(message, tag="13")]
        SettingsChange(super::SettingsChange),
    }
}
/// An object was created.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Create {
    /// The origin of the new object.
    #[prost(oneof="create::Origin", tags="1, 2, 3")]
    pub origin: ::core::option::Option<create::Origin>,
}
/// Nested message and enum types in `Create`.
pub mod create {
    /// An object was created from scratch.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct New {
    }
    /// An object was uploaded into Drive.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Upload {
    }
    /// An object was created by copying an existing object.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Copy {
        /// The original object.
        #[prost(message, optional, tag="1")]
        pub original_object: ::core::option::Option<super::TargetReference>,
    }
    /// The origin of the new object.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Origin {
        /// If present, indicates the object was newly created (e.g. as a blank
        /// document), not derived from a Drive object or external object.
        #[prost(message, tag="1")]
        New(New),
        /// If present, indicates the object originated externally and was uploaded
        /// to Drive.
        #[prost(message, tag="2")]
        Upload(Upload),
        /// If present, indicates the object was created by copying an existing Drive
        /// object.
        #[prost(message, tag="3")]
        Copy(Copy),
    }
}
/// An empty message indicating an object was edited.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Edit {
}
/// An object was moved.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Move {
    /// The added parent object(s).
    #[prost(message, repeated, tag="1")]
    pub added_parents: ::prost::alloc::vec::Vec<TargetReference>,
    /// The removed parent object(s).
    #[prost(message, repeated, tag="2")]
    pub removed_parents: ::prost::alloc::vec::Vec<TargetReference>,
}
/// An object was renamed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rename {
    /// The previous title of the drive object.
    #[prost(string, tag="1")]
    pub old_title: ::prost::alloc::string::String,
    /// The new title of the drive object.
    #[prost(string, tag="2")]
    pub new_title: ::prost::alloc::string::String,
}
/// An object was deleted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delete {
    /// The type of delete action taken.
    #[prost(enumeration="delete::Type", tag="1")]
    pub r#type: i32,
}
/// Nested message and enum types in `Delete`.
pub mod delete {
    /// The type of deletion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Deletion type is not available.
        Unspecified = 0,
        /// An object was put into the trash.
        Trash = 1,
        /// An object was deleted permanently.
        PermanentDelete = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Trash => "TRASH",
                Type::PermanentDelete => "PERMANENT_DELETE",
            }
        }
    }
}
/// A deleted object was restored.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Restore {
    /// The type of restore action taken.
    #[prost(enumeration="restore::Type", tag="1")]
    pub r#type: i32,
}
/// Nested message and enum types in `Restore`.
pub mod restore {
    /// The type of restoration.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// The type is not available.
        Unspecified = 0,
        /// An object was restored from the trash.
        Untrash = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Untrash => "UNTRASH",
            }
        }
    }
}
/// A change of the permission setting on an item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionChange {
    /// The set of permissions added by this change.
    #[prost(message, repeated, tag="1")]
    pub added_permissions: ::prost::alloc::vec::Vec<Permission>,
    /// The set of permissions removed by this change.
    #[prost(message, repeated, tag="2")]
    pub removed_permissions: ::prost::alloc::vec::Vec<Permission>,
}
/// The permission setting of an object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    /// Indicates the
    /// [Google Drive permissions
    /// role](<https://developers.google.com/drive/web/manage-sharing#roles>). The
    /// role determines a user's ability to read, write, and comment on items.
    #[prost(enumeration="permission::Role", tag="1")]
    pub role: i32,
    /// If true, the item can be discovered (e.g. in the user's "Shared with me"
    /// collection) without needing a link to the item.
    #[prost(bool, tag="6")]
    pub allow_discovery: bool,
    /// The entity granted the role.
    #[prost(oneof="permission::Scope", tags="2, 3, 4, 5")]
    pub scope: ::core::option::Option<permission::Scope>,
}
/// Nested message and enum types in `Permission`.
pub mod permission {
    /// Represents any user (including a logged out user).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Anyone {
    }
    /// The [Google Drive permissions
    /// roles](<https://developers.google.com/drive/web/manage-sharing#roles>).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Role {
        /// The role is not available.
        Unspecified = 0,
        /// A role granting full access.
        Owner = 1,
        /// A role granting the ability to manage people and settings.
        Organizer = 2,
        /// A role granting the ability to contribute and manage content.
        FileOrganizer = 3,
        /// A role granting the ability to contribute content. This role is sometimes
        /// also known as "writer".
        Editor = 4,
        /// A role granting the ability to view and comment on content.
        Commenter = 5,
        /// A role granting the ability to view content. This role is sometimes also
        /// known as "reader".
        Viewer = 6,
        /// A role granting the ability to view content only after it has been
        /// published to the web. This role is sometimes also known as "published
        /// reader". See <https://support.google.com/sites/answer/6372880> for more
        /// information.
        PublishedViewer = 7,
    }
    impl Role {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Role::Unspecified => "ROLE_UNSPECIFIED",
                Role::Owner => "OWNER",
                Role::Organizer => "ORGANIZER",
                Role::FileOrganizer => "FILE_ORGANIZER",
                Role::Editor => "EDITOR",
                Role::Commenter => "COMMENTER",
                Role::Viewer => "VIEWER",
                Role::PublishedViewer => "PUBLISHED_VIEWER",
            }
        }
    }
    /// The entity granted the role.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scope {
        /// The user to whom this permission applies.
        #[prost(message, tag="2")]
        User(super::User),
        /// The group to whom this permission applies.
        #[prost(message, tag="3")]
        Group(super::Group),
        /// The domain to whom this permission applies.
        #[prost(message, tag="4")]
        Domain(super::Domain),
        /// If set, this permission applies to anyone, even logged out users.
        #[prost(message, tag="5")]
        Anyone(Anyone),
    }
}
/// A change about comments on an object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    /// Users who are mentioned in this comment.
    #[prost(message, repeated, tag="7")]
    pub mentioned_users: ::prost::alloc::vec::Vec<User>,
    /// The type of changed comment.
    #[prost(oneof="comment::Type", tags="1, 2, 3")]
    pub r#type: ::core::option::Option<comment::Type>,
}
/// Nested message and enum types in `Comment`.
pub mod comment {
    /// A regular posted comment.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Post {
        /// The sub-type of this event.
        #[prost(enumeration="post::Subtype", tag="1")]
        pub subtype: i32,
    }
    /// Nested message and enum types in `Post`.
    pub mod post {
        /// More detailed information about the change.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Subtype {
            /// Subtype not available.
            Unspecified = 0,
            /// A post was added.
            Added = 1,
            /// A post was deleted.
            Deleted = 2,
            /// A reply was added.
            ReplyAdded = 3,
            /// A reply was deleted.
            ReplyDeleted = 4,
            /// A posted comment was resolved.
            Resolved = 5,
            /// A posted comment was reopened.
            Reopened = 6,
        }
        impl Subtype {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Subtype::Unspecified => "SUBTYPE_UNSPECIFIED",
                    Subtype::Added => "ADDED",
                    Subtype::Deleted => "DELETED",
                    Subtype::ReplyAdded => "REPLY_ADDED",
                    Subtype::ReplyDeleted => "REPLY_DELETED",
                    Subtype::Resolved => "RESOLVED",
                    Subtype::Reopened => "REOPENED",
                }
            }
        }
    }
    /// A comment with an assignment.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Assignment {
        /// The sub-type of this event.
        #[prost(enumeration="assignment::Subtype", tag="1")]
        pub subtype: i32,
        /// The user to whom the comment was assigned.
        #[prost(message, optional, tag="7")]
        pub assigned_user: ::core::option::Option<super::User>,
    }
    /// Nested message and enum types in `Assignment`.
    pub mod assignment {
        /// More detailed information about the change.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Subtype {
            /// Subtype not available.
            Unspecified = 0,
            /// An assignment was added.
            Added = 1,
            /// An assignment was deleted.
            Deleted = 2,
            /// An assignment reply was added.
            ReplyAdded = 3,
            /// An assignment reply was deleted.
            ReplyDeleted = 4,
            /// An assignment was resolved.
            Resolved = 5,
            /// A resolved assignment was reopened.
            Reopened = 6,
            /// An assignment was reassigned.
            Reassigned = 7,
        }
        impl Subtype {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Subtype::Unspecified => "SUBTYPE_UNSPECIFIED",
                    Subtype::Added => "ADDED",
                    Subtype::Deleted => "DELETED",
                    Subtype::ReplyAdded => "REPLY_ADDED",
                    Subtype::ReplyDeleted => "REPLY_DELETED",
                    Subtype::Resolved => "RESOLVED",
                    Subtype::Reopened => "REOPENED",
                    Subtype::Reassigned => "REASSIGNED",
                }
            }
        }
    }
    /// A suggestion.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Suggestion {
        /// The sub-type of this event.
        #[prost(enumeration="suggestion::Subtype", tag="1")]
        pub subtype: i32,
    }
    /// Nested message and enum types in `Suggestion`.
    pub mod suggestion {
        /// More detailed information about the change.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Subtype {
            /// Subtype not available.
            Unspecified = 0,
            /// A suggestion was added.
            Added = 1,
            /// A suggestion was deleted.
            Deleted = 2,
            /// A suggestion reply was added.
            ReplyAdded = 3,
            /// A suggestion reply was deleted.
            ReplyDeleted = 4,
            /// A suggestion was accepted.
            Accepted = 7,
            /// A suggestion was rejected.
            Rejected = 8,
            /// An accepted suggestion was deleted.
            AcceptDeleted = 9,
            /// A rejected suggestion was deleted.
            RejectDeleted = 10,
        }
        impl Subtype {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Subtype::Unspecified => "SUBTYPE_UNSPECIFIED",
                    Subtype::Added => "ADDED",
                    Subtype::Deleted => "DELETED",
                    Subtype::ReplyAdded => "REPLY_ADDED",
                    Subtype::ReplyDeleted => "REPLY_DELETED",
                    Subtype::Accepted => "ACCEPTED",
                    Subtype::Rejected => "REJECTED",
                    Subtype::AcceptDeleted => "ACCEPT_DELETED",
                    Subtype::RejectDeleted => "REJECT_DELETED",
                }
            }
        }
    }
    /// The type of changed comment.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// A change on a regular posted comment.
        #[prost(message, tag="1")]
        Post(Post),
        /// A change on an assignment.
        #[prost(message, tag="2")]
        Assignment(Assignment),
        /// A change on a suggestion.
        #[prost(message, tag="3")]
        Suggestion(Suggestion),
    }
}
/// A change in the object's data leak prevention status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataLeakPreventionChange {
    /// The type of Data Leak Prevention (DLP) change.
    #[prost(enumeration="data_leak_prevention_change::Type", tag="1")]
    pub r#type: i32,
}
/// Nested message and enum types in `DataLeakPreventionChange`.
pub mod data_leak_prevention_change {
    /// The type of the change.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// An update to the DLP state that is neither FLAGGED or CLEARED.
        Unspecified = 0,
        /// Document has been flagged as containing sensitive content.
        Flagged = 1,
        /// Document is no longer flagged as containing sensitive content.
        Cleared = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Flagged => "FLAGGED",
                Type::Cleared => "CLEARED",
            }
        }
    }
}
/// Activity in applications other than Drive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationReference {
    /// The reference type corresponding to this event.
    #[prost(enumeration="application_reference::Type", tag="1")]
    pub r#type: i32,
}
/// Nested message and enum types in `ApplicationReference`.
pub mod application_reference {
    /// The type of the action.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// The type is not available.
        UnspecifiedReferenceType = 0,
        /// The links of one or more Drive items were posted.
        Link = 1,
        /// Comments were made regarding a Drive item.
        Discuss = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::UnspecifiedReferenceType => "UNSPECIFIED_REFERENCE_TYPE",
                Type::Link => "LINK",
                Type::Discuss => "DISCUSS",
            }
        }
    }
}
/// Information about settings changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingsChange {
    /// The set of changes made to restrictions.
    #[prost(message, repeated, tag="1")]
    pub restriction_changes: ::prost::alloc::vec::Vec<settings_change::RestrictionChange>,
}
/// Nested message and enum types in `SettingsChange`.
pub mod settings_change {
    /// Information about restriction policy changes to a feature.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RestrictionChange {
        /// The feature which had a change in restriction policy.
        #[prost(enumeration="restriction_change::Feature", tag="1")]
        pub feature: i32,
        /// The restriction in place after the change.
        #[prost(enumeration="restriction_change::Restriction", tag="2")]
        pub new_restriction: i32,
    }
    /// Nested message and enum types in `RestrictionChange`.
    pub mod restriction_change {
        /// The feature which had changes to its restriction policy.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Feature {
            /// The feature which changed restriction settings was not available.
            Unspecified = 0,
            /// When restricted, this prevents items from being shared outside the
            /// domain.
            SharingOutsideDomain = 1,
            /// When restricted, this prevents direct sharing of individual items.
            DirectSharing = 2,
            /// When restricted, this prevents actions like copy, download, and print
            /// that might result in uncontrolled duplicates of items.
            ItemDuplication = 3,
            /// When restricted, this prevents use of Drive File Stream.
            DriveFileStream = 4,
        }
        impl Feature {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Feature::Unspecified => "FEATURE_UNSPECIFIED",
                    Feature::SharingOutsideDomain => "SHARING_OUTSIDE_DOMAIN",
                    Feature::DirectSharing => "DIRECT_SHARING",
                    Feature::ItemDuplication => "ITEM_DUPLICATION",
                    Feature::DriveFileStream => "DRIVE_FILE_STREAM",
                }
            }
        }
        /// The restriction applicable to a feature.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Restriction {
            /// The type of restriction is not available.
            Unspecified = 0,
            /// The feature is available without restriction.
            Unrestricted = 1,
            /// The use of this feature is fully restricted.
            FullyRestricted = 2,
        }
        impl Restriction {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Restriction::Unspecified => "RESTRICTION_UNSPECIFIED",
                    Restriction::Unrestricted => "UNRESTRICTED",
                    Restriction::FullyRestricted => "FULLY_RESTRICTED",
                }
            }
        }
    }
}
/// The request message for querying Drive activity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDriveActivityRequest {
    /// Details on how to consolidate related actions that make up the activity. If
    /// not set, then related actions are not consolidated.
    #[prost(message, optional, tag="5")]
    pub consolidation_strategy: ::core::option::Option<ConsolidationStrategy>,
    /// The miminum number of activities desired in the response; the server will
    /// attempt to return at least this quanitity. The server may also return fewer
    /// activities if it has a partial response ready before the request times out.
    /// If not set, a default value is used.
    #[prost(int32, tag="6")]
    pub page_size: i32,
    /// The token identifying which page of results to return. Set this to the
    /// next_page_token value returned from a previous query to obtain the
    /// following page of results. If not set, the first page of results will be
    /// returned.
    #[prost(string, tag="7")]
    pub page_token: ::prost::alloc::string::String,
    /// The filtering for items returned from this query request. The format of the
    /// filter string is a sequence of expressions, joined by an optional "AND",
    /// where each expression is of the form "field operator value".
    ///
    /// Supported fields:
    ///
    ///    - `time`: Uses numerical operators on date values either in
    ///      terms of milliseconds since Jan 1, 1970 or in RFC 3339 format.
    ///      Examples:
    ///        - `time > 1452409200000 AND time <= 1492812924310`
    ///        - `time >= "2016-01-10T01:02:03-05:00"`
    ///
    ///    - `detail.action_detail_case`: Uses the "has" operator (:) and
    ///      either a singular value or a list of allowed action types enclosed in
    ///      parentheses.
    ///      Examples:
    ///        - `detail.action_detail_case: RENAME`
    ///        - `detail.action_detail_case:(CREATE EDIT)`
    ///        - `-detail.action_detail_case:MOVE`
    #[prost(string, tag="8")]
    pub filter: ::prost::alloc::string::String,
    /// The primary criteria in the query. The default is
    /// ancestor_name = `items/root` if no key is specified.
    #[prost(oneof="query_drive_activity_request::Key", tags="1, 2")]
    pub key: ::core::option::Option<query_drive_activity_request::Key>,
}
/// Nested message and enum types in `QueryDriveActivityRequest`.
pub mod query_drive_activity_request {
    /// The primary criteria in the query. The default is
    /// ancestor_name = `items/root` if no key is specified.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Key {
        /// Return activities for this Drive item. The format is
        /// `items/ITEM_ID`.
        #[prost(string, tag="1")]
        ItemName(::prost::alloc::string::String),
        /// Return activities for this Drive folder and all children and descendants.
        /// The format is `items/ITEM_ID`.
        #[prost(string, tag="2")]
        AncestorName(::prost::alloc::string::String),
    }
}
/// How the individual activities are consolidated. A set of activities may be
/// consolidated into one combined activity if they are related in some way, such
/// as one actor performing the same action on multiple targets, or multiple
/// actors performing the same action on a single target. The strategy defines
/// the rules for which activities are related.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsolidationStrategy {
    /// How the individual activities are consolidated.
    #[prost(oneof="consolidation_strategy::Strategy", tags="1, 2")]
    pub strategy: ::core::option::Option<consolidation_strategy::Strategy>,
}
/// Nested message and enum types in `ConsolidationStrategy`.
pub mod consolidation_strategy {
    /// A strategy which does no consolidation of individual activities.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoConsolidation {
    }
    /// A strategy which consolidates activities using the grouping rules from the
    /// legacy V1 Activity API. Similar actions occurring within a window of time
    /// can be grouped across multiple targets (such as moving a set of files at
    /// once) or multiple actors (such as several users editing the same item).
    /// Grouping rules for this strategy are specific to each type of action.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Legacy {
    }
    /// How the individual activities are consolidated.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Strategy {
        /// The individual activities are not consolidated.
        #[prost(message, tag="1")]
        None(NoConsolidation),
        /// The individual activities are consolidated using the legacy strategy.
        #[prost(message, tag="2")]
        Legacy(Legacy),
    }
}
/// Response message for querying Drive activity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDriveActivityResponse {
    /// List of activity requested.
    #[prost(message, repeated, tag="1")]
    pub activities: ::prost::alloc::vec::Vec<DriveActivity>,
    /// Token to retrieve the next page of results, or
    /// empty if there are no more results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A single Drive activity comprising one or more Actions by one or more
/// Actors on one or more Targets. Some Action groupings occur spontaneously,
/// such as moving an item into a shared folder triggering a permission change.
/// Other groupings of related Actions, such as multiple Actors editing one item
/// or moving multiple files into a new folder, are controlled by the selection
/// of a ConsolidationStrategy in the QueryDriveActivityRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriveActivity {
    /// Key information about the primary action for this activity. This is either
    /// representative, or the most important, of all actions in the activity,
    /// according to the ConsolidationStrategy in the request.
    #[prost(message, optional, tag="2")]
    pub primary_action_detail: ::core::option::Option<ActionDetail>,
    /// All actor(s) responsible for the activity.
    #[prost(message, repeated, tag="3")]
    pub actors: ::prost::alloc::vec::Vec<Actor>,
    /// Details on all actions in this activity.
    #[prost(message, repeated, tag="4")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
    /// All Google Drive objects this activity is about (e.g. file, folder, drive).
    /// This represents the state of the target immediately after the actions
    /// occurred.
    #[prost(message, repeated, tag="5")]
    pub targets: ::prost::alloc::vec::Vec<Target>,
    /// The period of time when this activity occurred.
    #[prost(oneof="drive_activity::Time", tags="6, 7")]
    pub time: ::core::option::Option<drive_activity::Time>,
}
/// Nested message and enum types in `DriveActivity`.
pub mod drive_activity {
    /// The period of time when this activity occurred.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Time {
        /// The activity occurred at this specific time.
        #[prost(message, tag="6")]
        Timestamp(::prost_types::Timestamp),
        /// The activity occurred over this time range.
        #[prost(message, tag="7")]
        TimeRange(super::TimeRange),
    }
}
/// Generated client implementations.
pub mod drive_activity_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for querying activity on Drive items. Activity is user
    /// or system action on Drive items that happened in the past. A Drive item can
    /// be a file or folder, or a Team Drive.
    #[derive(Debug, Clone)]
    pub struct DriveActivityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DriveActivityServiceClient<tonic::transport::Channel> {
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
    impl<T> DriveActivityServiceClient<T>
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
        ) -> DriveActivityServiceClient<InterceptedService<T, F>>
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
            DriveActivityServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Query past activity in Google Drive.
        pub async fn query_drive_activity(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDriveActivityRequest>,
        ) -> Result<tonic::Response<super::QueryDriveActivityResponse>, tonic::Status> {
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
                "/google.apps.drive.activity.v2.DriveActivityService/QueryDriveActivity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
