use config::config;
use tutoria::TutorIA;

use crate::error::Result;

mod ais;
mod config;
mod error;
mod tutoria;
mod utils;

const DIR: &str = "../tutorIA";

pub struct TutorIAContext {
    pub materia: String
}

pub async fn create_tutoria_assistant(assistant_name: String, ctx: TutorIAContext) -> Result<TutorIA> {
    let tutoria = TutorIA::init_from_dir(DIR, assistant_name, ctx).await?;

    Ok(tutoria)
}

pub async fn delete_tutoria_assistant(tutoria: TutorIA) -> Result<()> {
    tutoria.delete().await?;

    Ok(())
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
