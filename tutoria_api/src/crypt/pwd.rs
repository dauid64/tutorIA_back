use crate::config::config;
use super::error::{Result, Error};

use super::encrypt_into_b64u;

pub fn encrypt_pwd(pwd_clear: &String) -> Result<String> {
    let key = &config().pwd_key;

    let encrypted = encrypt_into_b64u(&key, pwd_clear)?;

    Ok(encrypted)
}

pub fn validate_pwd(pwd_clear: &String, pwd_ref: &str) -> Result<()> {
    let pwd = encrypt_pwd(pwd_clear)?;

    if pwd == pwd_ref {
        Ok(())
    } else {
        Err(Error::PwdNotMacthing)
    }
}
