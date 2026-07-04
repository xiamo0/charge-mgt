//! CertificateSigned Confirmation (Block M)
use serde::{Deserialize, Serialize};
use crate::common::{CertificateSignedStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedConfirmation {
    pub status: CertificateSignedStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
