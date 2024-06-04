use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::crypt;

use self::error::Result;

pub mod routes;
pub mod middlewares;
mod error;

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, username: &str, id: Uuid) -> Result<()> {
    let token = crypt::jwt::encode_jwt(username, id)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}