//! RemoteStopTransaction 响应

use serde::{Deserialize, Serialize};
use crate::common::status::RemoteStartStopStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteStopTransactionConfirmation {
    pub status: RemoteStartStopStatus,
}

impl RemoteStopTransactionConfirmation {
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