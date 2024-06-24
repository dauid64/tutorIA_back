use ais::{message::Message, OaClient};
use config::config;
use tutoria::TutorIA;

use crate::error::Result;

pub mod ais;
mod config;
pub mod error;
pub mod tutoria;
mod utils;

pub struct TutorIAContext {
    pub materia: String
}

pub async fn send_tutoria_message(oac: OaClient, tutoria: TutorIA) -> Result<TutorIA> {
    let tutoria = Message::send_message(oac, tutoria).await?;
    
    Ok(tutoria)
}