use axum::middleware::from_fn_with_state;
use axum::Router;
use axum::routing::get;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;

use model::model::ModelController;

use crate::actions::user_actions::get_members;
use crate::config::Config;
use crate::router::auth_router::{AppState, auth_router};
use crate::router::hello_router::hello_router;
use crate::utils::auth_mw::auth;
use crate::utils::cors_setup::cors_layer;

mod actions;
mod config;
mod model;
mod router;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Config::init();

    let db_pool = PgPoolOptions::new()
        .connect(config.database_url.as_str())
        .await?;

    let mc = ModelController::new(db_pool);
    let app = server_main(mc)
        .layer(cors_layer())
        .layer(tower::ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let tcp_listener = tokio::net::TcpListener::bind(config.api_url).await?;
    println!("Listening on {}", tcp_listener.local_addr()?);
    axum::serve(tcp_listener, app).await?;

    Ok(())
}

fn server_main(mc: ModelController) -> Router {
    let cloned = mc.clone();
    let app_state = AppState { mc: cloned };
    Router::new()
        .nest("/", hello_router())
        .route(
            "/api/members",
            get(get_members)
                .with_state(app_state.clone())
                .route_layer(from_fn_with_state(app_state.clone(), auth)),
        )
        .nest("/api/setup", auth_router(mc.clone()))
}
