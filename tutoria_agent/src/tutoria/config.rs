use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(default)]
    pub name: String,
    pub model: String,
    pub instructions_file: String
}