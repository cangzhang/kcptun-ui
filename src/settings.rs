use std::collections::BTreeMap;
use std::{fs, path::Path};

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::{instance::Instance, settings};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigFile {
    pub file_paths: Vec<String>,
    pub auto_launch_kcptun: bool,
}

#[derive(Default, Debug)]
pub struct State {
    pub configs: BTreeMap<u128, Instance>,
    pub auto_launch_kcptun: bool,
}

pub fn load_settings() -> State {
    let config_file_name = "./.config.toml";
    if !Path::new(config_file_name).exists() && fs::File::create(config_file_name).is_ok() {
        println!("[settings] created new config");
    }

    let mut configs: BTreeMap<u128, Instance> = BTreeMap::new();
    let mut auto_launch_kcptun = false;

    if let Ok(content) = fs::read_to_string(config_file_name) {
        println!("[settings] loaded");
        if let Ok(data) = toml::from_str::<ConfigFile>(&content) {
            auto_launch_kcptun = data.auto_launch_kcptun;

            for (_, c) in data.file_paths.iter().enumerate() {
                if !c.is_empty() {
                    let mut ins = Instance::new();

                    ins.update_config(c);
                    configs.insert(ins.uid, ins);
                }
            }

            if data.file_paths.is_empty() {
                configs.insert(0, Instance::new());
            }
        }
    } else {
        println!("[settings] load failed");
    }

    State {
        configs,
        auto_launch_kcptun,
    }
}

pub fn save(conf: &State) -> bool {
    let mut app_config = settings::ConfigFile {
        file_paths: vec![],
        auto_launch_kcptun: conf.auto_launch_kcptun,
    };

    for (_k, c) in &conf.configs {
        if !c.path.is_empty() {
            app_config.file_paths.push(c.path.to_owned());
        }
    }

    println!("[settings] {:?}", app_config);
    let data = toml::to_string_pretty(&app_config).unwrap();
    if fs::write("./.config.toml", data).is_ok() {
        return true;
    }

    false
}
