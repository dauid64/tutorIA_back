use redis::{aio::ConnectionManager, Commands};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use redis::AsyncCommands;

pub use self::error::{Error, Result};
use crate::config::config;

mod error;

pub type Db = Pool<Postgres>;
pub type Rc = ConnectionManager;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config().db_url)
        .await
        .map_err(|err| Error::FailToCreatePool(err.to_string()))
}

#[derive(Serialize, Deserialize)]
struct MyData {
    field1: String,
    field2: i32,
}

pub async fn new_redis_pool() -> Result<Rc> {
    let rc_url = config().rc_url.as_str();

    let client = redis::Client::open(rc_url).map_err(|err: redis::RedisError| Error::FailToConnectRedis(err.to_string()))?;

    let con = ConnectionManager::new(client).await.map_err(|err: redis::RedisError| Error::FailToConnectRedis(err.to_string()))?;

    Ok(con)
}
