use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::actions::auth_actions::CustomResult;
use crate::model::model::{ModelController, Member};

pub async fn get_members(State(mc): State<ModelController>) -> CustomResult<impl IntoResponse> {
    let members = sqlx::query_as!(Member, "SELECT * from member")
        .fetch_all(&mc.db_pool)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status":"fail",
                "message": format!("Database Error : {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    Ok(Json(members))
}
