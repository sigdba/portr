use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub image: Image,
    pub run: Run,
    pub environment: Option<HashMap<String, String>>,

    #[serde(default)]
    pub cli: Cli,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub name: String,
    pub entrypoint: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Run {
    pub mount_pwd: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Cli {
    pub args: Vec<CliArg>,
}

#[derive(Deserialize, Default, Debug)]
pub struct CliArg {}

impl Config {
    pub fn new<'a>(config_path: &'a PathBuf) -> Result<Self, Box<dyn Error>> {
        let s = fs::read_to_string(&config_path).or_else(|e| {
            Err(format!(
                "Error reading config file: {}\n{}",
                config_path.to_str().unwrap_or("<path unknown>"),
                e
            ))
        })?;
        Ok(toml::from_str(&s)?)
    }
}
