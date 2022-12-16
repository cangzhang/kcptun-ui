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

    let conf_to_control = app_conf.clone();
    let batch_control = move |enable: bool| {
        let conf_to_control = conf_to_control.clone();
        thread::spawn(move || {
            match conf_to_control.try_lock() {
                Ok(mut conf) => {
                    for idx in 0..conf.configs.len() {
                        let idx = idx as u8;
                        let v = conf.configs.get_mut(&idx).unwrap();
                        if enable {
                            v.run();
                        } else {
                            v.kill();
                        }
                    }
                }
                Err(e) => {
                    println!("[batch_control] {:?}", e);
                }
            };
        });
    };

    let conf = app_conf.clone();
    let remove_config = move |idx: u8| {
        let conf = conf.clone();
        thread::spawn(move || {
            let mut conf = conf.lock().unwrap();
            if conf.configs.len() == 1 {
                let ins = conf.configs.get_mut(&idx).unwrap();
                *ins = Instance::new();
            } else {
                conf.configs.remove_entry(&idx);
            }
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
                if ui.checkbox(
                    "Launch kcptun when starting app",
                    &mut state.auto_launch_kcptun,
                ) {
                    save_conf();
                }

                if ui.button("Add Config") {
                    let len = state.configs.len();
                    state.configs.insert(len as u8, Instance::new());
                    save_conf();
                }

                ui.same_line();

                if ui.button("Start ALL") {
                    batch_control(true);
                }

                ui.same_line();

                if ui.button("Stop ALL") {
                    batch_control(false);
                }

                ui.separator();

                TabBar::new("AllTabs").build(ui, || {
                    for (idx, ins) in state.configs.iter_mut() {
                        ins.make_tab_ui(ui, idx, cur_dir.clone(), &save_conf, &remove_config);
                    }
                });
            });
    });
}
