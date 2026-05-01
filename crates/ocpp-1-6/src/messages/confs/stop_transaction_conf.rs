//! StopTransaction 响应

use serde::{Deserialize, Serialize};
use crate::common::id_tag::IdTagInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Default)]
pub struct StopTransactionConfirmation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}