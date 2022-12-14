use std::{
    process::Child,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
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
        let running = self.running.clone();
        let log = self.log.clone();

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

        thread::spawn(move || {
            let mut log = log.lock().unwrap();

//            loop {
//                for r in rx.recv() {
//                    println!("[receiver] {:?}", r);
//                }
//
//                thread::sleep(Duration::from_secs(1));
//            }
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
