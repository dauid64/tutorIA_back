use std::path::PathBuf;

use crate::tutor_ia::error::Result;

pub mod error;
pub mod config;

pub struct TutorIA {
    dir: PathBuf
}