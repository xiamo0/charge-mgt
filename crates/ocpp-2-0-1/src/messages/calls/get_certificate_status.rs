//! GetCertificateStatus Request (Functional Block J)
//! 查询证书 OCSP 状态

use serde::{Deserialize, Serialize};
use crate::common::OCSPRequestDataType;

/// GetCertificateStatus 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusRequest {
    /// OCSP 请求数据
    pub ocsp_request_data: OCSPRequestDataType,
}

impl GetCertificateStatusRequest {
    pub fn new(ocsp_request_data: OCSPRequestDataType) -> Self {
        Self { ocsp_request_data }
    }
}

pub const ACTION: &str = "GetCertificateStatus";