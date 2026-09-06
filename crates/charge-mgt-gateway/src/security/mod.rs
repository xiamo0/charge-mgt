//! OCPP 1.6 链路安全（模式 1-6）
//!
//! - 模式 1/2：明文 ws://（无认证 / Basic Auth）
//! - 模式 3/4：wss:// 单向 TLS（无认证 / Basic Auth）
//! - 模式 5/6：wss:// + mTLS 双向证书认证（纯 mTLS / mTLS + Basic Auth）

pub mod auth;
pub mod basic_auth;
pub mod policy;
pub mod tls;

pub use policy::SecurityMode;
