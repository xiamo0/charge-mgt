//! GetInstalledCertificateIds Confirmation
use crate::common::response_status::GetInstalledCertificateStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsConfirmation {
    pub status: GetInstalledCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data_chain: Option<Vec<crate::common::CertificateHashDataChainType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
