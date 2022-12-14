use std::{
    process::Child,
    sync::{mpsc, Arc, Mutex, RwLock},
    thread,
};

use crate::cmd;

#[derive(Default, Debug)]
pub struct Instance {
    pub path: String,
    pub running: Arc<Mutex<bool>>,
    pub log: Arc<RwLock<String>>,
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
        let cmd = self.cmd.clone();
        let running = self.running.clone();

        thread::spawn(move || {
            let mut running = running.lock().unwrap();
            if *running {
                println!("[instance::run] already running");
                return;
            }

            match cmd::run(Some(tx)) {
                Ok(child) => {
                    let mut cmd = cmd.lock().unwrap();
                    *cmd = Some(child);
                    *running = true;
                }
                Err(e) => {
                    println!("[run] error {:?}", e);
                }
            }
        });

        let log = self.log.clone();
        thread::spawn(move || {
            loop {
                let mut write_guard = log.write().unwrap();
                if let Ok((log_line, _pid)) = rx.recv() {
                    println!("[receiver] {:?}", log_line);
                    write_guard.push_str(&log_line);
                }
                drop(write_guard);
            }
        });
    }

    pub fn kill(&mut self) {
        let cmd = self.cmd.clone();
        let running = self.running.clone();
        thread::spawn(move || {
            let mut cmd = cmd.lock().unwrap();
            if let Some(c) = &mut *cmd {
                let r = c.kill();
                println!("[instance::kill] {:?}", r);
            }
            *cmd = None;

            let mut running = running.lock().unwrap();
            *running = false;
        });
    }
}
