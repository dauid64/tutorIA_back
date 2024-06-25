
use error::Result;

use crate::{ais::message::Message, config, TutorIAContext};

pub mod error;

#[derive(Clone)]
pub struct TutorIA {
    pub model: String,
    pub messages: Vec<Message>
}

impl TutorIA {
    pub fn new(messages: Vec<Message>) -> TutorIA {
        TutorIA {
            model: config().openai_model.clone(),
            messages: messages
        }
    }

    pub fn get_initial_system_msg(ctx: TutorIAContext) -> Result<Message> {
        let initial_message = Message {
            content: format!("Você é um professor particular de(a) {:?}", ctx.materia),
            role: "system".to_string()
        };

        Ok(initial_message)
    }

    pub fn add_message(&mut self, new_msg: Message) {
        self.messages.push(new_msg);
    }
}
