//! GetCertificateStatus Confirmation (Block M)
use serde::{Deserialize, Serialize};
use crate::common::{GetCertificateStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusConfirmation {
    pub status: GetCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
