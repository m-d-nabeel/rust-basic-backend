use axum::extract::State;
use axum::Json;

use crate::database::model::{Channel, ChannelForCreate, ModelController};

pub async fn get_channels(State(mc): State<ModelController>) -> Json<Vec<Channel>> {
    let channels = mc.get_channels().await.unwrap();
    Json(channels)
}


pub async fn create_channel(State(mc): State<ModelController>,
                            Json(channel_fc): Json<ChannelForCreate>) -> Json<Channel> {
    let created_channel = mc.create_channel(channel_fc).await.unwrap();
    Json(created_channel)
}