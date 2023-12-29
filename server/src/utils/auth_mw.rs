use axum::extract::{Request, State};
use axum::http::{HeaderName, StatusCode};
use axum::Json;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;

use crate::actions::auth_actions::CustomResult;
use crate::model::model::{TokenClaims, Member};
use crate::router::auth_router::AppState;

pub async fn auth(
    cookie_jar: CookieJar,
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> CustomResult<impl IntoResponse> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(HeaderName::from_lowercase(b"authorization").unwrap())
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or_else(|| {
        let json_error = json!({
            "status":"fail",
            "message":"You are not logged in, please provide token",
        });
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(b"my_ultra_secure_secret"),
        &Validation::default(),
    )
        .map_err(|_| {
            let json_error = json!( {
            "status": "fail",
            "message": "Invalid token",
        });
            (StatusCode::UNAUTHORIZED, Json(json_error))
        })?
        .claims;
    let member_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        let json_error = json!( {
            "status": "fail",
            "message": "Invalid token",
        });
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;
    let member = sqlx::query_as!(Member, "SELECT * FROM member WHERE id = $1", member_id)
        .fetch_optional(&app_state.mc.db_pool)
        .await
        .map_err(|e| {
            let json_error = json!( {
                "status": "fail",
                "message": format!("Error fetching member from database: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
        })?;

    let member = member.ok_or_else(|| {
        let json_error = json!( {
            "status": "fail",
            "message": "The member belonging to this token no longer exists",
        });
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    req.extensions_mut().insert(member);
    Ok(next.run(req).await)
}
