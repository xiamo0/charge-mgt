//! DeleteCertificate Confirmation (Block M)
use serde::{Deserialize, Serialize};
use crate::common::{DeleteCertificateStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateConfirmation {
    pub status: DeleteCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
