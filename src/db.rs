use sqlx::mysql::MySqlPool;
use sqlx::Error;
use std::env;

pub async fn db_connection() -> Result<MySqlPool, Error> {
    let database_url = env::var("DATABASE_URL").expect("Required a database url");

    MySqlPool::new(&*database_url).await
}
