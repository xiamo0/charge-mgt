//! HTTP 管理 API 鉴权（JWT）
//!
//! - `jwt`：token 生成/校验
//! - `password`：argon2id 密码哈希
//! - `service`：登录逻辑
//! - `middleware`：require_auth 请求拦截
//! - `handler`：/auth/login、/auth/me 端点

pub mod handler;
pub mod jwt;
pub mod middleware;
pub mod password;
pub mod service;

pub use middleware::AuthContext;
