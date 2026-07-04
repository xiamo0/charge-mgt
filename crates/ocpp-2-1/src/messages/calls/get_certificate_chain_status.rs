//! GetCertificateChainStatus Request (Block M — 2.1 New)
use crate::common::CertificateStatusRequestInfoType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateChainStatusRequest {
    pub certificate_status_requests: Vec<CertificateStatusRequestInfoType>,
}

pub const ACTION: &str = "GetCertificateChainStatus";
