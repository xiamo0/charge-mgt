//! GetCertificateStatus Confirmation
use crate::common::response_status::GetCertificateStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusConfirmation {
    pub status: GetCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
