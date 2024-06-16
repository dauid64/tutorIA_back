use crate::{ais::error::Result, config, tutoria::config::Config};
use async_openai::{config::OpenAIConfig, types::{CreateAssistantRequestArgs, CreateThreadRequestArgs, ThreadObject}, Client};
use derive_more::From;

pub mod error;
pub mod assistant;

pub type OaClient = Client<OpenAIConfig>;

#[derive(From, Debug)]
pub struct AsstId(String);

impl AsstId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}


pub fn new_oa_client() -> Result<OaClient> {
    let api_key = &config().openai_api_key;

    let config = OpenAIConfig::new().with_api_key(api_key);

    let oac = Client::with_config(config);
    
    Ok(oac)
}