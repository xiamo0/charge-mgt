use axum::{Router, routing::{get, patch, post}};

use crate::handler::{
    charge_connector, charge_point, charge_reservation, charge_transaction, identity, profile,
};

pub fn build() -> Router {
    Router::new().nest("/api/v1", v1_routes())
}

fn v1_routes() -> Router {
    Router::new()
        // charge points
        .route("/charge-points", get(charge_point::list).post(charge_point::create))
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
        .route(
            "/reservations/:id/cancel",
            post(charge_reservation::cancel),
        )
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
