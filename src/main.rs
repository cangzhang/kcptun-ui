#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    env,
    sync::{Arc, Mutex},
    thread,
};

use imgui::{TabBar, TabBarFlags};
use instance::Instance;

mod cmd;
mod instance;
mod settings;
mod support;
mod tray;

fn main() {
    let sys_tray = tray::make_tray();

    let cur_dir = env::current_dir().unwrap();
    let app_conf = settings::load_settings();
    let app_conf = Arc::new(Mutex::new(app_conf));

    let conf = app_conf.clone();
    thread::spawn(move || {
        let mut conf = conf.lock().unwrap();
        if !conf.auto_launch_kcptun {
            return;
        }

        conf.configs.iter_mut().for_each(|(_k, ins)| {
            if !ins.path.is_empty() {
                ins.run();
            }
        });
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
                    conf.configs.iter_mut().for_each(|(_k, ins)| {
                        let running = ins.running.clone();
                        let running = running.read().unwrap();

                        if !ins.path.is_empty() {
                            if enable {
                                if !*running {
                                    ins.run();
                                }
                            } else {
                                if *running {
                                    ins.kill();
                                }
                            }
                        }

                        drop(running);
                    });
                }
                Err(e) => {
                    println!("[batch_control] error: {:?}", e);
                }
            };
        });
    };

    let conf = app_conf.clone();
    let remove_config = move |k: u128| {
        let conf = conf.clone();
        thread::spawn(move || {
            let mut conf = conf.lock().unwrap();
            if conf.configs.len() == 1 {
                let ins = conf.configs.get_mut(&k).unwrap();
                *ins = Instance::new();
            } else {
                conf.configs.remove_entry(&k);
            }
        });
    };

    let conf = app_conf.clone();
    let system = support::init(conf);
    system.main_loop(move |_run, ui| {
        let _ = sys_tray;

        let mut state = app_conf.lock().unwrap();
        let tab_bar_flags = TabBarFlags::AUTO_SELECT_NEW_TABS;

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
                    let ins = Instance::new();
                    state.configs.insert(ins.uid, Instance::new());
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

                TabBar::new("AllTabs").flags(tab_bar_flags).build(ui, || {
                    let mut idx = 0;

                    for (_k, ins) in state.configs.iter_mut() {
                        idx += 1;
                        ins.make_tab_ui(ui, idx, cur_dir.clone(), &save_conf, &remove_config);
                    }
                });
            });
    });
}
