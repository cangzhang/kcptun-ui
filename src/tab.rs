use std::{collections::HashMap, path::PathBuf};

use imgui::{StyleVar, TabItem, Ui};
use rfd::FileDialog;

use crate::settings::Instance;

pub fn make_config_tab(
    ui: &Ui,
    tab_index: u8,
    cur_dir: &PathBuf,
    config_map: &mut HashMap<u8, Instance>,
) {
    let order = tab_index + 1;
    let tab_name = format!("Config #{}", order);
    TabItem::new(tab_name).build(ui, || {
        let has_config = config_map.contains_key(&tab_index);

        if !has_config {
            ui.text("Please specify your config");
        }

        if ui.button("Select") {
            let f = FileDialog::new()
                .add_filter("kcptun config", &["json"])
                .set_directory(cur_dir)
                .pick_file();
            if let Some(f) = f {
                let f = f.to_string_lossy().into_owned();
                if let Some(_) = config_map.get(&tab_index) {
                    *config_map.get_mut(&tab_index).unwrap() = Instance::new(&f);
                } else {
                    config_map.entry(tab_index).or_insert(Instance::new(&f));
                }
            }
        }

        let style = ui.push_style_var(StyleVar::FramePadding([0.0, 0.0]));
        ui.same_line();
        if let Some(el) = config_map.get(&0) {
            ui.text(&el.path);
        }
        style.pop();

        // ui.new_line();
        if has_config {
            let remove_btn_text = format!("Remove Config #{order}");
            if ui.button(&remove_btn_text) {
                config_map.remove_entry(&tab_index);
            }
        }

        ui.spacing();
        ui.separator();
    });
}
