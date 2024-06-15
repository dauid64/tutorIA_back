use async_openai::types::{CreateAssistantRequestArgs, CreateThreadRequestArgs, ThreadObject};
use super::error::Result;
use crate::tutoria::config::Config;

use super::{AsstId, OaClient};

pub async fn create_assitant(client: &OaClient, config: Config, instructions: String) -> Result<AsstId> {
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(config.name)
        .instructions(&instructions)
        .model(config.model)
        .build()?;

    let assistant = client.assistants().create(assistant_request).await?;

    let assistant_id = AsstId::from(assistant.id);

    Ok(assistant_id)
}

pub async fn create_thread(client: OaClient) -> Result<ThreadObject> {
    let thread_request = CreateThreadRequestArgs::default().build()?;

    let thread = client.threads().create(thread_request.clone()).await?;

    Ok(thread)
}