use axum::response::{IntoResponse, Response};

pub enum Error {
    NotFound,
    InternalServerError,
    BadRequest,
    Unauthorized,
    Forbidden,
    Conflict,
    UnprocessableEntity,
    TooManyRequests,
    ServiceUnavailable,
    GatewayTimeout,
    Other,
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => {
                Response::builder()
                    .status(404)
                    .body("Not Found".into())
                    .unwrap()
            }
            Error::InternalServerError => {
                Response::builder()
                    .status(500)
                    .body("Internal Server Error".into())
                    .unwrap()
            }
            Error::BadRequest => {
                Response::builder()
                    .status(400)
                    .body("Bad Request".into())
                    .unwrap()
            }
            Error::Unauthorized => {
                Response::builder()
                    .status(401)
                    .body("Unauthorized".into())
                    .unwrap()
            }
            Error::Forbidden => {
                Response::builder()
                    .status(403)
                    .body("Forbidden".into())
                    .unwrap()
            }
            Error::Conflict => {
                Response::builder()
                    .status(409)
                    .body("Conflict".into())
                    .unwrap()
            }
            Error::UnprocessableEntity => {
                Response::builder()
                    .status(422)
                    .body("Unprocessable Entity".into())
                    .unwrap()
            }
            Error::TooManyRequests => {
                Response::builder()
                    .status(429)
                    .body("Too Many Requests".into())
                    .unwrap()
            }
            Error::ServiceUnavailable => {
                Response::builder()
                    .status(503)
                    .body("Service Unavailable".into())
                    .unwrap()
            }
            Error::GatewayTimeout => {
                Response::builder()
                    .status(504)
                    .body("Gateway Timeout".into())
                    .unwrap()
            }
            Error::Other => {
                Response::builder()
                    .status(500)
                    .body("Internal Server Error".into())
                    .unwrap()
            }
        }
    }
}