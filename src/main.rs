#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{collections::HashMap, env};

use imgui::{TabBar, TabItem};
use rfd::FileDialog;

mod support;
mod tray;

// https://github.com/imgui-rs/imgui-rs/issues/669#issuecomment-1257644053
fn main() {
    let sys_tray = tray::make_tray();

    let cur_dir = env::current_dir().unwrap();

    let mut checked = false;
    let mut config_paths: HashMap<u8, String> = HashMap::new();

    let system = support::init(file!());
    system.main_loop(move |_run, ui| {
        let _ = sys_tray;

        ui.window("Main")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(ui.io().display_size, imgui::Condition::Always)
            .no_decoration()
            .build(|| {
                ui.text("Here Is Some Text Sample 测试");
                ui.separator();
                ui.checkbox("This is an option", &mut checked);
                ui.separator();
                TabBar::new("All Tabs").build(ui, || {
                    TabItem::new("Tab A").build(ui, || {
                        ui.text("Please specify your config for kcptun A");
                        if ui.button("Select") {
                            let f = FileDialog::new()
                                .add_filter("kcptun config", &["json"])
                                .set_directory(&cur_dir)
                                .pick_file();
                            if let Some(f) = f {
                                let f = f.to_string_lossy().into_owned();
                                if let Some(_) = config_paths.get(&0) {
                                    *config_paths.get_mut(&0).unwrap() = f;
                                } else {
                                    config_paths.entry(0).or_insert(f);
                                }
                            }
                        }
                        if let Some(el) = config_paths.get(&0) {
                            ui.text(el);
                        }
                    });
                    TabItem::new("Tab B").build(ui, || {
                        ui.text("Please specify your config for kcptun B");
                        ui.button("Select");
                    });
                });
            });
    });
}
