///
/// Messages
///
/// Use global external addresses for GFE-based external HTTP(S) load balancers in Premium Tier.
///
/// Use global internal addresses for reserved peering network range.
///
/// Use regional external addresses for the following resources:
///
/// - External IP addresses for VM instances - Regional external forwarding rules - Cloud NAT external IP addresses - GFE based LBs in Standard Tier - Network LBs in Premium or Standard Tier - Cloud VPN gateways (both Classic and HA)
///
/// Use regional internal IP addresses for subnet IP ranges (primary and secondary). This includes:
///
/// - Internal IP addresses for VM instances - Alias IP ranges of VM instances (/32 only) - Regional internal forwarding rules - Internal TCP/UDP load balancer addresses - Internal HTTP(S) load balancer addresses - Cloud DNS inbound forwarding IP addresses
///
/// For more information, read reserved IP address.
///
/// (== resource_for {$api_version}.addresses ==) (== resource_for {$api_version}.globalAddresses ==)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    /// The static IP address represented by this resource.
    #[prost(string, optional, tag="462920692")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    /// The type of address to reserve, either INTERNAL or EXTERNAL. If unspecified, defaults to EXTERNAL.
    /// Check the AddressType enum for the list of possible values.
    #[prost(string, optional, tag="264307877")]
    pub address_type: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[prost(string, optional, tag="30525366")]
    pub creation_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    /// An optional description of this resource. Provide this field when you create the resource.
    #[prost(string, optional, tag="422937596")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[prost(uint64, optional, tag="3355")]
    pub id: ::core::option::Option<u64>,
    /// The IP version that will be used by this address. Valid options are IPV4 or IPV6. This can only be specified for a global address.
    /// Check the IpVersion enum for the list of possible values.
    #[prost(string, optional, tag="294959552")]
    pub ip_version: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Type of the resource. Always compute#address for addresses.
    #[prost(string, optional, tag="3292052")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `\[a-z]([-a-z0-9]*[a-z0-9\])?`. The first character must be a lowercase letter, and all following characters (except for the last character) must be a dash, lowercase letter, or digit. The last character must be a lowercase letter or digit.
    #[prost(string, optional, tag="3373707")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The URL of the network in which to reserve the address. This field can only be used with INTERNAL type with the VPC_PEERING purpose.
    #[prost(string, optional, tag="232872494")]
    pub network: ::core::option::Option<::prost::alloc::string::String>,
    /// This signifies the networking tier used for configuring this address and can only take the following values: PREMIUM or STANDARD. Global forwarding rules can only be Premium Tier. Regional forwarding rules can be either Premium or Standard Tier. Standard Tier addresses applied to regional forwarding rules can be used with any external load balancer. Regional forwarding rules in Premium Tier can only be used with a network load balancer.
    ///
    /// If this field is not specified, it is assumed to be PREMIUM.
    /// Check the NetworkTier enum for the list of possible values.
    #[prost(string, optional, tag="517397843")]
    pub network_tier: ::core::option::Option<::prost::alloc::string::String>,
    /// The prefix length if the resource reprensents an IP range.
    #[prost(int32, optional, tag="453565747")]
    pub prefix_length: ::core::option::Option<i32>,
    /// The purpose of this resource, which can be one of the following values:
    /// - `GCE_ENDPOINT` for addresses that are used by VM instances, alias IP ranges, internal load balancers, and similar resources.
    /// - `DNS_RESOLVER` for a DNS resolver address in a subnetwork
    /// - `VPC_PEERING` for addresses that are reserved for VPC peer networks.
    /// - `NAT_AUTO` for addresses that are external IP addresses automatically reserved for Cloud NAT.
    /// Check the Purpose enum for the list of possible values.
    #[prost(string, optional, tag="316407070")]
    pub purpose: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] The URL of the region where the regional address resides. This field is not applicable to global addresses. You must specify this field as part of the HTTP request URL.
    #[prost(string, optional, tag="138946292")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Server-defined URL for the resource.
    #[prost(string, optional, tag="456214797")]
    pub self_link: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] The status of the address, which can be one of RESERVING, RESERVED, or IN_USE. An address that is RESERVING is currently in the process of being reserved. A RESERVED address is currently reserved and available to use. An IN_USE address is currently being used by another resource and is not available.
    /// Check the Status enum for the list of possible values.
    #[prost(string, optional, tag="181260274")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// The URL of the subnetwork in which to reserve the address. If an IP address is specified, it must be within the subnetwork's IP range. This field can only be used with INTERNAL type with a GCE_ENDPOINT or DNS_RESOLVER purpose.
    #[prost(string, optional, tag="307827694")]
    pub subnetwork: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] The URLs of the resources that are using this address.
    #[prost(string, repeated, tag="111578632")]
    pub users: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Address`.
pub mod address {
    /// The type of address to reserve, either INTERNAL or EXTERNAL. If unspecified, defaults to EXTERNAL.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AddressType {
        /// A value indicating that the enum field is not set.
        UndefinedAddressType = 0,
        External = 35607499,
        Internal = 279295677,
        UnspecifiedType = 53933922,
    }
    impl AddressType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AddressType::UndefinedAddressType => "UNDEFINED_ADDRESS_TYPE",
                AddressType::External => "EXTERNAL",
                AddressType::Internal => "INTERNAL",
                AddressType::UnspecifiedType => "UNSPECIFIED_TYPE",
            }
        }
    }
    /// The IP version that will be used by this address. Valid options are IPV4 or IPV6. This can only be specified for a global address.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IpVersion {
        /// A value indicating that the enum field is not set.
        UndefinedIpVersion = 0,
        Ipv4 = 2254341,
        Ipv6 = 2254343,
        UnspecifiedVersion = 21850000,
    }
    impl IpVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IpVersion::UndefinedIpVersion => "UNDEFINED_IP_VERSION",
                IpVersion::Ipv4 => "IPV4",
                IpVersion::Ipv6 => "IPV6",
                IpVersion::UnspecifiedVersion => "UNSPECIFIED_VERSION",
            }
        }
    }
    /// This signifies the networking tier used for configuring this address and can only take the following values: PREMIUM or STANDARD. Global forwarding rules can only be Premium Tier. Regional forwarding rules can be either Premium or Standard Tier. Standard Tier addresses applied to regional forwarding rules can be used with any external load balancer. Regional forwarding rules in Premium Tier can only be used with a network load balancer.
    ///
    /// If this field is not specified, it is assumed to be PREMIUM.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NetworkTier {
        /// A value indicating that the enum field is not set.
        UndefinedNetworkTier = 0,
        Premium = 399530551,
        Standard = 484642493,
    }
    impl NetworkTier {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NetworkTier::UndefinedNetworkTier => "UNDEFINED_NETWORK_TIER",
                NetworkTier::Premium => "PREMIUM",
                NetworkTier::Standard => "STANDARD",
            }
        }
    }
    /// The purpose of this resource, which can be one of the following values:
    /// - `GCE_ENDPOINT` for addresses that are used by VM instances, alias IP ranges, internal load balancers, and similar resources.
    /// - `DNS_RESOLVER` for a DNS resolver address in a subnetwork
    /// - `VPC_PEERING` for addresses that are reserved for VPC peer networks.
    /// - `NAT_AUTO` for addresses that are external IP addresses automatically reserved for Cloud NAT.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Purpose {
        /// A value indicating that the enum field is not set.
        UndefinedPurpose = 0,
        DnsResolver = 476114556,
        GceEndpoint = 230515243,
        NatAuto = 163666477,
        VpcPeering = 400800170,
    }
    impl Purpose {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Purpose::UndefinedPurpose => "UNDEFINED_PURPOSE",
                Purpose::DnsResolver => "DNS_RESOLVER",
                Purpose::GceEndpoint => "GCE_ENDPOINT",
                Purpose::NatAuto => "NAT_AUTO",
                Purpose::VpcPeering => "VPC_PEERING",
            }
        }
    }
    /// [Output Only] The status of the address, which can be one of RESERVING, RESERVED, or IN_USE. An address that is RESERVING is currently in the process of being reserved. A RESERVED address is currently reserved and available to use. An IN_USE address is currently being used by another resource and is not available.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// A value indicating that the enum field is not set.
        UndefinedStatus = 0,
        InUse = 17393485,
        Reserved = 432241448,
        Reserving = 514587225,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::UndefinedStatus => "UNDEFINED_STATUS",
                Status::InUse => "IN_USE",
                Status::Reserved => "RESERVED",
                Status::Reserving => "RESERVING",
            }
        }
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressAggregatedList {
    /// [Output Only] Unique identifier for the resource; defined by the server.
    #[prost(string, optional, tag="3355")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of AddressesScopedList resources.
    #[prost(map="string, message", tag="100526016")]
    pub items: ::std::collections::HashMap<::prost::alloc::string::String, AddressesScopedList>,
    /// [Output Only] Type of resource. Always compute#addressAggregatedList for aggregated lists of addresses.
    #[prost(string, optional, tag="3292052")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] This token allows you to get the next page of results for list requests. If the number of results is larger than maxResults, use the nextPageToken as a value for the query parameter pageToken in the next list request. Subsequent list requests will have their own nextPageToken to continue paging through the results.
    #[prost(string, optional, tag="79797525")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Server-defined URL for this resource.
    #[prost(string, optional, tag="456214797")]
    pub self_link: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Informational warning message.
    #[prost(message, optional, tag="50704284")]
    pub warning: ::core::option::Option<Warning>,
}
/// Contains a list of addresses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressList {
    /// [Output Only] Unique identifier for the resource; defined by the server.
    #[prost(string, optional, tag="3355")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of Address resources.
    #[prost(message, repeated, tag="100526016")]
    pub items: ::prost::alloc::vec::Vec<Address>,
    /// [Output Only] Type of resource. Always compute#addressList for lists of addresses.
    #[prost(string, optional, tag="3292052")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] This token allows you to get the next page of results for list requests. If the number of results is larger than maxResults, use the nextPageToken as a value for the query parameter pageToken in the next list request. Subsequent list requests will have their own nextPageToken to continue paging through the results.
    #[prost(string, optional, tag="79797525")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Server-defined URL for this resource.
    #[prost(string, optional, tag="456214797")]
    pub self_link: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Informational warning message.
    #[prost(message, optional, tag="50704284")]
    pub warning: ::core::option::Option<Warning>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressesScopedList {
    /// [Output Only] A list of addresses contained in this scope.
    #[prost(message, repeated, tag="337673122")]
    pub addresses: ::prost::alloc::vec::Vec<Address>,
    /// [Output Only] Informational warning which replaces the list of addresses when the list is empty.
    #[prost(message, optional, tag="50704284")]
    pub warning: ::core::option::Option<Warning>,
}
/// A request message for Addresses.AggregatedList. See the method description for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedListAddressesRequest {
    /// A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either `=`, `!=`, `>`, or `<`.
    ///
    /// For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`.
    ///
    /// You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels.
    ///
    /// To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = "Intel Skylake") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = "Intel Skylake") OR (cpuPlatform = "Intel Broadwell") AND (scheduling.automaticRestart = true) ```
    #[prost(string, optional, tag="336120696")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates whether every visible scope for each scope type (zone, region, global) should be included in the response. For new resource types added after this field, the flag has no effect as new resource types will always include every visible scope for each scope type in response. For resource types which predate this field, if this flag is omitted or false, only scopes of the scope types where the resource type is expected to be found will be included.
    #[prost(bool, optional, tag="391327988")]
    pub include_all_scopes: ::core::option::Option<bool>,
    /// The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)
    #[prost(uint32, optional, tag="54715419")]
    pub max_results: ::core::option::Option<u32>,
    /// Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name.
    ///
    /// You can also sort results in descending order based on the creation timestamp using `orderBy="creationTimestamp desc"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first.
    ///
    /// Currently, only sorting by `name` or `creationTimestamp desc` is supported.
    #[prost(string, optional, tag="160562920")]
    pub order_by: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results.
    #[prost(string, optional, tag="19994697")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// Project ID for this request.
    #[prost(string, tag="227560217")]
    pub project: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// [Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding).
    #[prost(string, optional, tag="106079")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] A warning data value corresponding to the key.
    #[prost(string, optional, tag="111972721")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// A request message for Addresses.Delete. See the method description for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAddressRequest {
    /// Name of the address resource to delete.
    #[prost(string, tag="462920692")]
    pub address: ::prost::alloc::string::String,
    /// Project ID for this request.
    #[prost(string, tag="227560217")]
    pub project: ::prost::alloc::string::String,
    /// Name of the region for this request.
    #[prost(string, tag="138946292")]
    pub region: ::prost::alloc::string::String,
    /// An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, optional, tag="37109963")]
    pub request_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// [Output Only] If errors are generated during processing of the operation, this field will be populated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// [Output Only] The array of errors encountered while processing this operation.
    #[prost(message, repeated, tag="315977579")]
    pub errors: ::prost::alloc::vec::Vec<Errors>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Errors {
    /// [Output Only] The error type identifier for this error.
    #[prost(string, optional, tag="3059181")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Indicates the field in the request that caused the error. This property is optional.
    #[prost(string, optional, tag="290430901")]
    pub location: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] An optional, human-readable error message.
    #[prost(string, optional, tag="418054151")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// A request message for RegionOperations.Get. See the method description for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegionOperationRequest {
    /// Name of the Operations resource to return.
    #[prost(string, tag="52090215")]
    pub operation: ::prost::alloc::string::String,
    /// Project ID for this request.
    #[prost(string, tag="227560217")]
    pub project: ::prost::alloc::string::String,
    /// Name of the region for this request.
    #[prost(string, tag="138946292")]
    pub region: ::prost::alloc::string::String,
}
/// A request message for Addresses.Insert. See the method description for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertAddressRequest {
    /// The body resource for this request
    #[prost(message, optional, tag="483888121")]
    pub address_resource: ::core::option::Option<Address>,
    /// Project ID for this request.
    #[prost(string, tag="227560217")]
    pub project: ::prost::alloc::string::String,
    /// Name of the region for this request.
    #[prost(string, tag="138946292")]
    pub region: ::prost::alloc::string::String,
    /// An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, optional, tag="37109963")]
    pub request_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A request message for Addresses.List. See the method description for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAddressesRequest {
    /// A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either =, !=, >, or <.
    ///
    /// For example, if you are filtering Compute Engine instances, you can exclude instances named example-instance by specifying name != example-instance.
    ///
    /// You can also filter nested fields. For example, you could specify scheduling.automaticRestart = false to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels.
    ///
    /// To filter on multiple expressions, provide each separate expression within parentheses. For example, (scheduling.automaticRestart = true) (cpuPlatform = "Intel Skylake"). By default, each expression is an AND expression. However, you can include AND and OR expressions explicitly. For example, (cpuPlatform = "Intel Skylake") OR (cpuPlatform = "Intel Broadwell") AND (scheduling.automaticRestart = true).
    #[prost(string, optional, tag="336120696")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
    /// The maximum number of results per page that should be returned. If the number of available results is larger than maxResults, Compute Engine returns a nextPageToken that can be used to get the next page of results in subsequent list requests. Acceptable values are 0 to 500, inclusive. (Default: 500)
    #[prost(uint32, optional, tag="54715419")]
    pub max_results: ::core::option::Option<u32>,
    /// Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name.
    ///
    /// You can also sort results in descending order based on the creation timestamp using orderBy="creationTimestamp desc". This sorts results based on the creationTimestamp field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first.
    ///
    /// Currently, only sorting by name or creationTimestamp desc is supported.
    #[prost(string, tag="160562920")]
    pub order_by: ::prost::alloc::string::String,
    /// Specifies a page token to use. Set pageToken to the nextPageToken returned by a previous list request to get the next page of results.
    #[prost(string, optional, tag="19994697")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// Project ID for this request.
    #[prost(string, tag="227560217")]
    pub project: ::prost::alloc::string::String,
    /// Name of the region for this request.
    #[prost(string, tag="138946292")]
    pub region: ::prost::alloc::string::String,
}
/// Represents an Operation resource.
///
/// Google Compute Engine has three Operation resources:
///
/// * \[Global\](/compute/docs/reference/rest/{$api_version}/globalOperations) * \[Regional\](/compute/docs/reference/rest/{$api_version}/regionOperations) * \[Zonal\](/compute/docs/reference/rest/{$api_version}/zoneOperations)
///
/// You can use an operation resource to manage asynchronous API requests. For more information, read Handling API responses.
///
/// Operations can be global, regional or zonal.
/// - For global operations, use the globalOperations resource.
/// - For regional operations, use the regionOperations resource.
/// - For zonal operations, use the zoneOperations resource.
///
/// For more information, read  Global, Regional, and Zonal Resources. (== resource_for {$api_version}.globalOperations ==) (== resource_for {$api_version}.regionOperations ==) (== resource_for {$api_version}.zoneOperations ==)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// [Output Only] The value of `requestId` if you provided it in the request. Not present otherwise.
    #[prost(string, optional, tag="297240295")]
    pub client_operation_id: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Deprecated\] This field is deprecated.
    #[prost(string, optional, tag="30525366")]
    pub creation_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] A textual description of the operation, which is set when the operation is created.
    #[prost(string, optional, tag="422937596")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] The time that this operation was completed. This value is in RFC3339 text format.
    #[prost(string, optional, tag="114938801")]
    pub end_time: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] If errors are generated during processing of the operation, this field will be populated.
    #[prost(message, optional, tag="96784904")]
    pub error: ::core::option::Option<Error>,
    /// [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as NOT FOUND.
    #[prost(string, optional, tag="202521945")]
    pub http_error_message: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a 404 means the resource was not found.
    #[prost(int32, optional, tag="312345196")]
    pub http_error_status_code: ::core::option::Option<i32>,
    /// [Output Only] The unique identifier for the operation. This identifier is defined by the server.
    #[prost(uint64, optional, tag="3355")]
    pub id: ::core::option::Option<u64>,
    /// [Output Only] The time that this operation was requested. This value is in RFC3339 text format.
    #[prost(string, optional, tag="433722515")]
    pub insert_time: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Type of the resource. Always compute#operation for Operation resources.
    #[prost(string, optional, tag="3292052")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Name of the operation.
    #[prost(string, optional, tag="3373707")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] The type of operation, such as insert, update, or delete, and so on.
    #[prost(string, optional, tag="177650450")]
    pub operation_type: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses.
    #[prost(int32, optional, tag="72663597")]
    pub progress: ::core::option::Option<i32>,
    /// [Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations.
    #[prost(string, optional, tag="138946292")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Server-defined URL for the resource.
    #[prost(string, optional, tag="456214797")]
    pub self_link: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format.
    #[prost(string, optional, tag="37467274")]
    pub start_time: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] The status of the operation, which can be one of the following: PENDING, RUNNING, or DONE.
    #[prost(enumeration="operation::Status", optional, tag="181260274")]
    pub status: ::core::option::Option<i32>,
    /// [Output Only] An optional textual description of the current status of the operation.
    #[prost(string, optional, tag="297428154")]
    pub status_message: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] The unique target ID, which identifies a specific incarnation of the target resource.
    #[prost(uint64, optional, tag="258165385")]
    pub target_id: ::core::option::Option<u64>,
    /// [Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the persistent disk that the snapshot was created from.
    #[prost(string, optional, tag="62671336")]
    pub target_link: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] User who requested the operation, for example: user@example.com.
    #[prost(string, optional, tag="3599307")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] If warning messages are generated during processing of the operation, this field will be populated.
    #[prost(message, repeated, tag="498091095")]
    pub warnings: ::prost::alloc::vec::Vec<Warnings>,
    /// [Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations.
    #[prost(string, optional, tag="3744684")]
    pub zone: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    /// [Output Only] The status of the operation, which can be one of the following: PENDING, RUNNING, or DONE.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// A value indicating that the enum field is not set.
        UndefinedStatus = 0,
        Done = 2104194,
        Pending = 35394935,
        Running = 121282975,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::UndefinedStatus => "UNDEFINED_STATUS",
                Status::Done => "DONE",
                Status::Pending => "PENDING",
                Status::Running => "RUNNING",
            }
        }
    }
}
/// A request message for RegionOperations.Wait. See the method description for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitRegionOperationRequest {
    /// Name of the Operations resource to return.
    #[prost(string, tag="52090215")]
    pub operation: ::prost::alloc::string::String,
    /// Project ID for this request.
    #[prost(string, tag="227560217")]
    pub project: ::prost::alloc::string::String,
    /// Name of the region for this request.
    #[prost(string, tag="138946292")]
    pub region: ::prost::alloc::string::String,
}
/// [Output Only] Informational warning message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Warning {
    /// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
    /// Check the Code enum for the list of possible values.
    #[prost(string, optional, tag="3059181")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Metadata about this warning in key: value format. For example:
    /// "data": [ { "key": "scope", "value": "zones/us-east1-d" }
    #[prost(message, repeated, tag="3076010")]
    pub data: ::prost::alloc::vec::Vec<Data>,
    /// [Output Only] A human-readable description of the warning code.
    #[prost(string, optional, tag="418054151")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Warning`.
pub mod warning {
    /// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// A value indicating that the enum field is not set.
        UndefinedCode = 0,
        CleanupFailed = 150308440,
        DeprecatedResourceUsed = 391835586,
        DeprecatedTypeUsed = 346526230,
        DiskSizeLargerThanImageSize = 369442967,
        ExperimentalTypeUsed = 451954443,
        ExternalApiWarning = 175546307,
        FieldValueOverriden = 329669423,
        InjectedKernelsDeprecated = 417377419,
        MissingTypeDependency = 344505463,
        NextHopAddressNotAssigned = 324964999,
        NextHopCannotIpForward = 383382887,
        NextHopInstanceNotFound = 464250446,
        NextHopInstanceNotOnNetwork = 243758146,
        NextHopNotRunning = 417081265,
        NotCriticalError = 105763924,
        NoResultsOnPage = 30036744,
        RequiredTosAgreement = 3745539,
        ResourceInUseByOtherResourceWarning = 496728641,
        ResourceNotDeleted = 168598460,
        SchemaValidationIgnored = 275245642,
        SingleInstancePropertyTemplate = 268305617,
        UndeclaredProperties = 390513439,
        Unreachable = 13328052,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::UndefinedCode => "UNDEFINED_CODE",
                Code::CleanupFailed => "CLEANUP_FAILED",
                Code::DeprecatedResourceUsed => "DEPRECATED_RESOURCE_USED",
                Code::DeprecatedTypeUsed => "DEPRECATED_TYPE_USED",
                Code::DiskSizeLargerThanImageSize => "DISK_SIZE_LARGER_THAN_IMAGE_SIZE",
                Code::ExperimentalTypeUsed => "EXPERIMENTAL_TYPE_USED",
                Code::ExternalApiWarning => "EXTERNAL_API_WARNING",
                Code::FieldValueOverriden => "FIELD_VALUE_OVERRIDEN",
                Code::InjectedKernelsDeprecated => "INJECTED_KERNELS_DEPRECATED",
                Code::MissingTypeDependency => "MISSING_TYPE_DEPENDENCY",
                Code::NextHopAddressNotAssigned => "NEXT_HOP_ADDRESS_NOT_ASSIGNED",
                Code::NextHopCannotIpForward => "NEXT_HOP_CANNOT_IP_FORWARD",
                Code::NextHopInstanceNotFound => "NEXT_HOP_INSTANCE_NOT_FOUND",
                Code::NextHopInstanceNotOnNetwork => "NEXT_HOP_INSTANCE_NOT_ON_NETWORK",
                Code::NextHopNotRunning => "NEXT_HOP_NOT_RUNNING",
                Code::NotCriticalError => "NOT_CRITICAL_ERROR",
                Code::NoResultsOnPage => "NO_RESULTS_ON_PAGE",
                Code::RequiredTosAgreement => "REQUIRED_TOS_AGREEMENT",
                Code::ResourceInUseByOtherResourceWarning => "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING",
                Code::ResourceNotDeleted => "RESOURCE_NOT_DELETED",
                Code::SchemaValidationIgnored => "SCHEMA_VALIDATION_IGNORED",
                Code::SingleInstancePropertyTemplate => "SINGLE_INSTANCE_PROPERTY_TEMPLATE",
                Code::UndeclaredProperties => "UNDECLARED_PROPERTIES",
                Code::Unreachable => "UNREACHABLE",
            }
        }
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Warnings {
    /// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
    /// Check the Code enum for the list of possible values.
    #[prost(string, optional, tag="3059181")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    /// [Output Only] Metadata about this warning in key: value format. For example:
    /// "data": [ { "key": "scope", "value": "zones/us-east1-d" }
    #[prost(message, repeated, tag="3076010")]
    pub data: ::prost::alloc::vec::Vec<Data>,
    /// [Output Only] A human-readable description of the warning code.
    #[prost(string, optional, tag="418054151")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Warnings`.
pub mod warnings {
    /// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// A value indicating that the enum field is not set.
        UndefinedCode = 0,
        CleanupFailed = 150308440,
        DeprecatedResourceUsed = 391835586,
        DeprecatedTypeUsed = 346526230,
        DiskSizeLargerThanImageSize = 369442967,
        ExperimentalTypeUsed = 451954443,
        ExternalApiWarning = 175546307,
        FieldValueOverriden = 329669423,
        InjectedKernelsDeprecated = 417377419,
        MissingTypeDependency = 344505463,
        NextHopAddressNotAssigned = 324964999,
        NextHopCannotIpForward = 383382887,
        NextHopInstanceNotFound = 464250446,
        NextHopInstanceNotOnNetwork = 243758146,
        NextHopNotRunning = 417081265,
        NotCriticalError = 105763924,
        NoResultsOnPage = 30036744,
        RequiredTosAgreement = 3745539,
        ResourceInUseByOtherResourceWarning = 496728641,
        ResourceNotDeleted = 168598460,
        SchemaValidationIgnored = 275245642,
        SingleInstancePropertyTemplate = 268305617,
        UndeclaredProperties = 390513439,
        Unreachable = 13328052,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::UndefinedCode => "UNDEFINED_CODE",
                Code::CleanupFailed => "CLEANUP_FAILED",
                Code::DeprecatedResourceUsed => "DEPRECATED_RESOURCE_USED",
                Code::DeprecatedTypeUsed => "DEPRECATED_TYPE_USED",
                Code::DiskSizeLargerThanImageSize => "DISK_SIZE_LARGER_THAN_IMAGE_SIZE",
                Code::ExperimentalTypeUsed => "EXPERIMENTAL_TYPE_USED",
                Code::ExternalApiWarning => "EXTERNAL_API_WARNING",
                Code::FieldValueOverriden => "FIELD_VALUE_OVERRIDEN",
                Code::InjectedKernelsDeprecated => "INJECTED_KERNELS_DEPRECATED",
                Code::MissingTypeDependency => "MISSING_TYPE_DEPENDENCY",
                Code::NextHopAddressNotAssigned => "NEXT_HOP_ADDRESS_NOT_ASSIGNED",
                Code::NextHopCannotIpForward => "NEXT_HOP_CANNOT_IP_FORWARD",
                Code::NextHopInstanceNotFound => "NEXT_HOP_INSTANCE_NOT_FOUND",
                Code::NextHopInstanceNotOnNetwork => "NEXT_HOP_INSTANCE_NOT_ON_NETWORK",
                Code::NextHopNotRunning => "NEXT_HOP_NOT_RUNNING",
                Code::NotCriticalError => "NOT_CRITICAL_ERROR",
                Code::NoResultsOnPage => "NO_RESULTS_ON_PAGE",
                Code::RequiredTosAgreement => "REQUIRED_TOS_AGREEMENT",
                Code::ResourceInUseByOtherResourceWarning => "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING",
                Code::ResourceNotDeleted => "RESOURCE_NOT_DELETED",
                Code::SchemaValidationIgnored => "SCHEMA_VALIDATION_IGNORED",
                Code::SingleInstancePropertyTemplate => "SINGLE_INSTANCE_PROPERTY_TEMPLATE",
                Code::UndeclaredProperties => "UNDECLARED_PROPERTIES",
                Code::Unreachable => "UNREACHABLE",
            }
        }
    }
}
/// Generated client implementations.
pub mod addresses_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    /// Services
    ///
    /// The Addresses API.
    #[derive(Debug, Clone)]
    pub struct AddressesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AddressesClient<tonic::transport::Channel> {
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
    impl<T> AddressesClient<T>
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
        ) -> AddressesClient<InterceptedService<T, F>>
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
            AddressesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieves an aggregated list of addresses.
        pub async fn aggregated_list(
            &mut self,
            request: impl tonic::IntoRequest<super::AggregatedListAddressesRequest>,
        ) -> Result<tonic::Response<super::AddressAggregatedList>, tonic::Status> {
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
                "/google.cloud.compute.v1small.Addresses/AggregatedList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the specified address resource.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAddressRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.compute.v1small.Addresses/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an address resource in the specified project by using the data included in the request.
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertAddressRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.compute.v1small.Addresses/Insert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves a list of addresses contained within the specified region.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAddressesRequest>,
        ) -> Result<tonic::Response<super::AddressList>, tonic::Status> {
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
                "/google.cloud.compute.v1small.Addresses/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod region_operations_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The RegionOperations API.
    #[derive(Debug, Clone)]
    pub struct RegionOperationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RegionOperationsClient<tonic::transport::Channel> {
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
    impl<T> RegionOperationsClient<T>
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
        ) -> RegionOperationsClient<InterceptedService<T, F>>
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
            RegionOperationsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieves the specified region-specific Operations resource.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRegionOperationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.compute.v1small.RegionOperations/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Waits for the specified Operation resource to return as `DONE` or for the request to approach the 2 minute deadline, and retrieves the specified Operation resource. This method differs from the `GET` method in that it waits for no more than the default deadline (2 minutes) and then returns the current state of the operation, which might be `DONE` or still in progress.
        ///
        /// This method is called on a best-effort basis. Specifically:
        /// - In uncommon cases, when the server is overloaded, the request might return before the default deadline is reached, or might return after zero seconds.
        /// - If the default deadline is reached, there is no guarantee that the operation is actually done when the method returns. Be prepared to retry if the operation is not `DONE`.
        pub async fn wait(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitRegionOperationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.compute.v1small.RegionOperations/Wait",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
