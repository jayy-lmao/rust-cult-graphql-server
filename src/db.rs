use sqlx::postgres::PgPool;
use sqlx::Error;
use std::env;

pub async fn db_connection() -> Result<PgPool, Error> {
    let database_url = env::var("DATABASE_URL").expect("Required a database url");
    dbg!(database_url.clone());

    PgPool::new(&*database_url).await
}
