//! GetInstalledCertificateIds Confirmation (Block M)
use crate::common::{
    CertificateHashDataChainType, GetInstalledCertificateStatusEnumType, StatusInfoType,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsConfirmation {
    pub status: GetInstalledCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data_chain: Option<Vec<CertificateHashDataChainType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
