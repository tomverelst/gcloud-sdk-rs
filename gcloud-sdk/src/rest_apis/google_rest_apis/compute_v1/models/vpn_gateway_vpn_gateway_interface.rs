use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// VpnGatewayVpnGatewayInterface : A VPN gateway interface.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VpnGatewayVpnGatewayInterface {
    /// [Output Only] Numeric identifier for this VPN interface associated with the VPN gateway.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// URL of the VLAN attachment (interconnectAttachment) resource for this VPN gateway interface. When the value of this field is present, the VPN gateway is used for HA VPN over Cloud Interconnect; all egress or ingress traffic for this VPN gateway interface goes through the specified VLAN attachment resource.
    #[serde(
        rename = "interconnectAttachment",
        skip_serializing_if = "Option::is_none"
    )]
    pub interconnect_attachment: Option<String>,
    /// [Output Only] IP address for this VPN interface associated with the VPN gateway. The IP address could be either a regional external IP address or a regional internal IP address. The two IP addresses for a VPN gateway must be all regional external or regional internal IP addresses. There cannot be a mix of regional external IP addresses and regional internal IP addresses. For HA VPN over Cloud Interconnect, the IP addresses for both interfaces could either be regional internal IP addresses or regional external IP addresses. For regular (non HA VPN over Cloud Interconnect) HA VPN tunnels, the IP address must be a regional external IP address.
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// [Output Only] IPv6 address for this VPN interface associated with the VPN gateway. The IPv6 address must be a regional external IPv6 address. The format is RFC 5952 format (e.g. 2001:db8::2d9:51:0:0).
    #[serde(rename = "ipv6Address", skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
}

impl VpnGatewayVpnGatewayInterface {
    /// A VPN gateway interface.
    pub fn new() -> VpnGatewayVpnGatewayInterface {
        VpnGatewayVpnGatewayInterface {
            id: None,
            interconnect_attachment: None,
            ip_address: None,
            ipv6_address: None,
        }
    }
}
