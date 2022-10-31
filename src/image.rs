use std::error::Error;
use std::fs::File;
use std::process::Stdio;
use thiserror::Error;

use crate::docker;
use crate::util;

use crate::context::LaunchContext;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("Unexpected error {0} loading image")]
    ImageLoadFailed(i32),
}

fn load_image(ctx: &LaunchContext, path: &str) -> Result<(), Box<dyn Error>> {
    let full_path = ctx.deploy_root.join(path);
    let fp = util::with_path(&full_path, File::open, "Error opening image load file")?;
    let res = docker::docker_command(ctx)?
        .stdin(fp)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("load")
        .spawn()?
        .wait()?
        .code()
        .unwrap();
    if res != 0 {
        Err(Box::new(ImageError::ImageLoadFailed(res)))
    } else {
        Ok(())
    }
}

pub fn get_image_name(ctx: &LaunchContext) -> Result<&str, Box<dyn Error>> {
    if let Some(path) = &ctx.config.image.load_file {
        load_image(ctx, path.as_str())?;
    }

    Ok(&ctx.config.image.name)
}
