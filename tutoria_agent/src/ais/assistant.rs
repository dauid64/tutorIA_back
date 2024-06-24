use async_openai::types::{AssistantObject, CreateAssistantRequestArgs, CreateMessageRequestArgs, CreateRunRequestArgs, CreateThreadRequestArgs, MessageContent, MessageRole, RunStatus, ThreadObject};
use super::error::{ Result, Error };
use crate::tutoria::{config::Config, TutorIA};

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

pub async fn get_assistant(client: &OaClient, assistant_id: &String) -> Result<AssistantObject> {
    let assistant = client.assistants().retrieve(assistant_id.as_str()).await?;

    Ok(assistant)
}

pub async fn create_thread(client: &OaClient) -> Result<ThreadObject> {
    let thread_request = CreateThreadRequestArgs::default().build()?;

    let thread = client.threads().create(thread_request.clone()).await?;

    Ok(thread)
}

pub async fn delete_assistant(tutoria: &TutorIA) -> Result<()> {
    let assistant_id = tutoria.assistant_id.as_str();
    tutoria.oac.assistants().delete(assistant_id).await?;

    Ok(())
}

pub async fn send_message(tutoria: &TutorIA, thread_id: &str, content: String) -> Result<String> {
    let query = [("limit", "1")];
    
    let message = CreateMessageRequestArgs::default()
            .role(MessageRole::User)
            .content(content)
            .build()?;

    let _message_obj = tutoria.oac
        .threads()
        .messages(thread_id)
        .create(message)
        .await?;
    
    let run_request = CreateRunRequestArgs::default()
        .assistant_id(tutoria.assistant_id.to_string())
        .parallel_tool_calls(false)
        .build()?;

    let run = tutoria.oac
        .threads()
        .runs(thread_id)
        .create(run_request)
        .await?;

    let mut awaiting_response = true;

    while awaiting_response {
        let run = tutoria.oac.threads().runs(thread_id).retrieve(&run.id).await?;

        match run.status {
            RunStatus::Completed => {
                awaiting_response = false;

                let response = tutoria.oac.threads().messages(thread_id).list(&query).await?;

                let message_id = response.data.first().unwrap().id.clone();

                let message = tutoria.oac
                    .threads()
                    .messages(thread_id)
                    .retrieve(&message_id)
                    .await?;

                let content = message.content.first().unwrap();

                let text = match content {
                    MessageContent::Text(text) => text.text.value.clone(),
                    MessageContent::ImageFile(_) | MessageContent::ImageUrl(_) => {
                        panic!("imaged are not expected in this example");
                    }
                };

                return Ok(text)
            },
            RunStatus::Queued | RunStatus::InProgress => (),
            _other => {
                awaiting_response = false;
                return Err(Error::SendMessageOpenAIError)
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    Err(Error::SendMessageOpenAIError)
}