use home::home_dir;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub compile_command: String,
    pub compile_args: Vec<String>,
    pub contest_directory: String,
    pub cpp_template: String,
    pub port: i32,
}

pub fn load_config() -> Config {
    if let Some(home_path) = home_dir() {
        let mut config_path = PathBuf::from(home_path);
        config_path.push("cpx.json");

        if let Ok(data) = fs::read_to_string(config_path) {
            if let Ok(local_cfg) = serde_json::from_str(&data) {
                return local_cfg;
            }
        }
    }

    default_config()
}

fn default_config() -> Config {
    let default = Config {
        compile_command: "g++".into(),
        compile_args: vec![
            "solution.cpp".into(),
            "-o".into(),
            "cpp.out".into(),
            "-std=c++17".into(),
        ],
        contest_directory: {
            if let Some(dir) = home_dir() {
                format!("{}", dir.display())
            } else {
                "/".into()
            }
        },
        cpp_template: "".into(),
        port: 27121,
    };

    if let Some(home_path) = home_dir() {
        let mut config_path = PathBuf::from(home_path);
        config_path.push("cpx.json");

        // Clean this up later but it shouldn't matter too much I think
        let json = serde_json::to_string_pretty(&default).expect("Uh oh");
        let mut file = fs::File::create(&config_path).expect("Uh oh");

        match file.write_all(json.as_bytes()) {
            Ok(_) => println!("Writing default config to {}", config_path.display()),
            Err(_) => eprintln!(
                "Failed to write default config to {}",
                config_path.display()
            ),
        }
    }

    default
}
