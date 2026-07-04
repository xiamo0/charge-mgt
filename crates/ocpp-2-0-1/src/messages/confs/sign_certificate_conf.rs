//! SignCertificate Confirmation
use crate::common::response_status::SignCertificateStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateConfirmation {
    pub status: SignCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
