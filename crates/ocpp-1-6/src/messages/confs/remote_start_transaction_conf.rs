//! RemoteStartTransaction 响应

use serde::{Deserialize, Serialize};
use crate::common::status::RemoteStartStopStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteStartTransactionConfirmation {
    pub status: RemoteStartStopStatus,
}

impl RemoteStartTransactionConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: RemoteStartStopStatus::Accepted,
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: RemoteStartStopStatus::Rejected,
        }
    }
}