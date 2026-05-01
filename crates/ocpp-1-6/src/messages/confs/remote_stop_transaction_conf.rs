//! RemoteStopTransaction 响应

use crate::common::status::RemoteStartStopStatus;
use serde::{Deserialize, Serialize};

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
