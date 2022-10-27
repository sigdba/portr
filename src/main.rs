use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use toml::value::Table;
use toml::Value;

fn get_root<'a>() -> PathBuf {
    env::current_exe().unwrap().parent().unwrap().to_path_buf()
}

fn load_config() -> toml::value::Table {
    let config_path = get_root().join("portr.toml");

    println!("Loading config: {:?}", config_path);
    let ret = fs::read_to_string(&config_path)
        .expect("Failed to read config file")
        .parse::<Value>()
        .expect("Failed to parse config file");

    match ret {
        Value::Table(t) => t,
        _ => panic!("Invalid config file, not a TOML table"),
    }
}

fn get_config<'a, T>(
    conf: &'a Table,
    section: &str,
    key: &str,
    as_fn: fn(&'a Value) -> Option<T>,
) -> Option<T> {
    let o = conf
        .get(section)
        .expect(format!("configuration error: section not found: {}", section).as_str())
        .as_table()
        .expect(format!("configuration error: key is not a table: {}", section).as_str())
        .get(key);
    match o {
        Some(v) => as_fn(v),
        None => None,
    }
}

fn require_config<'a, T>(
    conf: &'a Table,
    section: &str,
    key: &str,
    as_fn: fn(&'a Value) -> Option<T>,
) -> T {
    get_config(conf, section, key, as_fn).expect(
        format!(
            "configuration error: missing required key '{}' in section '{}'",
            key, section
        )
        .as_str(),
    )
}

fn require_config_str<'a>(conf: &'a Table, section: &str, key: &str) -> &'a str {
    require_config(conf, section, key, |v| v.as_str())
}

fn main() {
    let conf = load_config();

    let img = require_config_str(&conf, "image", "name");
    println!("Image: {:?}", img);

    Command::new("cat")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn sub-process")
        .wait()
        .expect("Error waiting for sub-process");
}
