use std::{
    process::Child,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::cmd;

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
        let cmd = self.cmd.clone();

        thread::spawn(move || {
            if let Ok(child) = cmd::run(Some(tx), 0) {
                let mut cmd = cmd.lock().unwrap();
                *cmd = Some(child);
            }
        });

        let log = self.log.clone();
        let running = self.running.clone();
        thread::spawn(move || loop {
            let mut log = log.lock().unwrap();
            for (log_line, pid, _idx) in rx.recv() {
                // println!("{log_line}");
                log.push_str(&log_line);
                *running.lock().unwrap() = pid > 0;
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
                println!("{:?}", r);
            }
            *cmd = None;

            let mut running = running.lock().unwrap();
            *running = false;
        });
    }
}
