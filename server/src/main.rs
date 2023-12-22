use axum::http::{HeaderValue, Method};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::database::db::connect_db;
use crate::router::communication_router::communication_router;
use crate::router::hello_router::hello_router;
use crate::utils::model::ModelController;

pub use self::utils::errors::Result;

mod router;
mod actions;
mod utils;
mod database;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = connect_db().await.expect("Error connecting to the database");
    database::db_init::db_initialization(&db_pool).await?;
    let mc = ModelController::new().await?;
    let cors_middleware = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, "content-type".parse::<Method>().unwrap()])
        .allow_headers(Any);

    let app = Router::new()
        .nest("/", hello_router())
        .nest("/chatician", communication_router(mc.clone()))
        .layer(cors_middleware);
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await?;
    println!("Listening on {}", tcp_listener.local_addr()?);
    axum::serve(tcp_listener, app).await?;
    Ok(())
}
