//! GetCertificateChainStatus Confirmation (Block M — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::CertificateStatusType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateChainStatusConfirmation {
    pub certificate_status: Vec<CertificateStatusType>,
}
