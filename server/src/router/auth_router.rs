use axum::extract::FromRef;
use axum::Router;
use axum::routing::post;

use crate::actions::auth_actions::{login_member_handler, register_member_handler};
use crate::model::model::ModelController;

#[derive(Clone, FromRef, Debug)]
pub struct AppState {
    pub mc: ModelController,
}

pub fn auth_router(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/register", post(register_member_handler))
        .route("/login", post(login_member_handler))
        .with_state(app_state.clone())
}
