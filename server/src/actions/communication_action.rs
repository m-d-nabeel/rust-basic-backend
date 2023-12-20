use axum::{debug_handler, Json};
use axum::extract::State;
use axum::response::{Html, IntoResponse};

use crate::Result;
use crate::utils::model::{Message, MessageForCreate, ModelController};

pub async fn root_chatician() -> impl IntoResponse {
    Html("Hello, from chatician")
}


pub async fn post_chatician() -> impl IntoResponse {
    Html("Hello, from chatician")
}

#[debug_handler]
pub async fn create_message(
    State(mc): State<ModelController>,
    Json(message_fc): Json<MessageForCreate>,
) -> Result<Json<Message>> {
    println!("message_fc: {:?}", message_fc);
    let message_list = mc.create_message(message_fc).await.unwrap();
    Ok(Json(message_list))
}

pub async fn get_messages(State(mc): State<ModelController>) -> Result<Json<Vec<Message>>> {
    let message_list = mc.get_messages().await?;
    Ok(Json(message_list))
}