//! CancelReservation 消息及处理器
//!
//! 取消预订的请求与默认处理器。

use super::super::confs::cancel_reservation_conf::CancelReservationConfirmation;
use serde::{Deserialize, Serialize};

/// CancelReservation 请求，携带要取消的 reservationId
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelReservationRequest {
    /// 要取消的 reservationId
    pub reservation_id: i32,
}

/// CancelReservation 处理器接口
pub trait CancelReservationHandler: Send + Sync {
    fn handle(&self, req: CancelReservationRequest) -> CancelReservationConfirmation;
}

/// 默认实现：接受取消请求
pub struct DefaultCancelReservationHandler;

impl Default for DefaultCancelReservationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultCancelReservationHandler {
    /// 创建默认处理器
    pub fn new() -> Self {
        Self
    }
}

impl CancelReservationHandler for DefaultCancelReservationHandler {
    fn handle(&self, _req: CancelReservationRequest) -> CancelReservationConfirmation {
        CancelReservationConfirmation::accepted()
    }
}
