use axum::extract::FromRef;
use axum::Router;
use axum::routing::get;

use crate::actions::user_action::{create_user, get_users};
use crate::database::model::ModelController;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn user_router(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/", get(get_users).post(create_user))
        .with_state(app_state)
}