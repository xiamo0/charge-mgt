//! 业务服务层。
//!
//! 纯业务逻辑，**不**依赖 axum；每个子模块对应一个 REST 资源，提供 list /
//! get / create / update / delete 一类操作，并通过 [`crate::error::AppError`]
//! 表达错误。
//!
//! 命名约定：每个子模块内的函数与 handler 一一对应，handler 只做 axum 抽取
//! 与响应包装，业务行为都委托给这里。

pub mod charge_connector;
pub mod charge_point;
pub mod charge_transaction;
pub mod identity;
pub mod profile;
pub mod reservation;
