//! CancelReservation 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::cancel_reservation_conf::CancelReservationConfirmation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelReservationRequest {
    pub reservation_id: i32,
}

pub trait CancelReservationHandler: Send + Sync {
    fn handle(&self, req: CancelReservationRequest) -> CancelReservationConfirmation;
}

pub struct DefaultCancelReservationHandler;

impl Default for DefaultCancelReservationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultCancelReservationHandler {
    pub fn new() -> Self {
        Self
    }
}

impl CancelReservationHandler for DefaultCancelReservationHandler {
    fn handle(&self, _req: CancelReservationRequest) -> CancelReservationConfirmation {
        CancelReservationConfirmation::accepted()
    }
}