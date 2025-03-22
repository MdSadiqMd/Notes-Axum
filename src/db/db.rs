use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn connect() -> Pool<Postgres> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Database Url not found");
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Postgres Connection succesfull");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to Connect to Database: {:?}", err);
            std::process::exit(1)
        }
    }
}
