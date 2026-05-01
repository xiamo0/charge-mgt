//! GetLocalListVersion 响应

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLocalListVersionConfirmation {
    pub list_version: i32,
}

impl GetLocalListVersionConfirmation {
    pub fn new(version: i32) -> Self {
        Self { list_version: version }
    }
}