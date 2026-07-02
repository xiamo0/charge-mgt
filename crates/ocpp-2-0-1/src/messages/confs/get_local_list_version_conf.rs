//! GetLocalListVersion Confirmation
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionConfirmation {
    pub version_number: i32,
}
