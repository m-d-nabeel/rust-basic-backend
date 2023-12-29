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
/***************************************************************/
//
// /// # Chat Functionalities
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Message {
//     pub id: uuid::Uuid,
//     pub user_id: uuid::Uuid,
//     pub name: Option<String>,
//     pub message_text: String,
//     pub created_at: Option<OffsetDateTime>,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct MessageForCreate {
//     pub user_id: uuid::Uuid,
//     pub message_text: String,
// }
//
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct MessageId {
//     pub id: uuid::Uuid,
// }
//
// impl ModelController {
//     pub async fn create_message(
//         &self,
//         message_fc: MessageForCreate,
//         channel_id: uuid::Uuid,
//     ) -> Result<(), sqlx::Error> {
//         let id_of_msg = sqlx::query_as!(
//             MessageId,
//             r#"INSERT INTO messages(user_id, message_text)
//             VALUES ($1, $2)
//             RETURNING id;"#,
//             message_fc.user_id,
//             message_fc.message_text
//         )
//         .fetch_one(&self.db_pool)
//         .await?;
//
//         sqlx::query!(
//             "INSERT INTO channel_messages(channel_id, message_id)
//             VALUES ($1, $2)",
//             channel_id,
//             id_of_msg.id
//         )
//         .execute(&self.db_pool)
//         .await?;
//
//         Ok(())
//     }
//
//     pub async fn get_messages(&self, channel_id: uuid::Uuid) -> Result<Vec<Message>, sqlx::Error> {
//         let messages = sqlx::query_as!(
//             Message,
//             r#"SELECT m.id, m.user_id, u.name, m.message_text, m.created_at
//         FROM messages AS m
//         INNER JOIN channel_messages AS cm ON m.id = cm.message_id
//         INNER JOIN users u on m.user_id = u.id
//         WHERE cm.channel_id = $1
//         ORDER BY m.created_at ASC;"#,
//             channel_id
//         )
//         .fetch_all(&self.db_pool)
//         .await?;
//
//         Ok(messages)
//     }
// }
//
// /***************************************************************/
//
// /// # Channel Functionalities
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Channel {
//     pub id: uuid::Uuid,
//     pub channel_name: Option<String>,
//     pub description: Option<String>,
//     pub created_by: Option<i32>,
//     pub created_at: Option<OffsetDateTime>,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct ChannelForCreate {
//     pub channel_name: String,
//     pub description: String,
//     pub created_by: i32,
// }
//
// impl ModelController {
//     pub async fn create_channel(
//         &self,
//         channel_fc: ChannelForCreate,
//     ) -> Result<Channel, sqlx::Error> {
//         let inserted_channel = sqlx::query_as!(
//             Channel,
//             r#"INSERT INTO channels(channel_name, description, created_by)
//             VALUES ($1, $2, $3)
//             RETURNING id, channel_name,description,created_by,created_at;"#,
//             channel_fc.channel_name,
//             channel_fc.description,
//             channel_fc.created_by
//         )
//         .fetch_one(&self.db_pool)
//         .await?;
//         Ok(inserted_channel)
//     }
//     pub async fn get_channels(&self) -> Result<Vec<Channel>, sqlx::Error> {
//         let channels = sqlx::query_as!(Channel, r#"SELECT * FROM channels;"#)
//             .fetch_all(&self.db_pool)
//             .await?;
//         Ok(channels)
//     }
// }
//
// /***************************************************************/
// /// # User Functionalities
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct User {
//     id: uuid::Uuid,
//     name: Option<String>,
//     description: Option<String>,
//     created_at: Option<OffsetDateTime>,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct UserForCreate {
//     pub name: String,
//     pub description: String,
// }
//
// impl ModelController {
//     pub async fn create_user(&self, user_fc: UserForCreate) -> Result<User, sqlx::Error> {
//         let inserted_user = sqlx::query_as!(
//             User,
//             r#"INSERT INTO users(name, description)
//             VALUES ($1, $2)
//             RETURNING id, name,description,created_at;"#,
//             user_fc.name,
//             user_fc.description
//         )
//         .fetch_one(&self.db_pool)
//         .await?;
//         Ok(inserted_user)
//     }
//     pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
//         let users = sqlx::query_as!(User, r#"SELECT * FROM users;"#)
//             .fetch_all(&self.db_pool)
//             .await?;
//         Ok(users)
//     }
// }
/******************************************************************************/

/// # Real User
/// **Authentication** Purpose
#[derive(Debug, Deserialize, sqlx::FromRow, Clone, Serialize)]
pub struct RealUser {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<OffsetDateTime>,
}
#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: Option<OffsetDateTime>,
}
#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
