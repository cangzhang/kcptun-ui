use std::{collections::HashMap, path::PathBuf};

use imgui::{TabItem, Ui, StyleVar};
use rfd::FileDialog;

pub fn make_config_tab(
    ui: &Ui,
    tab_index: u8,
    cur_dir: &PathBuf,
    config_paths: &mut HashMap<u8, String>,
) {
    let order = tab_index + 1;
    let tab_name = format!("Config #{}", order);
    TabItem::new(tab_name).build(ui, || {
        let has_config = config_paths.contains_key(&tab_index);

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
                if let Some(_) = config_paths.get(&tab_index) {
                    *config_paths.get_mut(&tab_index).unwrap() = f;
                } else {
                    config_paths.entry(tab_index).or_insert(f);
                }
            }
        }

        let style = ui.push_style_var(StyleVar::FramePadding([0.0, 0.0]));
        ui.same_line();
        if let Some(el) = config_paths.get(&0) {
            ui.text(el);
        }
        style.pop();

        // ui.new_line();
        if has_config {
            let remove_btn_text = format!("Remove Config #{order}");
            if ui.button(&remove_btn_text) {
                config_paths.remove_entry(&tab_index);
            }
        }

        ui.spacing();
        ui.separator();
    });
}
