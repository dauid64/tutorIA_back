use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs
};
use serde::Serialize;

use super::error::{Error, Result};
use crate::{ais::OaClient, tutoria::TutorIA};

#[derive(Clone, Serialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub async fn send_message(oac: &OaClient, tutoria: TutorIA) -> Result<TutorIA> {
        let mut messages_formatted = Vec::new();
        let messages = tutoria.messages.clone().into_iter();

        for message in messages {
            if message.role == "assistant" {
                let message_formatted = ChatCompletionRequestAssistantMessageArgs::default()
                    .content(message.content)
                    .build()?
                    .into();
                messages_formatted.push(message_formatted);
            } else if message.role == "user" {
                let message_formatted = ChatCompletionRequestUserMessageArgs::default()
                    .content(message.content)
                    .build()?
                    .into();
                messages_formatted.push(message_formatted);
            } else if  message.role == "system" {
                let message_formatted = ChatCompletionRequestSystemMessageArgs::default()
                    .content(message.content)
                    .build()?
                    .into();
                messages_formatted.push(message_formatted);
            } else {
                return Err(Error::NoRoleDefined)
            }
        }

        let request = CreateChatCompletionRequestArgs::default()
            .model(tutoria.model.clone())
            .messages(messages_formatted)
            .build()?;

        let response = oac.chat().create(request).await?;

        let response_msg = Message {
            role: response.choices[0].message.role.to_string(),
            content: response.choices[0]
                .message
                .content
                .clone()
                .unwrap_or("".to_string()),
        };

        let mut new_tutoria = tutoria.clone();
        new_tutoria.add_message(response_msg);

        Ok(new_tutoria)
    }
}
