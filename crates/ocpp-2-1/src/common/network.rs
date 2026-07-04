//! Network Connection Types (Functional Block B)

use serde::{Deserialize, Serialize};

/// 网络接口类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OCPPInterfaceEnumType {
    Wired0,
    Wired1,
    Wired2,
    Wired3,
    Wireless0,
    Wireless1,
    Wireless2,
    Wireless3,
}

/// OCPP 版本枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OCPPVersionEnumType {
    OCPP12,
    OCPP15,
    OCPP16,
    OCPP20,
}

/// OCPP 传输层枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OCPPTransportEnumType {
    JSON,
    SOAP,
}

/// 网络连接配置类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConnectionProfileType {
    pub ocpp_version: OCPPVersionEnumType,
    pub ocpp_transport: OCPPTransportEnumType,
    pub ocpp_csms_url: String,
    pub message_timeout: i32,
    pub security_profile: i32,
    pub ocpp_interface: OCPPInterfaceEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<VPNType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn: Option<APNType>,
}

/// VPN 配置类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VPNType {
    pub server: String,
    pub user: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// APN 配置类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APNType {
    pub apn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_pin: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_only_preferred_network: Option<bool>,
}

/// 设置网络配置响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SetNetworkProfileStatusEnumType {
    Accepted,
    Rejected,
    Failed,
}
