mod actors;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:krishnaOp8@localhost/postgres")
        .await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(())
}
