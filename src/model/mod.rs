pub use crate::model::error::{Result, Error};

use self::store::{new_db_pool, Db};

mod error;
mod store;
mod base;
pub mod aluno;
pub mod usuario;

#[derive(Clone)]
pub struct ModelManager {
    db: Db
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        Ok(ModelManager { db })
    }

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}