use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{debug_handler, Json};
use sqlx::testing::TestTermination;

use crate::database::model::{Message, MessageForCreate, ModelController};
use crate::utils::errors::Error;
use crate::Result;

pub async fn root_chatician() -> impl IntoResponse {
    Html("Hello, from chatician")
}

pub async fn post_chatician() -> impl IntoResponse {
    Html("Hello, from chatician")
}

#[debug_handler]
pub async fn create_message(
    Path(channel_id): Path<i32>,
    State(mc): State<ModelController>,
    Json(message_fc): Json<MessageForCreate>,
) -> Result<StatusCode> {
    if message_fc.message_text.len() == 0 {
        return Err(Error::BadRequest);
    }
    println!("message_fc: {:?}", message_fc);
    match mc.create_message(message_fc, channel_id).await.is_success() {
        true => Ok(StatusCode::CREATED),
        false => Err(Error::InternalServerError),
    }
}

pub async fn get_messages(
    Path(channel_id): Path<i32>,
    State(mc): State<ModelController>,
) -> Result<Json<Vec<Message>>> {
    let message_list = mc.get_messages(channel_id).await.unwrap();
    Ok(Json(message_list))
}
