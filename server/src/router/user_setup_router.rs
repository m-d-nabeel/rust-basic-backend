use axum::extract::FromRef;
use axum::Router;
use axum::routing::post;

use crate::actions::user_setup_action::{login_user_handler, register_user_handler};
use crate::model::model::ModelController;

#[derive(Clone, FromRef, Debug)]
pub struct AppState {
    pub mc: ModelController,
}

pub fn user_setup_router(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/register", post(register_user_handler))
        .route("/login", post(login_user_handler))
        .with_state(app_state.clone())
}
