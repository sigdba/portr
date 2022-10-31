use crate::context::LaunchContext;
use std::error::Error;
use std::process::Command;

pub fn docker_command(_ctx: &LaunchContext) -> Result<Command, Box<dyn Error>> {
    Ok(Command::new("docker"))
}
