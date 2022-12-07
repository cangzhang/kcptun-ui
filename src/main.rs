#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use imgui::{TabBar, TabItem};

mod support;

fn main() {
    // https://github.com/imgui-rs/imgui-rs/issues/669#issuecomment-1257644053
    let mut checked = false;

    let system = support::init(file!());
    system.main_loop(move |run, ui| {
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
                        ui.text("Please specify your config for kcptun");
                        ui.button("Select");
                    });
                    TabItem::new("Tab B").build(ui, || {
                        ui.text("Please specify your config for kcptun");
                        ui.button("Select");
                    });
                });
            });
    });
}
