//! Get15118EVCertificate Confirmation
use serde::{Deserialize, Serialize};
use crate::common::response_status::Iso15118EVCertificateStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateConfirmation {
    pub status: Iso15118EVCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
