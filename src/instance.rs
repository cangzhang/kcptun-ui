use std::{
    path::PathBuf,
    process::Child,
    sync::{mpsc, Arc, Mutex, RwLock},
    thread,
};

use imgui::{ListClipper, TabItem, Ui, WindowFlags};
use rfd::FileDialog;
use uuid::Uuid;

use crate::cmd;

#[derive(Default, Debug)]
pub struct Instance {
    pub uid: Uuid,
    pub path: String,
    pub running: Arc<RwLock<bool>>,
    pub cmd: Arc<Mutex<Option<Child>>>,
    pub logs: Arc<RwLock<Vec<String>>>,
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
        // self.uid = Uuid::new_v4();
    }

    pub fn run(&mut self) {
        let (tx, rx) = mpsc::channel();
        let cmd = self.cmd.clone();
        let running = self.running.clone();
        let path = self.path.to_owned();

        thread::spawn(move || {
            let mut running = running.write().unwrap();
            if *running {
                println!("[instance::run] already running");
                return;
            }

            match cmd::run(&path, Some(tx)) {
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
        thread::spawn(move || loop {
            let mut write_guard = logs.write().unwrap();
            if let Ok((log_line, _pid)) = rx.try_recv() {
                println!("[receiver] {:?}", log_line);
                write_guard.push(log_line);
            }
            drop(write_guard);
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

            let mut running = running.write().unwrap();
            *running = false;
            let mut logs = logs.write().unwrap();
            *logs = vec![];
        });
    }

    pub fn update_config_path(&mut self, cur_dir: PathBuf) {
        let f = FileDialog::new()
            .add_filter("kcptun config", &["json"])
            .set_directory(cur_dir)
            .pick_file();
        if let Some(f) = f {
            self.kill();
            let f = f.to_string_lossy().into_owned();
            self.path = f;
        }
    }

    pub fn remove_config(&mut self) {
        self.kill();
        self.path = String::new();
    }

    pub fn toggle_status(&mut self) {
        let running = self.running.clone();
        println!("{:?}", running);
        let running = running.read().unwrap();
        if *running {
            self.kill();
        } else {
            self.run();
        }
    }

    pub fn make_tab_ui(
        &mut self,
        ui: &Ui,
        idx: &u8,
        cur_dir: PathBuf,
        on_config_change_cb: &dyn Fn(),
        on_remove_config: &dyn Fn(u8),
    ) {
        let order = idx + 1;
        let name = format!("#{order}");

        TabItem::new(&name).build(ui, || {
            let seq = idx + 1;
            let mut status_text = format!("[#{seq}] Please specify your config.");
            let has_config = true;

            if has_config {
                let running = self.running.clone();
                let running = running.read().unwrap();

                if !self.path.is_empty() {
                    status_text = format!("Path: {}. Running: {}", self.path, *running);
                }

                ui.text(&status_text);

                let select_text = if self.path.is_empty() {
                    "Select"
                } else {
                    "Re-Select"
                };
                if ui.button(select_text) {
                    self.update_config_path(cur_dir);
                    on_config_change_cb();
                }

                ui.same_line();

                if !self.path.is_empty() {
                    let remove_btn_text = format!("Remove Config #{order}");
                    if ui.button(&remove_btn_text) {
                        self.remove_config();
                        on_remove_config(*idx);
                        on_config_change_cb();
                    }
                }

                if !self.path.is_empty() {
                    if *running {
                        if ui.button("Stop") {
                            self.toggle_status();
                        }
                    } else if ui.button("Run") {
                        self.toggle_status();
                    }
                }

                ui.separator();

                let logs = self.logs.read().unwrap();
                ui.child_window(self.uid.to_string())
                    .flags(WindowFlags::HORIZONTAL_SCROLLBAR)
                    .build(|| {
                        if !logs.is_empty() {
                            let mut clipper = ListClipper::new(logs.len() as i32).begin(ui);
                            while clipper.step() {
                                for line in clipper.display_start()..clipper.display_end() {
                                    ui.text(&logs[line as usize]);
                                }
                            }
                        }
                        if ui.scroll_y() >= ui.scroll_max_y() {
                            ui.set_scroll_here_y();
                        }
                    });
            } else {
                ui.text(&status_text);
            }
        });
    }
}
