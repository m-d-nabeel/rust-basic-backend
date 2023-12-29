use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;

#[derive(Clone, Debug)]
pub struct ModelController {
    pub db_pool: PgPool,
}

impl ModelController {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

#[derive(Debug, Deserialize, sqlx::FromRow, Clone, Serialize)]
pub struct Member {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub profile_pic: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct LoginMemberSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterMemberSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilteredMember {
    pub id: String,
    pub name: String,
    pub email: String,
    pub status: Option<String>,
    pub profile_pic: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub identity: Option<String>,
    pub sent_from_id: String,
    pub sent_to_id: String,
    pub message_text: String,
    pub status: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageToCreate {
    pub sent_from_id: String,
    pub sent_to_id: String,
    pub message_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
