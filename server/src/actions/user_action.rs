use axum::extract::State;
use axum::Json;

use crate::database::model::{ModelController, User, UserForCreate};

pub async fn get_users(State(mc): State<ModelController>) -> Json<Vec<User>> {
    let users = mc.get_users().await.unwrap();
    Json(users)
}


pub async fn create_user(State(mc): State<ModelController>,
                         Json(user_fc): Json<UserForCreate>) -> Json<User> {
    let created_user = mc.create_user(user_fc).await.unwrap();
    Json(created_user)
}