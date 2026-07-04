//! GetLocalListVersion Request (Block D)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetLocalListVersionRequest {}

pub const ACTION: &str = "GetLocalListVersion";
