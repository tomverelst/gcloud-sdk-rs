/*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::compute_v1::models;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;

/// VpnGateway : Represents a HA VPN gateway. HA VPN is a high-availability (HA) Cloud VPN solution that lets you securely connect your on-premises network to your Google Cloud Virtual Private Cloud network through an IPsec VPN connection in a single region. For more information about Cloud HA VPN solutions, see Cloud VPN topologies .
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VpnGateway {
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The IP family of the gateway IPs for the HA-VPN gateway interfaces. If not specified, IPV4 will be used.
    #[serde(rename = "gatewayIpVersion", skip_serializing_if = "Option::is_none")]
    pub gateway_ip_version: Option<GatewayIpVersion>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of resource. Always compute#vpnGateway for VPN gateways.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A fingerprint for the labels being applied to this VpnGateway, which is essentially a hash of the labels set used for optimistic locking. The fingerprint is initially generated by Compute Engine and changes after every request to modify or update labels. You must always provide an up-to-date fingerprint hash in order to update or change labels, otherwise the request will fail with error 412 conditionNotMet. To see the latest fingerprint, make a get() request to retrieve a VpnGateway.
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "labelFingerprint", skip_serializing_if = "Option::is_none")]
    pub label_fingerprint: Option<Vec<u8>>,
    /// Labels for this resource. These can only be added or modified by the setLabels method. Each label key/value pair must comply with RFC1035. Label values may be empty.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL of the network to which this VPN gateway is attached. Provided by the client when the VPN gateway is created.
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// [Output Only] URL of the region where the VPN gateway resides.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// The stack type for this VPN gateway to identify the IP protocols that are enabled. Possible values are: IPV4_ONLY, IPV4_IPV6. If not specified, IPV4_ONLY will be used.
    #[serde(rename = "stackType", skip_serializing_if = "Option::is_none")]
    pub stack_type: Option<StackType>,
    /// The list of VPN interfaces associated with this VPN gateway.
    #[serde(rename = "vpnInterfaces", skip_serializing_if = "Option::is_none")]
    pub vpn_interfaces: Option<Vec<models::VpnGatewayVpnGatewayInterface>>,
}

impl VpnGateway {
    /// Represents a HA VPN gateway. HA VPN is a high-availability (HA) Cloud VPN solution that lets you securely connect your on-premises network to your Google Cloud Virtual Private Cloud network through an IPsec VPN connection in a single region. For more information about Cloud HA VPN solutions, see Cloud VPN topologies .
    pub fn new() -> VpnGateway {
        VpnGateway {
            creation_timestamp: None,
            description: None,
            gateway_ip_version: None,
            id: None,
            kind: None,
            label_fingerprint: None,
            labels: None,
            name: None,
            network: None,
            region: None,
            self_link: None,
            stack_type: None,
            vpn_interfaces: None,
        }
    }
}
/// The IP family of the gateway IPs for the HA-VPN gateway interfaces. If not specified, IPV4 will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GatewayIpVersion {
    #[serde(rename = "IPV4")]
    Ipv4,
    #[serde(rename = "IPV6")]
    Ipv6,
}

impl Default for GatewayIpVersion {
    fn default() -> GatewayIpVersion {
        Self::Ipv4
    }
}
/// The stack type for this VPN gateway to identify the IP protocols that are enabled. Possible values are: IPV4_ONLY, IPV4_IPV6. If not specified, IPV4_ONLY will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StackType {
    #[serde(rename = "IPV4_IPV6")]
    Ipv6,
    #[serde(rename = "IPV4_ONLY")]
    Only,
}

impl Default for StackType {
    fn default() -> StackType {
        Self::Ipv6
    }
}
