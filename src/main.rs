#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    env, fs,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use imgui::TabBar;

mod config;
mod support;
mod tab;
mod tray;

// https://github.com/imgui-rs/imgui-rs/issues/669#issuecomment-1257644053
fn main() {
    let sys_tray = tray::make_tray();

    let cur_dir = env::current_dir().unwrap();
    let (config_paths, mut auto_launch_kcptun) = config::load_config();

    let config_map = Arc::new(Mutex::new(config_paths));

    let config_cloned = config_map.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        let config_map = config_cloned.lock().unwrap();
        println!("[current] {:?}", config_map);

        let mut app_config = config::ConfigFile {
            file_paths: vec![],
            auto_launch_kcptun,
        };
        for i in 0..config_map.len() {
            let idx = i as u8;
            let c = config_map.get(&idx).unwrap();
            app_config.file_paths.push(c.path.to_owned());
        }

        let data = toml::to_string_pretty(&app_config).unwrap();
        if let Ok(_) = fs::write("./.config.toml", data) {
            println!("[config] saved");
        }
    });

    let system = support::init();
    system.main_loop(move |_run, ui| {
        let _ = sys_tray;

        let mut config_map = config_map.lock().unwrap();

        ui.window("Main")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(ui.io().display_size, imgui::Condition::Always)
            .no_decoration()
            .build(|| {
                ui.text("KCPTUN UI");
                ui.spacing();

                ui.checkbox("Launch kcptun when starting app", &mut auto_launch_kcptun);
                ui.separator();

                TabBar::new("All Tabs").build(ui, || {
                    if config_map.is_empty() {
                        tab::make_config_tab(ui, 0, &cur_dir, &mut config_map);
                        return;
                    }

                    for i in 0..config_map.len() {
                        tab::make_config_tab(ui, i as u8, &cur_dir, &mut config_map);
                    }
                });
            });
    });
}
