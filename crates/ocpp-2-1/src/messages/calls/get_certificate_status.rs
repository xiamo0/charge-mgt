//! GetCertificateStatus Request (Block M)
use crate::common::OCSPRequestDataType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusRequest {
    pub ocsp_request_data: OCSPRequestDataType,
}

pub const ACTION: &str = "GetCertificateStatus";
