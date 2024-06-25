use ais::{message::Message, OaClient};
use config::config;
use tutoria::TutorIA;

use crate::error::Result;

pub mod ais;
mod config;
pub mod error;
pub mod tutoria;

pub struct TutorIAContext {
    pub materia: String
}

pub async fn send_tutoria_message(oac: OaClient, tutoria: TutorIA) -> Result<TutorIA> {
    let tutoria = Message::send_message(oac, tutoria).await?;
    
    Ok(tutoria)
}

#[cfg(test)]
mod tests {
    use ais::new_oa_client;

    use super::*;

    #[tokio::test]
    async fn send_message_ok() -> Result<()>{
        let oac = new_oa_client()?;

        let messages: Vec<Message> = [
            Message {
                content: "Você é um professor de matématica".to_string(),
                role: "system".to_string()
            },
            Message {
                content: "Olá".to_string(),
                role: "user".to_string()
            },
            Message {
                content: "Olá".to_string(),
                role: "assistant".to_string()
            },
            Message {
                content: "Quanto é 2+2?".to_string(),
                role: "user".to_string()
            },
            Message {
                content: "2+2 é igual a 4".to_string(),
                role: "assistant".to_string()
            },
            Message {
                content: "obrigado!".to_string(),
                role: "user".to_string()
            },
        ].to_vec();

        let tutoria = TutorIA::new(messages);

        let tutoria = send_tutoria_message(oac, tutoria).await?;

        println!("{:?}", tutoria.messages);

        Ok(())
    }
}