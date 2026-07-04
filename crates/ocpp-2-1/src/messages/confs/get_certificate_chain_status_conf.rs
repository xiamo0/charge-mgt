//! GetCertificateChainStatus Confirmation (Block M — 2.1 New)
use crate::common::CertificateStatusType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateChainStatusConfirmation {
    pub certificate_status: Vec<CertificateStatusType>,
}
