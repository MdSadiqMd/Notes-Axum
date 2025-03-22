use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn connect() -> Pool<Postgres> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Database URL not found");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ PostgreSQL connection successful");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => {
            println!("✅ Migrations applied successfully");
        }
        Err(e) => {
            println!("🔥 Failed to apply migrations: {:?}", e);
            std::process::exit(1);
        }
    }

    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => {
            println!("✅ Database connection verified");
        }
        Err(e) => {
            println!("🔥 Database connection test failed: {:?}", e);
            std::process::exit(1);
        }
    }

    pool
}
