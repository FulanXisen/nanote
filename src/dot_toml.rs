use std::fs;

use anyhow::Result;

use serde_derive::{Deserialize, Serialize};

use crate::quick_page::QuickItem;
#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    name: String,
    version: String,
    author: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DotToml {
    pub path: String,
    pub application: Application,
    pub quick: Vec<QuickItem>,
}

pub fn read_total_from_dot_toml(path: &String) -> Result<DotToml> {
    let toml_str = fs::read_to_string(path)?;
    let dot_toml = toml::from_str(&toml_str)?;
    Ok(dot_toml)
}

pub fn write_total_to_dot_toml(dot_toml: &DotToml, path: &String) -> Result<()> {
    let toml_str = toml::to_string(dot_toml)?;
    fs::write(path, toml_str)?;
    Ok(())
}
