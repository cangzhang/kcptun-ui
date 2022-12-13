use std::{
    collections::HashMap,
    fs,
    path::Path,
    process::Child,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::{cmd, settings};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    pub file_paths: Vec<String>,
    pub auto_launch_kcptun: bool,
}

#[derive(Default, Debug)]
pub struct Instance {
    pub path: String,
    pub running: Arc<Mutex<bool>>,
    pub log: Arc<Mutex<String>>,
    pub pid: u32,
    pub cmd: Arc<Mutex<Option<Child>>>,
}

impl Instance {
    pub fn new(path: &String) -> Self {
        Self {
            path: path.to_owned(),
            ..Default::default()
        }
    }

    pub fn run(&mut self) {
        let (tx, rx) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        thread::spawn(move || {
            if let Ok(child) = cmd::run(Some(tx), 0) {
                let _ = tx2.send(child);
            }
        });

        let cmd = self.cmd.clone();
        thread::spawn(move || {
            let mut cmd = cmd.lock().unwrap();
            loop {
                for c in rx2.recv() {
                    *cmd = Some(c);
                }
            }
        });

        let log = self.log.clone();
        let running = self.running.clone();
        thread::spawn(move || loop {
            let mut log = log.lock().unwrap();
            let mut running = running.lock().unwrap();
            for (log_line, pid, _idx) in rx.recv() {
                println!("{log_line}");
                log.push_str(&log_line);
                *running = pid > 0;
            }
        });
    }

    pub fn kill(&mut self) {
        let mut cmd = self.cmd.lock().unwrap();
        let c = cmd.as_mut();
        if let Some(c) = c {
            let r = c.kill();
            println!("[Instance::kill] {:?}", r);
        }

        *cmd = None;
        //        self.running = false;
    }
}

pub struct State {
    pub configs: HashMap<u8, Instance>,
    pub auto_launch_kcptun: bool,
}

pub fn load_settings() -> State {
    let config_file_name = "./.config.toml";
    if !Path::new(config_file_name).exists() {
        if let Ok(_) = fs::File::create(config_file_name) {
            println!("[settings] created new config");
        }
    }

    let mut configs: HashMap<u8, Instance> = HashMap::new();
    let mut auto_launch_kcptun = false;

    if let Ok(content) = fs::read_to_string(config_file_name) {
        println!("[settings] loaded");
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
    for i in 0..conf.configs.len() {
        let idx = i as u8;
        let c = conf.configs.get(&idx).unwrap();
        app_config.file_paths.push(c.path.to_owned());
    }

    println!("[current app config] {:?}", app_config);
    let data = toml::to_string_pretty(&app_config).unwrap();
    if let Ok(_) = fs::write("./.config.toml", data) {
        return true;
    }

    false
}

pub fn auto_save(conf: Arc<Mutex<State>>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        let r = save(&conf.lock().unwrap());
        println!("[settings::auto_save] {r}");
    });
}
