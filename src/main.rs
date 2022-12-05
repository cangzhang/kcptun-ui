// slint::include_modules!();

use std::thread;

use slint::SharedString;

pub mod cmd;

fn main() {
    let ui = MainWindow::new();
    // let win = ui.window();

    let ui_handle = ui.as_weak();
    ui.on_update_title(move || {
        let ui = ui_handle.unwrap();

        let mut text = SharedString::new();
        text.push_str("updated");
        ui.set_win_title(text);
    });

    ui.on_start_cmd(move || {
        thread::spawn(|| {
            cmd::run();
        });
    });

    ui.run();
}

slint::slint! {
    import { Button, VerticalBox , CheckBox } from "std-widgets.slint";

    MainWindow := Window {
        property<string> win_title: "KCPTUN UI";
        callback update-title();
        callback start-cmd();

        title: win_title;
        default-font-family: "Microsoft Yahei UI";
        preferred-width: 260px;
        preferred-height: 100px;

        VerticalBox {
            Text {
                text: "Hello World";
                font-weight: 500;
                font-size: 20px;
            }

            Button {
                text: "Update Title";
                clicked => {
                    update-title();
                }
            }

            Button {
                text: "Start Kcptun";
                background: green;
                clicked => {
                    start-cmd();
                }
            }
        }
    }
}
