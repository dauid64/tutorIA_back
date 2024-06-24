use crate::{ais::error::Result, config, tutoria::config::Config};
use async_openai::{config::OpenAIConfig, types::{CreateAssistantRequestArgs, CreateThreadRequestArgs, ThreadObject}, Client};
use derive_more::{ From, Deref, Display };

pub mod error;
pub mod assistant;

pub type OaClient = Client<OpenAIConfig>;

#[derive(From, Debug, Deref, Display)]
pub struct AsstId(String);


pub fn new_oa_client() -> Result<OaClient> {
    let api_key = &config().openai_api_key;

    let config = OpenAIConfig::new().with_api_key(api_key);

    let oac = Client::with_config(config);
    
    Ok(oac)
}