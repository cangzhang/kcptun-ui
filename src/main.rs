use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use slint::SharedString;

pub mod cmd;

fn main() {
    let ui = MainWindow::new();
    // let win = ui.window();

    let logs = String::new();
    let logs = Arc::new(Mutex::new(logs));

    let (tx, rx) = mpsc::channel();
    ui.on_start_cmd(move || {
        let tx = tx.clone();

        thread::spawn(move || {
            let _r = cmd::run(tx);
        });
    });

    let ui_handle = ui.as_weak();
    let handle = Arc::new(ui_handle);
    thread::spawn(move || {
        let ui = handle.lock().unwrap();

        loop {
            match rx.recv() {
                Ok(line) => {
                    println!("[rx] {line}");
                    let mut logs = logs.lock().unwrap();
                    logs.push_str(&line);

                    let mut s = SharedString::new();
                    s.push_str(&logs);
                    ui.set_logs(s);
                }
                Err(_) => {}
            }
        }
    });

    ui.run();
}

slint::slint! {
    import { Button, VerticalBox, ScrollView, TextEdit } from "std-widgets.slint";

    MainWindow := Window {
        property<string> win_title: "KCPTUN UI";
        property<string> logs: "";

        callback start-cmd();

        title: win_title;
        default-font-family: "Microsoft Yahei UI";
        width: 400px;
        preferred-height: 500px;

        VerticalBox {
            Text {
                text: "Hello World";
                font-weight: 500;
                font-size: 20px;
            }

            Button {
                text: "Start Kcptun";
                background: green;
                clicked => {
                    start-cmd();
                }
            }

            TextEdit {
                font-size: 10px;
                width: parent.width - 20px;
                height: parent.height * 50%;
                read-only: true;
                text <=> logs;
            }
        }
    }
}
