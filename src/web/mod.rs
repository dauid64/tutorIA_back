use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::crypt;

use self::error::Result;

pub mod routes;
pub mod middlewares;
mod error;