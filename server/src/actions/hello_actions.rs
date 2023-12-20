use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HelloRouteParams {
    name: Option<String>,
}

pub async fn hello_world() -> impl IntoResponse {
    Html("<h1>Hello, World</h1>")
}

pub async fn hello_url_id(Path(id): Path<String>) -> impl IntoResponse {
    Html(format!("<h1>Hello, {}</h1>", id))
}

pub async fn hello_params(Query(params): Query<HelloRouteParams>) -> impl IntoResponse {
    Html(format!("<h1>Hello, {}</h1>", params.name.as_deref().unwrap_or("of_user")))
}