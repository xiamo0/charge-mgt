//! Get15118EVCertificate Confirmation (Block M)
use crate::common::{Iso15118EVCertificateStatusEnumType, StatusInfoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateConfirmation {
    pub status: Iso15118EVCertificateStatusEnumType,
    pub exi_response: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
