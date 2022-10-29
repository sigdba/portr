use std::process::{Command, Stdio};

use crate::Config;

pub struct LaunchContext {
    pub command: Command,
    pub config: Config,
}
