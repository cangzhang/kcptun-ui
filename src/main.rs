#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    env,
    sync::{Arc, Mutex},
    thread,
};

use imgui::TabBar;
use instance::Instance;

mod cmd;
mod instance;
mod settings;
mod support;
mod tab;
mod tray;

// https://github.com/imgui-rs/imgui-rs/issues/669#issuecomment-1257644053
fn main() {
    let sys_tray = tray::make_tray();

    let cur_dir = env::current_dir().unwrap();
    let (app_conf, _tab_status) = settings::load_settings();
    let app_conf = Arc::new(Mutex::new(app_conf));

    let conf = app_conf.clone();
    thread::spawn(move || {
        let mut conf = conf.lock().unwrap();
        if !conf.auto_launch_kcptun {
            return;
        }

        for i in 0..conf.configs.len() {
            let i = i as u8;
            let ins = conf.configs.get_mut(&i).unwrap();
            if !ins.path.is_empty() {
                ins.run();
            }
        }
    });

    let conf_to_save = app_conf.clone();
    let save_conf = move || {
        let conf = conf_to_save.clone();
        thread::spawn(move || {
            let conf = conf.lock().unwrap();
            settings::save(&conf);
        });
    };

    let system = support::init();
    system.main_loop(move |_run, ui| {
        let _ = sys_tray;
        let mut state = app_conf.lock().unwrap();

        ui.window("Main")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(ui.io().display_size, imgui::Condition::Always)
            .no_decoration()
            .build(|| {
                // ui.text("KCPTUN UI");
                if ui.checkbox(
                    "Launch kcptun when starting app",
                    &mut state.auto_launch_kcptun,
                ) {
                    save_conf();
                }

                // ui.spacing();
                // ui.same_line();

                if ui.button("Add Config") {
                    let len = state.configs.len();
                    state.configs.insert(len as u8, Instance::new());
                    save_conf();
                }

                ui.separator();

                TabBar::new("AllTabs").build(ui, || {
                    for i in 0..state.configs.len() {
                        tab::make_config_tab(ui, i as u8, &cur_dir, &mut state.configs, &save_conf);
                    }
                });
            });
    });
}
