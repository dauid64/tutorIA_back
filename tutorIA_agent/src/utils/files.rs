use std::{fs::read_to_string, path::Path};
use crate::utils::error::{ Result, Error };

pub fn file_to_string(file: &Path) -> Result<String> {
    if !file.exists() {
        return Err(Error::FileNotFound(file.display().to_string()))
    }

    let content = read_to_string(file)
        .map_err(|err| Error::ReadFileError(err))?;
    
    Ok(content)
}

pub fn load_from_toml<T>(file: impl AsRef<Path>) -> Result<T>
    where 
        T: serde::de::DeserializeOwned
{
    let content = read_to_string(file.as_ref())?;

    Ok(toml::from_str(&content)?)
}
