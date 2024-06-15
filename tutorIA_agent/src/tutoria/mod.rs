use std::path::{Path, PathBuf};

use config::Config;

use crate::{ais::{assistand::create_assitant, new_oa_client, AsstId, OaClient}, tutoria::error::Result, utils::files::{file_to_string, load_from_toml}};

pub mod error;
pub mod config;

const INSTRUCTIONS: &str = "instructions.md";
const TUTORIA_TOML: &str = "tutorIA.toml";

pub struct TutorIA {
    dir: PathBuf,
    oac: OaClient,
    assistant_id: AsstId,
    config: Config
}

impl TutorIA {
    pub async fn init_from_dir(dir: impl AsRef<Path>) -> Result<Self> {
        let dir: &Path = dir.as_ref();

        let oac = new_oa_client()?;

        let config: Config = load_from_toml(dir.join(TUTORIA_TOML))?;

        let instructions = file_to_string(dir.join(&config.instructions_file).as_path())?;

        let assistant_id = create_assitant(&oac, config.clone(), instructions).await?;

        let tutoria = TutorIA {
            dir: dir.to_path_buf(),
            oac,
            config,
            assistant_id
        };

        Ok(tutoria)
    }
}