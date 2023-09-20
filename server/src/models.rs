use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct File {
    pub path: PathBuf,
    pub mode: Option<u32>,
}

#[derive(Deserialize)]
pub struct Toggle {
    pub toggle: bool,
}
