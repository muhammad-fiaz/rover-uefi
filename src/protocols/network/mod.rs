//! # Network
//!
//! Network protocol interfaces.
//!
//! Each wrapper borrows the firmware protocol
//! instance. Asynchronous tokens require the
//! caller to keep memory alive until
//! completion.

pub mod application;
pub mod arp_protocol;
pub mod dhcp_protocol;
pub mod dns;
pub mod enumeration;
pub mod guid;
pub mod http;
pub mod http_boot_callback;
pub mod ip;
pub mod ip_config;
pub mod ip_protocol;
pub mod ipv4;
pub mod ipv6;
pub mod mac;
pub mod managed;
pub mod mtftp;
pub mod packets;
pub mod pxe;
pub mod redfish_discover;
pub mod rest_ex;
pub mod simple_network;
pub mod stack;
pub mod tcp;
pub mod tls;
pub mod tls_protocol;
pub mod types;
pub mod udp;
pub mod usb_ethernet;
pub mod validation;
pub mod vlan_config;
pub mod wifi2;

pub use arp_protocol::{
    ARP_PROTOCOL_GUID, ARP_SERVICE_BINDING_PROTOCOL_GUID, EfiArpAdd, EfiArpCancel,
    EfiArpConfigData, EfiArpConfigure, EfiArpDelete, EfiArpFind, EfiArpFlush, EfiArpProtocol,
    EfiArpRequest,
};
pub use dhcp_protocol::{
    EFI_DHCP4_PROTOCOL_GUID, EFI_DHCP6_PROTOCOL_GUID, EfiDhcp4ModeData, EfiDhcp4Protocol,
    EfiDhcp4State, EfiDhcp6ModeData, EfiDhcp6Protocol, EfiDhcp6State,
};
pub use dns::protocol::{EfiDns4Protocol, EfiDns6Protocol};
pub use guid::{
    ARP_PROTOCOL_GUID as ARP_GUID, ARP_SERVICE_BINDING_PROTOCOL_GUID as ARP_SB_GUID, DNS4_GUID,
    DNS6_GUID, HTTP_GUID, IP4_GUID, IP6_GUID, MANAGED_NETWORK_GUID, PXE_BASE_CODE_GUID,
    REDFISH_DISCOVER_PROTOCOL_GUID as REDFISH_DISCOVER_GUID, REST_EX_PROTOCOL_GUID as REST_EX_GUID,
    REST_EX_SERVICE_BINDING_PROTOCOL_GUID as REST_EX_SB_GUID, SIMPLE_NETWORK_GUID, TCP4_GUID,
    TCP6_GUID, UDP4_GUID, UDP6_GUID, VLAN_CONFIG_PROTOCOL_GUID as VLAN_CONFIG_GUID,
};
pub use http::protocol::{EfiHttpProtocol, Http};
pub use http_boot_callback::{
    EFI_HTTP_BOOT_CALLBACK_PROTOCOL_GUID, EfiHttpBootCallback, EfiHttpBootCallbackDataType,
    EfiHttpBootCallbackProtocol,
};
pub use ip_config::{
    EFI_IP4_CONFIG2_PROTOCOL_GUID, EFI_IP6_CONFIG_PROTOCOL_GUID, EfiIp4Config2DataType,
    EfiIp4Config2ManualAddress, EfiIp4Config2Policy, EfiIp4Config2Protocol, EfiIp6ConfigPolicy,
    EfiIp6ConfigProtocol,
};
pub use mtftp::{
    EFI_MTFTP4_PROTOCOL_GUID, EFI_MTFTP6_PROTOCOL_GUID, EfiMtftp4ModeData, EfiMtftp4Protocol,
    EfiMtftp4Token, EfiMtftp6ModeData, EfiMtftp6Protocol,
};
pub use redfish_discover::{
    EFI_REDFISH_DISCOVER_HOST_INTERFACE, EFI_REDFISH_DISCOVER_KEEP_ALIVE,
    EFI_REDFISH_DISCOVER_RENEW, EFI_REDFISH_DISCOVER_SSDP, EFI_REDFISH_DISCOVER_SSDP_UDP6,
    EFI_REDFISH_DISCOVER_VALIDATION, EfiIpAddress, EfiRedfishDiscoverAbortAcquire,
    EfiRedfishDiscoverAcquireService, EfiRedfishDiscoverNetworkInterface,
    EfiRedfishDiscoverNetworkList, EfiRedfishDiscoverProtocol, EfiRedfishDiscoverReleaseService,
    EfiRedfishDiscoveredInformation, EfiRedfishDiscoveredInstance, EfiRedfishDiscoveredList,
    EfiRedfishDiscoveredToken, REDFISH_DISCOVER_PROTOCOL_GUID, REDFISH_DISCOVER_TOKEN_SIGNATURE,
};
pub use rest_ex::{
    EfiHttpMessage, EfiRestExAsyncSendReceive, EfiRestExConfigType, EfiRestExConfigure,
    EfiRestExEventService, EfiRestExGetModeData, EfiRestExGetService, EfiRestExProtocol,
    EfiRestExServiceAccessMode, EfiRestExServiceInfoHeader, EfiRestExServiceInfoV10,
    EfiRestExServiceInfoVer, EfiRestExServiceType, EfiRestExToken, EfiRestGetTime,
    EfiRestSendReceive, REST_EX_PROTOCOL_GUID, REST_EX_SERVICE_BINDING_PROTOCOL_GUID,
};
pub use tls_protocol::{
    EFI_TLS_CONFIGURATION_PROTOCOL_GUID, EFI_TLS_PROTOCOL_GUID,
    EFI_TLS_SERVICE_BINDING_PROTOCOL_GUID, EfiTlsConfigDataType, EfiTlsConfigurationGetData,
    EfiTlsConfigurationProtocol, EfiTlsConfigurationSetData, EfiTlsCryptMode, EfiTlsFragmentData,
    EfiTlsProtocol, EfiTlsSessionDataType,
};
pub use usb_ethernet::{
    USB_CDC_ACM_SUBCLASS, USB_CDC_CLASS, USB_CDC_DATA_CLASS, USB_CDC_ECM_SUBCLASS,
    USB_CDC_NCM_SUBCLASS, USB_ETHERNET_PROTOCOL_GUID, USB_MISC_CLASS, USB_RNDIS_ETHERNET_PROTOCOL,
    USB_RNDIS_SUBCLASS, UsbEthernet, UsbEthernetFunDescriptor, UsbEthernetProtocol,
    UsbEthernetUndi, UsbHeaderFunDescriptor, UsbUnionFunDescriptor,
    desc_subtype as usb_eth_desc_subtype, packet_filter as usb_eth_packet_filter,
};
pub use vlan_config::{
    EfiVlanConfigFind, EfiVlanConfigProtocol, EfiVlanConfigRemove, EfiVlanConfigSet,
    EfiVlanFindData, VLAN_CONFIG_PROTOCOL_GUID,
};
pub use wifi2::{
    EFI_MAX_SSID_LEN, EFI_WIFI2_PROTOCOL_GUID, EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL_GUID,
    Efi80211BssType, Efi80211ConnectNetworkResultCode, Efi80211ConnectNetworkToken,
    Efi80211DisconnectNetworkToken, Efi80211GetNetworksToken, Efi80211MacAddress, Efi80211Ssid,
    Efi80211SuiteSelector, EfiWirelessMacConnectionIiConnectNetwork,
    EfiWirelessMacConnectionIiDisconnectNetwork, EfiWirelessMacConnectionIiGetNetworks,
    EfiWirelessMacConnectionIiProtocol,
};
