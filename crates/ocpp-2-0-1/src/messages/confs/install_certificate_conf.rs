//! InstallCertificate Confirmation
use serde::{Deserialize, Serialize};
use crate::common::response_status::InstallCertificateStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateConfirmation {
    pub status: InstallCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
