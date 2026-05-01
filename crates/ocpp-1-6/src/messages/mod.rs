//! OCPP 1.6 消息类型

pub mod call;
pub mod call_error;
pub mod call_result;
pub mod config;
pub mod envelope;

pub mod calls;
pub mod confs;

pub use config::{OcppConfig, SharedConfig};
pub use envelope::{Call, CallError, CallResult};
