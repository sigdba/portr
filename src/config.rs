use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub image: Image,
    pub environment: Option<HashMap<String, String>>,

    #[serde(default)]
    pub cli: Cli,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub name: String,
    pub entrypoint: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Cli {
    pub args: Vec<CliArg>,
}

#[derive(Deserialize, Default, Debug)]
pub struct CliArg {}

impl Config {
    pub fn new<'a>(config_path: &'a PathBuf) -> Result<Self, Box<dyn Error>> {
        let s = fs::read_to_string(&config_path)?;
        Ok(toml::from_str(&s)?)
    }
}
