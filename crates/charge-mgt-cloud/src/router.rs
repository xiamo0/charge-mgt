//! v1 HTTP 路由装配。
//!
//! 所有业务端点统一 nest 在 `/api/v1` 下；健康检查、根路径等基础设施端点
//! 由 `main.rs::build_router` 在外层叠加（不在本文件）。
//!
//! # 路由清单（自动维护）
//!
//! | Method   | Path                                                            | Handler                                |
//! | -------- | --------------------------------------------------------------- | -------------------------------------- |
//! | GET/POST | `/api/v1/charge-points`                                         | list / create                          |
//! | GET/PATCH/DELETE | `/api/v1/charge-points/:charge_point_id`               | get / update / soft_delete             |
//! | POST     | `/api/v1/charge-points/:charge_point_id/restore`                | restore                                |
//! | GET      | `/api/v1/charge-points/:charge_point_id/connectors`             | nested_list                            |
//! | GET/PATCH | `/api/v1/charge-points/:charge_point_id/connectors/:connector_id` | get / update                       |
//! | GET      | `/api/v1/charge-points/:charge_point_id/charging-profiles`      | nested_list (profile)                  |
//! | GET      | `/api/v1/connectors`                                            | global list                            |
//! | GET/POST | `/api/v1/identities`                                            | list / create                          |
//! | GET/PATCH/DELETE | `/api/v1/identities/:id`                            | get / update / block (soft)            |
//! | GET      | `/api/v1/identities/by-tag/:tag_id`                             | get_by_tag                             |
//! | POST     | `/api/v1/identities/:id/activate`                               | activate                               |
//! | POST     | `/api/v1/identities/:id/block`                                  | block                                  |
//! | GET      | `/api/v1/transactions`                                          | list                                   |
//! | GET/PATCH | `/api/v1/transactions/:id`                                      | get / update                           |
//! | GET      | `/api/v1/transactions/by-transaction/:txn_id`                   | get_by_transaction_id                 |
//! | POST     | `/api/v1/transactions/:id/settle`                               | settle                                 |
//! | POST     | `/api/v1/transactions/:id/refund`                               | refund                                 |
//! | GET/POST | `/api/v1/reservations`                                          | list / create                          |
//! | GET/PATCH | `/api/v1/reservations/:id`                                      | get / update                           |
//! | POST     | `/api/v1/reservations/:id/cancel`                               | cancel                                 |
//! | GET/POST | `/api/v1/charging-profiles`                                    | list / create                          |
//! | GET/DELETE | `/api/v1/charging-profiles/:id`                                | get / delete (physical)                |

use axum::{
    Router,
    routing::{get, post},
};

/// 装配 v1 业务路由（nest 在 `/api/v1` 下）。
pub fn build() -> Router {
    Router::new().nest("/api/v1", v1_routes())
}

fn v1_routes() -> Router {
    let mut router = Router::new();
    #[cfg(feature = "ocpp_1_6")]
    {
        router = router.merge(ocpp_1_6_route());
    }
    #[cfg(feature = "ocpp_2_0_1")]
    {
        router = router.merge(ocpp_2_0_1_route());
    }
    router
}

#[cfg(feature = "ocpp_2_0_1")]
fn ocpp_2_0_1_route() -> Router {}
#[cfg(feature = "ocpp_1_6")]
fn ocpp_1_6_route() -> Router {
    use crate::ocpp16::http_handler::{
        charge_connector, charge_point, charge_reservation, charge_transaction, identity, profile,
        send_ocpp16_message,
    };
    Router::new()
        .route(
            "/send-ocpp-message/:action",
            post(send_ocpp16_message::send),
        )
        .route(
            "/charge-points",
            get(charge_point::list).post(charge_point::create),
        )
        .route(
            "/charge-points/:charge_point_id",
            get(charge_point::get)
                .patch(charge_point::update)
                .delete(charge_point::delete),
        )
        .route(
            "/charge-points/:charge_point_id/restore",
            post(charge_point::restore),
        )
        // nested connectors under charge point
        .route(
            "/charge-points/:charge_point_id/connectors",
            get(charge_connector::nested_list),
        )
        .route(
            "/charge-points/:charge_point_id/connectors/:connector_id",
            get(charge_connector::get).patch(charge_connector::update),
        )
        // nested profiles under charge point
        .route(
            "/charge-points/:charge_point_id/charging-profiles",
            get(profile::nested_list),
        )
        // connectors (global list only)
        .route("/connectors", get(charge_connector::list))
        // identities
        .route("/identities", get(identity::list).post(identity::create))
        .route(
            "/identities/:id",
            get(identity::get)
                .patch(identity::update)
                .delete(identity::delete),
        )
        .route("/identities/by-tag/:tag_id", get(identity::get_by_tag))
        .route("/identities/:id/activate", post(identity::activate))
        .route("/identities/:id/block", post(identity::block))
        // transactions (read-only + settle/refund; create is via OCPP)
        .route("/transactions", get(charge_transaction::list))
        .route(
            "/transactions/:id",
            get(charge_transaction::get).patch(charge_transaction::update),
        )
        .route(
            "/transactions/by-transaction/:txn_id",
            get(charge_transaction::get_by_transaction_id),
        )
        .route("/transactions/:id/settle", post(charge_transaction::settle))
        .route("/transactions/:id/refund", post(charge_transaction::refund))
        // reservations
        .route(
            "/reservations",
            get(charge_reservation::list).post(charge_reservation::create),
        )
        .route(
            "/reservations/:id",
            get(charge_reservation::get).patch(charge_reservation::update),
        )
        .route("/reservations/:id/cancel", post(charge_reservation::cancel))
        // charging profiles
        .route(
            "/charging-profiles",
            get(profile::list).post(profile::create),
        )
        .route(
            "/charging-profiles/:id",
            get(profile::get).delete(profile::delete),
        )
}
