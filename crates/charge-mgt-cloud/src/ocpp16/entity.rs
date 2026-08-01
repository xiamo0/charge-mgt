//! 平台实体层：sea-orm `Entity` / `Model` / `ActiveModel` 定义。
//!
//! 每个子模块对应一张 Phase 0 表，列类型与 SQL 严格对齐；SMALLINT 字段直接
//! 用 [`enums`] 中定义的 Rust enum 替换裸 `i16`。
//!
//! 为简洁起见，业务语义、字段含义、SQL 列名映射写在各 Model 的字段 `///` 中；
//! 枚举变体的业务含义在 [`enums`] 中。
//!
//! # 重导出
//!
//! 提供 `pub use ... as ChargeXxx` 别名，使其他模块可直接
//! `use crate::entity::ChargePoints` 而无需再 `::Entity`。

pub mod charge_connector;
pub mod charge_point;
pub mod charge_reservation;
pub mod charge_transaction;
pub mod enums;
pub mod identity_info;
pub mod sent_messages;
pub mod smart_charge_profile;

pub use charge_connector::Entity as ChargeConnectors;
pub use charge_point::Entity as ChargePoints;
pub use charge_reservation::Entity as ChargeReservations;
pub use charge_transaction::Entity as ChargeTransactions;
pub use identity_info::Entity as IdentityInfos;
pub use smart_charge_profile::Entity as SmartChargeProfiles;
