use std::fmt::Display;
use std::path::Path;

pub fn with_path<P: AsRef<Path> + Copy, R, E: Display>(
    path: P,
    f: fn(P) -> Result<R, E>,
    msg: &str,
) -> Result<R, String> {
    f(path).or_else(|e| {
        Err(format!(
            "{}: {}\n{}",
            msg,
            path.as_ref().to_str().unwrap_or("<path unknown>"),
            e
        ))
    })
}
