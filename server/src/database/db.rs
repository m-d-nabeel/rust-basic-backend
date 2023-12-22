use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::Result;

pub async fn connect_db() -> Result<PgPool> {
    // let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=mysecretpassword", tokio_postgres::NoTls).await.unwrap();
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("database connect error {}", e);
    //     }
    // });
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:mysecretpassword@localhost/postgres")
        .await.unwrap();
    Ok(pool)
}