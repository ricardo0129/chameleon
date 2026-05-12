use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(flatten)]
    pub dotfiles: DotFileList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dotfile {
    pub source: String,
    pub description: Option<String>,
}

pub type DotFileList = HashSet<String>;
