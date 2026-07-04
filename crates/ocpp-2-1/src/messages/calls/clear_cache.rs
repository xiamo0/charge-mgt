//! ClearCache Request (Block C)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClearCacheRequest {}

pub const ACTION: &str = "ClearCache";
