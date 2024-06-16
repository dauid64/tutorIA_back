use config::config;
use tutoria::{ctx::Context, TutorIA};

use crate::error::Result;

mod ais;
mod config;
mod error;
mod tutoria;
mod utils;

const DIR: &str = "../tutorIA";

pub async fn create_tutoria_assistant(assistant_name: String, ctx: Context) -> Result<TutorIA> {
    let tutoria = TutorIA::init_from_dir(DIR, assistant_name, ctx).await?;

    println!("{:?}", tutoria);

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
        let tutoria = create_tutoria_assistant("teste".to_string(), Context { materia: "Matematica".to_string()}).await?;

        delete_tutoria_assistant(tutoria).await?;

        Ok(())
    }
}
