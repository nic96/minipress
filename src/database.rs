use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn setup_database_pool() -> DbPool {
    DbPool::connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap()
}
