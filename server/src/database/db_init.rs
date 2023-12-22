use sqlx::PgPool;

pub async fn db_initialization(pool: &PgPool) -> Result<(), sqlx::Error> {
    let queries = include_str!("./queries/create_tables.sql");
    sqlx::query(queries).execute(pool).await?;
    Ok(())
}