//! GetCertificateChainStatus Request (Block M — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::CertificateStatusRequestInfoType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateChainStatusRequest {
    pub certificate_status_requests: Vec<CertificateStatusRequestInfoType>,
}

pub const ACTION: &str = "GetCertificateChainStatus";
