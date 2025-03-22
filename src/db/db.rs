use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

pub async fn connect() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Database URL expected");
    let pool = match PgPoolOptions::new()
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
    };
}
