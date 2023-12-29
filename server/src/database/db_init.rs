use sqlx::PgPool;

pub async fn db_initialization(pool: &PgPool) -> Result<(), sqlx::Error> {
    let queries = include_str!("./queries/create_tables.sql");

    for query in queries.split(";") {
        if !query.trim().is_empty() {
            sqlx::query(query).execute(pool).await?;
        }
    }
    Ok(())
}