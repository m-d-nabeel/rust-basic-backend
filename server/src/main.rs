use axum::Router;
use axum::routing::get;
use dotenv::dotenv;
use tower_http::trace::TraceLayer;

use model::model::ModelController;

use crate::actions::user_action::get_users;
use crate::config::Config;
use crate::database::db::connect_db;
use crate::router::hello_router::hello_router;
use crate::router::user_setup_router::{AppState, user_setup_router};
use crate::utils::auth_mw::auth;
use crate::utils::cors_setup::cors_layer;

mod actions;
mod config;
mod database;
mod model;
mod router;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Config::init();

    let db_pool = connect_db(config.database_url.as_str()).await?;
    database::db_init::db_initialization(&db_pool).await?;

    let mc = ModelController::new(db_pool);

    let app = main_router(mc)
        .layer(cors_layer())
        .layer(tower::ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // NOTE: Service Builder is used for top_down preference not needed here

    let tcp_listener = tokio::net::TcpListener::bind(config.api_url).await?;
    println!("Listening on {}", tcp_listener.local_addr()?);
    axum::serve(tcp_listener, app).await?;

    Ok(())
}

fn main_router(mc: ModelController) -> Router {
    let cloned = mc.clone();
    let app_state = AppState { mc:cloned};
    Router::new()
        .nest("/", hello_router())
        .route(
            "/chatician/users",
            get(get_users)
                .with_state(app_state.clone())
                .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth))
        )
        .nest("/chatician/users/setup", user_setup_router(mc.clone()))
}
