use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub name: String,
    pub model: String,
    pub instructions_file: String
}