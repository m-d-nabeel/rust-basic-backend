use axum::extract::FromRef;
use axum::Router;
use axum::routing::{get, post};

use crate::actions::chat_action::{create_message, get_messages, post_chatician, root_chatician};
use crate::database::model::ModelController;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn communication_router(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/", get(root_chatician).post(post_chatician))
        .route("/:channel_id", post(create_message).get(get_messages))
        .with_state(app_state)
}