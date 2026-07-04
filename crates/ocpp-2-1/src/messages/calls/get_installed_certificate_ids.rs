//! GetInstalledCertificateIds Request (Block M)
use crate::common::InstallCertificateUseEnumType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<Vec<InstallCertificateUseEnumType>>,
}

pub const ACTION: &str = "GetInstalledCertificateIds";
