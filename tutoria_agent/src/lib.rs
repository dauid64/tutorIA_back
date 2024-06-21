use ais::{assistant::get_assistant, new_oa_client, AsstId};
use config::config;
use tutoria::TutorIA;

use crate::error::Result;

mod ais;
mod config;
pub mod error;
mod tutoria;
mod utils;

pub struct TutorIAContext {
    pub materia: String
}

pub async fn create_tutoria_assistant(assistant_name: String, ctx: TutorIAContext) -> Result<TutorIA> {
    let tutoria = TutorIA::init_from_dir(assistant_name, ctx).await?;

    Ok(tutoria)
}

pub async fn delete_tutoria_assistant(tutoria: TutorIA) -> Result<()> {
    tutoria.delete().await?;

    Ok(())
}

pub async fn create_tutoria_thread(tutoria: TutorIA) -> Result<String> {
    let thread = tutoria.create_thread().await?;

    Ok(thread.id)
}

pub async fn send_tutoria_message(tutoria: TutorIA, thread_id: &str, content: String) -> Result<String> {
    let response_msg = tutoria.send_message(thread_id, content).await?;

    Ok(response_msg)
}

pub async fn get_tutoria(assistant_id: &String) -> Result<TutorIA> {
    let client = new_oa_client()?;
    let assistant = get_assistant(&client, &assistant_id).await?;
    
    let tutoria = TutorIA::new(
        AsstId::from(assistant_id.to_string()),
        client,
        assistant.name.unwrap()
    ).await?;

    Ok(tutoria)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_tutoria_assistant_ok() -> Result<()> {
        let tutoria = create_tutoria_assistant("teste".to_string(), TutorIAContext { materia: "matematica".to_string()}).await?;

        delete_tutoria_assistant(tutoria).await?;

        Ok(())
    }
}
