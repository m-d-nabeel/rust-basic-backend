use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn connect_db() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:mysecretpassword@localhost/postgres")
        .await?;
    Ok(pool)
}