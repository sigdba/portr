use bollard::container::{
    AttachContainerOptions, Config, CreateContainerOptions, StartContainerOptions,
};
use bollard::Docker;
use std::default::Default;
use std::env;
use std::fs;
use std::path::PathBuf;
use tokio::runtime::Runtime;
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
    println!("{:?}", img);

    let docker =
        Docker::connect_with_local_defaults().expect("Failed to connect to local Docker daemon");
    let rt = Runtime::new().unwrap();

    // let version = rt
    //     .block_on(docker.version())
    //     .expect("Failed to retrieve Docker version");
    // println!("{:?}", version);

    rt.block_on(async {
        let options = Some(CreateContainerOptions { name: "portr" });

        let config = Config {
            image: Some(img),
            cmd: Some(vec!["/bin/bash", "-c", "ls", "-l"]),
            attach_stdin: Some(true),
            attach_stderr: Some(true),
            attach_stdout: Some(true),
            tty: Some(true),
            open_stdin: Some(true),
            ..Default::default()
        };

        docker
            .create_container(options, config)
            .await
            .expect("Failed to create container");
        match docker
            .start_container("portr", None::<StartContainerOptions<String>>)
            .await
        {
            Ok(_) => (),
            Err(msg) => panic!("Error starting container: {:?}", msg),
        };

        let options = Some(AttachContainerOptions::<String> {
            stdin: Some(true),
            stdout: Some(true),
            stderr: Some(true),
            stream: Some(true),
            logs: Some(true),
            detach_keys: Some("ctrl-c".to_string()),
        });

        docker
            .attach_container("portr", options)
            .await
            .expect("Failed to attach to container");
    });
}
