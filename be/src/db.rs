use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub type Db = Pool<Postgres>;

pub async fn init_pool() -> Result<Db, sqlx::Error> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPoolOptions::new().max_connections(5).connect(&url).await
}
