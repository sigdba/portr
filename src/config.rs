use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use std::fs;

use crate::util::ResultExt;

use crate::util;

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
    pub load_file: Option<String>,
    pub entrypoint: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Run {
    pub mount_pwd: Option<String>,

    #[serde(default)]
    pub docker_args: Vec<String>,
    #[serde(default)]
    pub child_args: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Cli {
    #[serde(default)]
    pub args: Vec<CliArg>,
}

#[derive(Deserialize, Default, Debug)]
pub struct CliArg {}

impl Config {
    pub fn new<'a>(config_path: &'a PathBuf) -> Result<Self, Box<dyn Error>> {
        let s = util::with_path(
            &config_path,
            fs::read_to_string,
            "Error reading config file",
        )?;
        Ok(toml::from_str(&s).wrap(format!(
            "Error parsing config: {}",
            &config_path.to_str().unwrap_or("<Unknown Path>")
        ))?)
    }
}
