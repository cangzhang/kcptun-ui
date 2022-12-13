use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    pub file_paths: Vec<String>,
    pub auto_launch_kcptun: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    pub path: String,
    pub running: bool,
}

impl Instance {
    pub fn new(path: &String) -> Self {
        Self {
            path: path.to_owned(),
            running: false,
        }
    }
}

pub struct State {
    pub configs: HashMap<u8, Instance>,
    pub auto_launch_kcptun: bool,
}

pub fn load_settings() -> State {
    let config_file_name = "./.config.toml";
    if !Path::new(config_file_name).exists() {
        if let Ok(_) = File::create(config_file_name) {
            println!("[config] created new config");
        }
    }

    let mut configs: HashMap<u8, Instance> = HashMap::new();
    let mut auto_launch_kcptun = false;

    if let Ok(content) = fs::read_to_string(config_file_name) {
        println!("[config] loaded");
        match toml::from_str::<ConfigFile>(&content) {
            Ok(data) => {
                auto_launch_kcptun = data.auto_launch_kcptun;

                for (idx, c) in data.file_paths.iter().enumerate() {
                    configs.insert(idx as u8, Instance::new(c));
                }
            }
            Err(_) => {}
        }
    } else {
        println!("[config] load failed");
    }

    State {
        configs,
        auto_launch_kcptun,
    }
}
