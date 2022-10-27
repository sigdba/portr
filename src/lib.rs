use std::env;
use std::fs;
use std::process;

use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn get_root<'a>() -> PathBuf {
    env::current_exe().unwrap().parent().unwrap().to_path_buf()
}

#[derive(Deserialize)]
struct Config {
    image: Image,
    environment: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
struct Image {
    name: String,
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_path = get_root().join("portr.toml");
    let s = fs::read_to_string(&config_path)?;
    Ok(toml::from_str(&s)?)
}

fn get_image_name(conf: &Config) -> &str {
    // TODO: Add image building, etc.
    &conf.image.name
}

fn add_env_args(conf: &Config, cmd: &mut Command) {
    if let Some(env) = &conf.environment {
        for (k, v) in env.iter() {
            cmd.args(["-e", format!("{}={}", k, v).as_str()]);
        }
    }
}

fn add_docker_args(conf: &Config, cmd: &mut Command) {
    add_env_args(conf, cmd);
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let conf = load_config()?;
    let mut cmd = Command::new("docker");

    cmd.arg("run")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    add_docker_args(&conf, &mut cmd);
    cmd.arg(get_image_name(&conf));

    let res = cmd
        .spawn()
        .expect("Failed to spawn sub-process")
        .wait()
        .expect("Error waiting for sub-process")
        .code()
        .unwrap();

    // TODO: If we were interrupted we need to make sure the Docker process dies.

    process::exit(res);
}
