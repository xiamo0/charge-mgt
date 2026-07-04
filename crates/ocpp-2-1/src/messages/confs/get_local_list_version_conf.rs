//! GetLocalListVersion Confirmation (Block D)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionConfirmation {
    pub version_number: i32,
}
