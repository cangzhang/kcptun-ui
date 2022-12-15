use std::{
    process::Child,
    sync::{mpsc, Arc, Mutex, RwLock},
    thread,
};

use uuid::Uuid;

use crate::cmd;

#[derive(Default, Debug)]
pub struct Instance {
    pub uid: Uuid,
    pub path: String,
    pub running: Arc<Mutex<bool>>,
    pub logs: Arc<RwLock<Vec<String>>>,
    pub cmd: Arc<Mutex<Option<Child>>>,
}

impl Instance {
    pub fn new() -> Self {
        Self {
            uid: Uuid::new_v4(),
            ..Default::default()
        }
    }
    
    pub fn update_config(&mut self, path: &String) {
        if path.eq(&self.path) {
            return;
        }

        self.path = path.to_owned();
        self.uid = Uuid::new_v4();
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

        let logs = self.logs.clone();
        thread::spawn(move || {
            loop {
                let mut write_guard = logs.write().unwrap();
                if let Ok((log_line, _pid)) = rx.try_recv() {
                    println!("[receiver] {:?}", log_line);
                    write_guard.push(log_line);
                }
                drop(write_guard);
            }
        });
    }

    pub fn kill(&mut self) {
        let cmd = self.cmd.clone();
        let running = self.running.clone();
        let logs = self.logs.clone();

        thread::spawn(move || {
            let mut cmd = cmd.lock().unwrap();
            if let Some(c) = &mut *cmd {
                let r = c.kill();
                println!("[instance::kill] {:?}", r);
            }
            *cmd = None;

            let mut running = running.lock().unwrap();
            *running = false;
            let mut logs = logs.write().unwrap();
            *logs = vec![];
        });
    }
}
