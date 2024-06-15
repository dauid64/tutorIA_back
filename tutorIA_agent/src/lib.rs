use config::config;
use tutoria::TutorIA;

use crate::error::Result;

mod ais;
mod config;
mod error;
mod tutoria;
mod utils;

const DIR: &str = "../tutorIA";

pub async fn create_TutorIA_assistant(thread_id: String) -> Result<()> {
    let tutoria = TutorIA::init_from_dir(DIR).await?;

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
