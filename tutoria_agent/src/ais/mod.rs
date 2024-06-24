use crate::{ais::error::Result, config};
use async_openai::{config::OpenAIConfig, Client};
use derive_more::{ From, Deref, Display };

pub mod error;
pub mod message;

pub type OaClient = Client<OpenAIConfig>;

#[derive(From, Debug, Deref, Display)]
pub struct AsstId(String);


pub fn new_oa_client() -> Result<OaClient> {
    let api_key = &config().openai_api_key;

    let config = OpenAIConfig::new().with_api_key(api_key);

    let oac = Client::with_config(config);

    Ok(oac)
}