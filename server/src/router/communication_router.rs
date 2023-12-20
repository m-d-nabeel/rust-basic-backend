use axum::extract::FromRef;
use axum::Router;
use axum::routing::{get, post};

use crate::actions::communication_action::{create_message, get_messages, post_chatician, root_chatician};
use crate::utils::model::ModelController;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn communication_router(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/", get(root_chatician).post(post_chatician))
        .route("/send", post(create_message).get(get_messages))
        .with_state(app_state)
}