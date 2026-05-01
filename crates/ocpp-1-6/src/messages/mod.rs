//! OCPP 1.6 消息类型

pub mod call;
pub mod call_result;
pub mod call_error;
pub mod envelope;
pub mod config;

pub mod calls;
pub mod confs;

pub use envelope::{Call, CallResult, CallError};
pub use config::{OcppConfig, SharedConfig};