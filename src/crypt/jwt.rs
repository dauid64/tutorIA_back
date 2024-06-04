use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::config::config;

use super::error::{Result, Error};

#[derive(Serialize, Deserialize)]
pub struct Cliams {
    pub exp: usize,
    pub iat: usize,
    pub username: String,
    pub id: Uuid
}

pub fn encode_jwt(username: &str, id: Uuid) -> Result<String> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claim = Cliams{
        iat: now.timestamp() as usize,
        exp: (now+expire).timestamp() as usize,
        username: username.to_string(),
        id: id
    };
    let secret = &config().secret_jwt;

    return encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|err| Error::ErrorToEncodeJWT(err.to_string()))
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Cliams>> {
    let secret = &config().secret_jwt;
    let res: Result<TokenData<Cliams>> = decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map_err(|err| Error::ErrorToDecodeJWT(err.to_string()));
    
    res
}