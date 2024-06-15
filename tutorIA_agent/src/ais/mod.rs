use crate::ais::error::Result;
use async_openai::{config::OpenAIConfig, types::{CreateAssistantRequestArgs, CreateThreadRequestArgs, ThreadObject}, Client};

pub mod error;

pub struct AsstId(String);

pub async fn create_thread(client: Client<OpenAIConfig>) -> Result<ThreadObject> {
    let thread_request = CreateThreadRequestArgs::default().build()?;

    let thread = client.threads().create(thread_request.clone()).await?;

    Ok(thread)
}

pub async fn create_assitant(client: Client<OpenAIConfig>, name: &str, instructions: String, model: &String) -> Result<String> {
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(name)
        .instructions(&instructions)
        .model(model)
        .build()?;

    let assistant = client.assistants().create(assistant_request).await?;

    let assistant_id = assistant.id;

    Ok(assistant_id)
}