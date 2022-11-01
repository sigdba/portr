use std::fmt::Display;
use std::path::Path;

pub trait ResultExt<T, E> {
    fn wrap<M: Display>(self, msg: M) -> Result<T, String>;
}

impl<T, E: Display> ResultExt<T, E> for Result<T, E> {
    fn wrap<M: Display>(self, msg: M) -> Result<T, String> {
        self.or_else(|e| Err(format!("{}\n{}", msg, e)))
    }
}

pub fn with_path<P: AsRef<Path> + Copy, R, E: Display>(
    path: P,
    f: fn(P) -> Result<R, E>,
    msg: &str,
) -> Result<R, String> {
    f(path).wrap(format!(
        "{}: {}",
        msg,
        path.as_ref().to_str().unwrap_or("<path unknown>")
    ))
}
