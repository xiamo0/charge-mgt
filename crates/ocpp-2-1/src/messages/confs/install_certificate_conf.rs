//! InstallCertificate Confirmation (Block M)
use serde::{Deserialize, Serialize};
use crate::common::{InstallCertificateStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateConfirmation {
    pub status: InstallCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
