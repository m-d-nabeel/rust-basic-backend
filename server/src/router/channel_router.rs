use axum::extract::FromRef;
use axum::Router;
use axum::routing::get;

use crate::actions::channel_action::{create_channel, get_channels};
use crate::database::model::ModelController;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn channel_router(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/", get(get_channels).post(create_channel))
        .with_state(app_state)
}