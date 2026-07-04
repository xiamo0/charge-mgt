//! GetInstalledCertificateIds Request (Block M)
use serde::{Deserialize, Serialize};
use crate::common::InstallCertificateUseEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<Vec<InstallCertificateUseEnumType>>,
}

pub const ACTION: &str = "GetInstalledCertificateIds";
