use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct ModelController {
    pub db_pool: PgPool,
}

impl ModelController {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool
        }
    }
}
/***************************************************************/

/// Chat Functionalities
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub message_text: String,
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct MessageForCreate {
    pub user_id: i32,
    pub message_text: String,
}

impl ModelController {
    pub async fn create_message(&self, message_fc: MessageForCreate, channel_id: i32) -> Result<Message, sqlx::Error> {
        let inserted_message = sqlx::query_as!(
            Message,
            r#"INSERT INTO chatician.messages(user_id, message_text)
            VALUES ($1, $2)
            RETURNING id, user_id, message_text, created_at;"#,
            message_fc.user_id,
            message_fc.message_text
        )
            .fetch_one(&self.db_pool)
            .await?;

        let message_id = inserted_message.id.unwrap_or(0);

        sqlx::query!(
            "INSERT INTO chatician.channel_messages(channel_id, message_id)
            VALUES ($1, $2)",
            channel_id,
            message_id
        )
            .execute(&self.db_pool)
            .await?;

        Ok(inserted_message)
    }


    pub async fn get_messages(&self, channel_id: i32) -> Result<Vec<Message>, sqlx::Error> {
        let messages = sqlx::query_as!(
        Message,
        r#"SELECT m.id, m.user_id, m.message_text, m.created_at
        FROM chatician.messages AS m
        INNER JOIN chatician.channel_messages AS cm ON m.id = cm.message_id
        WHERE cm.channel_id = $1
        ORDER BY m.created_at DESC;"#,
        channel_id
    )
            .fetch_all(&self.db_pool)
            .await?;

        Ok(messages)
    }
}


/***************************************************************/

/// Channel Functionalities
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: Option<i32>,
    pub channel_name: Option<String>,
    pub description: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelForCreate {
    pub channel_name: String,
    pub description: String,
    pub created_by: i32,
}

impl ModelController {
    pub async fn create_channel(&self, channel_fc: ChannelForCreate) -> Result<Channel, sqlx::Error> {
        let inserted_channel = sqlx::query_as!(
            Channel,
            r#"INSERT INTO chatician.channels(channel_name, description, created_by)
            VALUES ($1, $2, $3)
            RETURNING id, channel_name,description,created_by,created_at;"#,
            channel_fc.channel_name,
            channel_fc.description,
            channel_fc.created_by
        )
            .fetch_one(&self.db_pool)
            .await?;
        Ok(inserted_channel)
    }
    pub async fn get_channels(&self) -> Result<Vec<Channel>, sqlx::Error> {
        let channels = sqlx::query_as!(
            Channel,
            r#"SELECT * FROM chatician.channels;"#
        ).fetch_all(&self.db_pool).await?;
        Ok(channels)
    }
}

/***************************************************************/
/// User Functionalities
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    created_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct UserForCreate {
    pub name: String,
    pub description: String,
}

impl ModelController {
    pub async fn create_user(&self, user_fc: UserForCreate) -> Result<User, sqlx::Error> {
        let inserted_user = sqlx::query_as!(
            User,
            r#"INSERT INTO chatician.users(name, description)
            VALUES ($1, $2)
            RETURNING id, name,description,created_at;"#,
            user_fc.name,
            user_fc.description
        )
            .fetch_one(&self.db_pool)
            .await?;
        Ok(inserted_user)
    }
    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"SELECT * FROM chatician.users;"#
        ).fetch_all(&self.db_pool).await?;
        Ok(users)
    }
}