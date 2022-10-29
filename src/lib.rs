use std::env;
use std::process;

use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use config::Config;
use context::ArgVector;
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

fn add_env_args(ctx: &mut LaunchContext) {
    if let Some(env) = &ctx.config.environment {
        for (k, v) in env.iter() {
            ctx.docker_args
                .add_args(["-e", format!("{}={}", k, v).as_str()]);
        }
    }
}

fn add_docker_args(ctx: &mut LaunchContext) {
    ctx.docker_args.add_args(["-ti", "--rm"]);
    add_env_args(ctx);
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let config_path = get_root().join("portr.toml");
    let mut ctx = LaunchContext::new(Config::new(&config_path)?);

    let mut cmd = Command::new("docker");

    cmd.arg("run")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    add_docker_args(&mut ctx);

    cmd.args(ctx.docker_args);
    cmd.arg(get_image_name(&ctx.config));
    cmd.args(ctx.child_args);

    let res = cmd
        .spawn()
        .expect("Failed to spawn sub-process")
        .wait()
        .expect("Error waiting for sub-process")
        .code()
        .unwrap();

    process::exit(res);
}
