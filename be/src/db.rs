use sqlx::{MySql, Pool, Postgres, mysql::MySqlPoolOptions, postgres::PgPoolOptions};
use std::env;

pub type Db = Pool<Postgres>;
pub type VmailDb = Pool<MySql>;

pub async fn init_pool() -> Result<Db, sqlx::Error> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPoolOptions::new().max_connections(5).connect(&url).await
}

pub async fn init_vmail_pool() -> Result<VmailDb, sqlx::Error> {
    let url = env::var("VMAIL_DATABASE_URL")
        .map_err(|_| sqlx::Error::Configuration("VMAIL_DATABASE_URL not set".into()))?;
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
}
