use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::config;
pub use self::error::{Result, Error};

mod error;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config().db_url)
        .await
        .map_err(|err| Error::FailToCreatePool(err.to_string()))
}