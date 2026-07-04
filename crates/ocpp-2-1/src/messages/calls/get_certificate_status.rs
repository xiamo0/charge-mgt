//! GetCertificateStatus Request (Block M)
use serde::{Deserialize, Serialize};
use crate::common::OCSPRequestDataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusRequest {
    pub ocsp_request_data: OCSPRequestDataType,
}

pub const ACTION: &str = "GetCertificateStatus";
