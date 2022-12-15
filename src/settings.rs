use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use serde_derive::Deserialize;
use serde_derive::Serialize;
use uuid::Uuid;

use crate::{instance::Instance, settings};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigFile {
    pub file_paths: Vec<String>,
    pub auto_launch_kcptun: bool,
}

pub struct State {
    pub configs: HashMap<u8, Instance>,
    pub auto_launch_kcptun: bool,
}

pub fn load_settings() -> (State, HashMap<Uuid, bool>) {
    let config_file_name = "./.config.toml";
    if !Path::new(config_file_name).exists() {
        if fs::File::create(config_file_name).is_ok() {
            println!("[settings] created new config");
        }
    }

    let mut configs: HashMap<u8, Instance> = HashMap::new();
    let mut auto_launch_kcptun = false;

    let mut tab_status = HashMap::new();

    if let Ok(content) = fs::read_to_string(config_file_name) {
        println!("[settings] loaded");
        if let Ok(data) = toml::from_str::<ConfigFile>(&content) {
            auto_launch_kcptun = data.auto_launch_kcptun;

            for (idx, c) in data.file_paths.iter().enumerate() {
                if !c.is_empty() {
                    let mut ins = Instance::new();
                    let uid = ins.uid.clone();

                    ins.update_config(c);
                    configs.insert(idx as u8, ins);
                    tab_status.insert(uid, true);
                }
            }

            if data.file_paths.is_empty() {
                configs.insert(0, Instance::new());
            }
        }
    } else {
        println!("[settings] load failed");
    }

    (
        State {
            configs,
            auto_launch_kcptun,
        },
        tab_status,
    )
}

pub fn save(conf: &State) -> bool {
    let mut app_config = settings::ConfigFile {
        file_paths: vec![],
        auto_launch_kcptun: conf.auto_launch_kcptun,
    };
    for i in 0..conf.configs.len() {
        let idx = i as u8;
        if let Some(c) = conf.configs.get(&idx) {
            if !c.path.is_empty() {
                app_config.file_paths.push(c.path.to_owned());
            }
        }
    }

    println!("[current app config] {:?}", app_config);
    let data = toml::to_string_pretty(&app_config).unwrap();
    if fs::write("./.config.toml", data).is_ok() {
        return true;
    }

    false
}

#[allow(dead_code)]
pub fn auto_save(conf: Arc<Mutex<State>>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        let r = save(&conf.lock().unwrap());
        println!("[settings::auto_save] {r}");
    });
}
