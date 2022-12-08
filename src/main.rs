#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{collections::HashMap, env};

use imgui::TabBar;

mod support;
mod tab;
mod tray;

// https://github.com/imgui-rs/imgui-rs/issues/669#issuecomment-1257644053
fn main() {
    let sys_tray = tray::make_tray();

    let cur_dir = env::current_dir().unwrap();

    let mut checked = false;
    let mut config_paths: HashMap<u8, String> = HashMap::new();

    let system = support::init();
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
                    if config_paths.is_empty() {
                        tab::make_config_tab(ui, 0, &cur_dir, &mut config_paths);
                        return;
                    }

                    for i in 0..config_paths.len() {
                        tab::make_config_tab(ui, i as u8, &cur_dir, &mut config_paths);
                    }
                });
            });
    });
}
