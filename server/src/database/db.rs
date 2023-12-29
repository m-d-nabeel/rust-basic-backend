use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn connect_db(db_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new().connect(db_url).await?;
    Ok(pool)
}
