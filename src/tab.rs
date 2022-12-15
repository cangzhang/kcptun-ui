use std::{collections::HashMap, path::PathBuf};

use imgui::{ListClipper, TabItem, Ui, WindowFlags};
use rfd::FileDialog;

use crate::instance::Instance;

pub fn make_config_tab(
    ui: &Ui,
    tab_index: u8,
    cur_dir: &PathBuf,
    config_map: &mut HashMap<u8, Instance>,
) {
    let order = tab_index + 1;
    let tab_name = format!("Config #{}", order);
    TabItem::new(tab_name).build(ui, || {
        let mut status_text = String::from("Please specify your config.");
        let has_config = config_map.contains_key(&tab_index);

        if has_config {
            if let Some(ins) = config_map.get_mut(&tab_index) {
                let running = ins.running.clone();
                let running = running.lock().unwrap();

                if !ins.path.is_empty() {
                    status_text = format!("Path: {}. Running: {}", ins.path, *running);
                }

                ui.text(&status_text);

                let select_text = if ins.path.is_empty() {
                    "Select"
                } else {
                    "Re-Select"
                };
                if ui.button(select_text) {
                    let f = FileDialog::new()
                        .add_filter("kcptun config", &["json"])
                        .set_directory(cur_dir)
                        .pick_file();
                    if let Some(f) = f {
                        let f = f.to_string_lossy().into_owned();
                        ins.kill();
                        ins.path = f;
                    }
                }

                ui.same_line();

                if !ins.path.is_empty() {
                    let remove_btn_text = format!("Remove Config #{order}");
                    if ui.button(&remove_btn_text) {
                        ins.kill();
                        ins.path = String::new();
                    }
                }

                ui.spacing();

                if *running {
                    if ui.button("Stop") {
                        ins.kill();
                    }
                } else if ui.button("Run") {
                    ins.run();
                }

                ui.spacing();
                ui.separator();

                let logs = ins.logs.read().unwrap();
                ui.child_window(ins.uid.to_string())
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
            }
        } else {
            ui.text(&status_text);
            if ui.button("Select") {
                let f = FileDialog::new()
                    .add_filter("kcptun config", &["json"])
                    .set_directory(cur_dir)
                    .pick_file();
                if let Some(f) = f {
                    let f = f.to_string_lossy().into_owned();
                    let mut ins = Instance::new();
                    ins.path = f;
                    config_map.insert(tab_index, ins);
                }
            }
        }
    });
}
