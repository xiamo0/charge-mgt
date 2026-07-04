//! SignCertificate Confirmation (Block M)
use serde::{Deserialize, Serialize};
use crate::common::{SignCertificateStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateConfirmation {
    pub status: SignCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
