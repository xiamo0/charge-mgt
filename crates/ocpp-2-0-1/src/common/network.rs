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

/// 配置 OCPP 版本
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    pub format: MessageFormatEnumType,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

pub use crate::common::id_token::MessageFormatEnumType;

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

impl NetworkConnectionProfileType {
    pub fn new(
        ocpp_version: OCPPVersionEnumType,
        ocpp_transport: OCPPTransportEnumType,
        ocpp_csms_url: impl Into<String>,
        message_timeout: i32,
        security_profile: i32,
        ocpp_interface: OCPPInterfaceEnumType,
    ) -> Self {
        Self {
            ocpp_version,
            ocpp_transport,
            ocpp_csms_url: ocpp_csms_url.into(),
            message_timeout,
            security_profile,
            ocpp_interface,
            vpn: None,
            apn: None,
        }
    }

    pub fn with_vpn(mut self, vpn: VPNType) -> Self {
        self.vpn = Some(vpn);
        self
    }

    pub fn with_apn(mut self, apn: APNType) -> Self {
        self.apn = Some(apn);
        self
    }
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

impl VPNType {
    pub fn new(
        server: impl Into<String>,
        user: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            server: server.into(),
            user: user.into(),
            password: password.into(),
            group: None,
            key: None,
        }
    }
}

/// APN (Access Point Name) 配置类型
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

impl APNType {
    pub fn new(apn: impl Into<String>) -> Self {
        Self {
            apn: apn.into(),
            apn_user_name: None,
            apn_password: None,
            sim_pin: None,
            preferred_network: None,
            use_only_preferred_network: None,
        }
    }
}

/// 设置网络配置响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SetNetworkProfileStatusEnumType {
    Accepted,
    Rejected,
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_profile_roundtrip() {
        let profile = NetworkConnectionProfileType::new(
            OCPPVersionEnumType::OCPP20,
            OCPPTransportEnumType::JSON,
            "wss://csms.example.com/ocpp",
            30,
            2,
            OCPPInterfaceEnumType::Wired0,
        );
        let json = serde_json::to_string(&profile).unwrap();
        let de: NetworkConnectionProfileType = serde_json::from_str(&json).unwrap();
        assert_eq!(profile, de);
    }

    #[test]
    fn test_set_network_profile_status() {
        let variants = [
            SetNetworkProfileStatusEnumType::Accepted,
            SetNetworkProfileStatusEnumType::Rejected,
            SetNetworkProfileStatusEnumType::Failed,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: SetNetworkProfileStatusEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }
}
