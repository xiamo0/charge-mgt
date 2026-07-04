//! HTTP 层数据传输对象（DTO）。
//!
//! 与 entity 层解耦：HTTP 请求/响应只携带 DTO，不直接暴露数据库 Model。
//! 这样数据库列变更时只动 service 内的映射，handler 与 DTO 不用改。
//!
//! 命名约定：
//! * `CreateXxx`：POST 请求体（创建新行）
//! * `UpdateXxx`：PATCH 请求体（部分字段可空，未提供则不更新）
//! * `XxxListQuery`：列表查询 query string（全部 `Option`）
//! * `XxxResponse`：直接复用 entity `Model`，类型别名形式导出
//!
//! 通用结构（`ApiResponse<T>` / `PageQuery` / `PageResult<T>`）在 [`common`]。

pub mod charge_connector;
pub mod charge_point;
pub mod charge_reservation;
pub mod charge_transaction;
pub mod common;
pub mod identity_info;
pub mod smart_charge_profile;
