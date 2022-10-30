use crate::config::CliArg;
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

fn add_env_args(ctx: &mut LaunchContext) -> Result<(), Box<dyn Error>> {
    if let Some(env) = &ctx.config.environment {
        for (k, v) in env.iter() {
            ctx.docker_args
                .add_args(["-e", format!("{}={}", k, v).as_str()]);
        }
    }
    Ok(())
}

fn add_docker_args(ctx: &mut LaunchContext) -> Result<(), Box<dyn Error>> {
    ctx.docker_args.add_args(["-ti", "--rm"]);
    if let Some(e) = &ctx.config.image.entrypoint {
        ctx.docker_args.add_args(["--entrypoint", e.as_str()]);
    }
    if let Some(p) = &ctx.config.run.mount_pwd {
        ctx.docker_args.add_args([
            "-v",
            format!("{}:{}", &env::current_dir()?.to_str().unwrap(), p.as_str()).as_str(),
        ])
    }
    Ok(())
}

fn passthrough_args<'a>(
    ctx: &mut LaunchContext,
    arg: &'a str,
) -> Result<Option<&'a str>, Box<dyn Error>> {
    ctx.child_args.add_args([arg]);
    Ok(None)
}

type ConfHandler = fn(&mut LaunchContext) -> Result<(), Box<dyn Error>>;
static CONF_HANDLERS: &'static [ConfHandler] = &[add_docker_args, add_env_args];

type ArgHandler =
    for<'a> fn(&mut LaunchContext, &'a str) -> Result<Option<&'a str>, Box<dyn Error>>;
type ArgConfHandler = fn(&Config, &CliArg) -> Result<ArgHandler, Box<dyn Error>>;
static ARG_CONF_HANDLERS: &'static [ArgConfHandler] = &[];

pub fn run() -> Result<(), Box<dyn Error>> {
    let config_path = get_root().join("portr.toml");
    let mut ctx = LaunchContext::new(Config::new(&config_path)?);

    for h in CONF_HANDLERS {
        h(&mut ctx)?;
    }

    let mut arg_handlers: Vec<ArgHandler> = Vec::new();
    for arg_conf in &ctx.config.cli.args {
        for ach in ARG_CONF_HANDLERS {
            arg_handlers.push(ach(&ctx.config, &arg_conf)?)
        }
    }
    arg_handlers.push(passthrough_args);

    for mut arg in env::args().skip(1) {
        for hdl in &arg_handlers {
            match hdl(&mut ctx, &arg)? {
                None => break,
                Some(a) => arg = a.to_string(),
            }
        }
    }

    let res = Command::new("docker")
        .arg("run")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(ctx.docker_args)
        .arg(get_image_name(&ctx.config))
        .args(ctx.child_args)
        .spawn()?
        .wait()?
        .code()
        .unwrap();

    process::exit(res);
}
