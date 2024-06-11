use crate::utils;

use super::store;
use derive_more::From;
use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize, Clone)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64},
    Store(store::Error),
    #[from(ignore)]
    Sqlx(String),
    ValidateFail(&'static str),
    Utils(utils::error::Error),
    #[from(ignore)]
    FieldNotFound(&'static str)
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}


impl std::error::Error for Error {}

// endregion: --- Error Boilerplate