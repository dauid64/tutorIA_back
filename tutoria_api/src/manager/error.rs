use crate::utils;

use super::stores;
use derive_more::From;
use serde::Serialize;
use serde_with::serde_as;
use uuid::Uuid;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize, Clone)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: Uuid},
    Stores(stores::Error),
    ValidateFail(&'static str),
    Utils(utils::error::Error),
    #[from(ignore)]
    OaError(String),
    #[from(ignore)]
    RedisConnectionError(String)
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}


impl std::error::Error for Error {}

// endregion: --- Error Boilerplate