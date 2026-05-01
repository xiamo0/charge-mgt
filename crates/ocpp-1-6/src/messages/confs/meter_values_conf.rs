//! MeterValues 响应

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MeterValuesConfirmation;
