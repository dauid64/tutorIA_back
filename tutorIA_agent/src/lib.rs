use std::path::Path;

use ais::create_assitant;
use async_openai::{config::OpenAIConfig, Client};
use config::config;
use utils::files::file_to_string;

use crate::error::Result;

mod ais;
mod config;
mod error;
mod tutor_ia;
mod utils;

const QUERY: &[(&str, &str)] = &[("limit", "1")];
const DIR: &str = "../tutorIA";

pub async fn create_TutorIA_assistant(thread_id: String) -> Result<()> {
    let dir: &Path = DIR.as_ref();
    let api_key = &config().openai_api_key;
    let model = &config().tutorIA_model;

    let config = OpenAIConfig::new().with_api_key(api_key);

    let client = Client::with_config(config);

    let assistant_name = "tutorIA";

    let instructions = file_to_string(dir.join("instructions.md").as_path())?;

    let assistant_id = create_assitant(client, assistant_name, instructions, model).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_tutoria_assistant_ok() -> Result<()> {
        create_TutorIA_assistant("teste".to_string()).await?;

        Ok(())
    }
}
