use crate::model;
use derive_more::From;
use tutoria_agent::ais;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from(ignore)]
    ConfigMissingEnv(&'static str),
    #[from(ignore)]
    ConfigWrongFormat(&'static str),
    Model(model::Error),
    OaError(ais::error::Error)
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate