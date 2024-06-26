use stores::{new_redis_pool, Rc};
use tutoria_agent::ais::{new_oa_client, OaClient};

pub use crate::manager::error::{Error, Result};

use self::stores::{new_db_pool, Db};

mod error;
mod stores;

#[derive(Clone)]
pub struct TutorIAManager {
    db: Db,
    rc: Rc,
    oac: OaClient,
}

impl TutorIAManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        let rc = new_redis_pool().await?;
        let oac = new_oa_client().map_err(|err| Error::OaError(err.to_string()))?;

        Ok(TutorIAManager { db, rc, oac })
    }

    pub fn db(&self) -> &Db {
        &self.db
    }

    pub async fn rc(&self) -> Rc {
        self.rc.clone()
    }

    pub fn oac(&self) -> &OaClient {
        &self.oac
    }
}
