//! NotifyCustomerInformation Request (Functional Block G)
//! 上报客户数据（分页）

use serde::{Deserialize, Serialize};

/// NotifyCustomerInformation 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationRequest {
    /// 客户数据 (max 512)
    pub data: String,
    /// 生成时间
    pub generated_at: String,
    /// 请求 ID
    pub request_id: i32,
    /// 序列号
    pub seq_no: i32,
    /// 是否有后续数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
}

impl NotifyCustomerInformationRequest {
    pub fn new(
        data: impl Into<String>,
        generated_at: impl Into<String>,
        request_id: i32,
        seq_no: i32,
    ) -> Self {
        Self {
            data: data.into(),
            generated_at: generated_at.into(),
            request_id,
            seq_no,
            tbc: Some(false),
        }
    }
}

pub const ACTION: &str = "NotifyCustomerInformation";
