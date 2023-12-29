use axum::http::{HeaderName, Method};
use tower_http::cors::CorsLayer;

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(["http://localhost:5173".parse().unwrap()])
        .allow_methods([
            Method::GET,
            Method::POST,
            "content-type".parse::<Method>().unwrap(),
        ])
        .allow_credentials(true)
        .allow_headers([
            HeaderName::from_lowercase(b"authorization").unwrap(),
            HeaderName::from_lowercase(b"accept").unwrap(),
        ])
}
