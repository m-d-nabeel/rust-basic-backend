use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::actions::user_setup_action::CustomResult;
use crate::model::model::{ModelController, RealUser};

pub async fn get_users(State(mc): State<ModelController>) -> CustomResult<impl IntoResponse> {
    let users = sqlx::query_as!(RealUser, "SELECT * FROM real_user")
        .fetch_all(&mc.db_pool)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status":"fail",
                "message": format!("Database Error : {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    Ok(Json(users))
}
