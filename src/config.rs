use std::{env, sync::OnceLock};

use crate::error::{Error, Result};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

pub struct Config {
    pub port: String,
    pub db_url: String,
    pub secret_jwt: String,
    pub pwd_key: Vec<u8>,
    pub web_folder: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            port: get_env("SERVICE_PORT")?,
            db_url: get_env("DATABASE_URL")?,
            secret_jwt: get_env("SECRET_JWT")?,
            pwd_key: get_env_b64u_as_u8s("PWD_KEY")?,
            web_folder: get_env("WEB_FOLDER")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}
