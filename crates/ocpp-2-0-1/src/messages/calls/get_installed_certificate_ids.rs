//! GetInstalledCertificateIds Request (Functional Block J)
//! 查询已安装证书

use super::install_certificate::InstallCertificateUseEnumType;
use serde::{Deserialize, Serialize};

/// GetInstalledCertificateIds 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsRequest {
    /// 证书类型列表 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<Vec<InstallCertificateUseEnumType>>,
}

impl GetInstalledCertificateIdsRequest {
    pub fn new() -> Self {
        Self {
            certificate_type: None,
        }
    }

    pub fn filter_by_types(mut self, types: Vec<InstallCertificateUseEnumType>) -> Self {
        self.certificate_type = Some(types);
        self
    }
}

impl Default for GetInstalledCertificateIdsRequest {
    fn default() -> Self {
        Self::new()
    }
}

pub const ACTION: &str = "GetInstalledCertificateIds";
