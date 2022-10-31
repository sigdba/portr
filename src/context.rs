use crate::Config;
use std::path::PathBuf;

pub struct LaunchContext {
    pub config: Config,
    pub deploy_root: PathBuf, // TODO: Perhaps this should be a function which returns files?
    pub docker_args: Vec<String>,
    pub child_args: Vec<String>,
}

impl LaunchContext {
    pub fn new(conf: Config, deploy_root: PathBuf) -> Self {
        LaunchContext {
            config: conf,
            deploy_root: deploy_root,
            docker_args: Vec::new(),
            child_args: Vec::new(),
        }
    }
}

pub trait ArgVector {
    fn add_args<I, S>(&mut self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;
}

impl ArgVector for Vec<String> {
    fn add_args<I, S>(&mut self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for a in args {
            self.push(String::from(a.as_ref()));
        }
    }
}
