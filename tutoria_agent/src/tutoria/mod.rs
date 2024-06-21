use std::{
    path::{Path, PathBuf},
    vec,
};

use crate::config::config as config_env;
use async_openai::{config::OpenAIConfig, types::ThreadObject, Client};
use config::Config;
use regex::Regex;

use crate::{
    ais::{
        assistant::{self, create_assitant, delete_assistant},
        new_oa_client, AsstId, OaClient,
    },
    tutoria::error::Result,
    utils::files::{file_to_string, load_from_toml},
    TutorIAContext,
};

pub mod config;
pub mod error;

const TUTORIA_TOML: &str = "tutorIA.toml";

pub struct TutorIA {
    pub(crate) dir: PathBuf,
    pub(crate) oac: OaClient,
    pub assistant_id: AsstId,
    pub(crate) config: Config,
}

impl TutorIA {
    pub async fn new(
        assistant_id: AsstId,
        oac: Client<OpenAIConfig>,
        assistant_name: String,
    ) -> Result<TutorIA> {
        let dir: &Path = &config_env().dir.as_ref();

        let mut config: Config = load_from_toml(dir.join(TUTORIA_TOML))?;

        config.name = assistant_name;

        Ok(TutorIA {
            dir: dir.to_path_buf(),
            oac,
            assistant_id,
            config,
        })
    }

    pub async fn init_from_dir(assistant_name: String, ctx: TutorIAContext) -> Result<Self> {
        let dir: &Path = &config_env().dir.as_ref();

        let oac = new_oa_client()?;

        let mut config: Config = load_from_toml(dir.join(TUTORIA_TOML))?;
        config.name = assistant_name;

        let instructions = file_to_string(dir.join(&config.instructions_file).as_path())?;
        let instructions_with_ctx = TutorIA::get_instructions_with_ctx(instructions, ctx)?;

        let assistant_id = create_assitant(&oac, config.clone(), instructions_with_ctx).await?;

        let tutoria = TutorIA {
            dir: dir.to_path_buf(),
            oac,
            config,
            assistant_id,
        };

        Ok(tutoria)
    }

    fn get_instructions_with_ctx(mut instructions: String, ctx: TutorIAContext) -> Result<String> {
        let params = vec![("\\{materia\\}", ctx.materia)];

        for (param, val) in params {
            let re = Regex::new(param).unwrap();
            instructions = re.replace_all(&instructions, val).to_string();
        }

        Ok(instructions)
    }

    pub async fn delete(self) -> Result<()> {
        delete_assistant(&self).await?;

        drop(self);

        Ok(())
    }

    pub async fn create_thread(&self) -> Result<ThreadObject> {
        let thread = assistant::create_thread(&self.oac).await?;

        Ok(thread)
    }

    pub async fn send_message(&self, thread_id: &str, content: String) -> Result<String> {
        let response_msg = assistant::send_message(self, thread_id, content).await?;

        Ok(response_msg)
    }
}
