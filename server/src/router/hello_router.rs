use axum::Router;
use axum::routing::get;

use crate::actions::hello_actions::{hello_params, hello_url_id, hello_world};

pub fn hello_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/params", get(hello_params))
        .route("/:id", get(hello_url_id))
}