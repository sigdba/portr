use std::env;
use std::process;

use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use config::Config;
use context::LaunchContext;

mod config;
mod context;

fn get_root<'a>() -> PathBuf {
    env::current_exe().unwrap().parent().unwrap().to_path_buf()
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
    cmd.args(["-ti", "--rm"]);
    add_env_args(conf, cmd);
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let config_path = get_root().join("portr.toml");
    let mut ctx = LaunchContext {
        command: Command::new("docker"),
        config: Config::new(&config_path)?,
    };

    ctx.command
        .arg("run")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    add_docker_args(&ctx.config, &mut ctx.command);
    ctx.command.arg(get_image_name(&ctx.config));

    let res = ctx
        .command
        .spawn()
        .expect("Failed to spawn sub-process")
        .wait()
        .expect("Error waiting for sub-process")
        .code()
        .unwrap();

    process::exit(res);
}
