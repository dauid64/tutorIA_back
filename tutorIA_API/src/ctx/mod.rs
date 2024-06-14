mod error;

use uuid::Uuid;

pub use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: Uuid
}

impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: Uuid::nil() }
    }

    pub fn new(user_id: Uuid) -> Result<Self> {
        if user_id == Uuid::nil() {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

impl Ctx {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}