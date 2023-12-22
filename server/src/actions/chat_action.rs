use axum::{debug_handler, Json};
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};

use crate::database::model::{Message, MessageForCreate, ModelController};
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
) -> Result<Json<Message>> {
    println!("message_fc: {:?}", message_fc);
    let created_message = mc
        .create_message(message_fc, channel_id)
        .await
        .unwrap();
    Ok(Json(created_message))
}

pub async fn get_messages(Path(channel_id): Path<i32>, State(mc): State<ModelController>) -> Result<Json<Vec<Message>>> {
    let message_list = mc.get_messages(channel_id).await.unwrap();
    Ok(Json(message_list))
}