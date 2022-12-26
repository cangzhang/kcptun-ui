use glium::glutin::event_loop::EventLoopProxy;
use tray_item::TrayItem;

use crate::support::CustomEvent;

pub fn make_tray(ev_loop_proxy: EventLoopProxy<CustomEvent>) -> TrayItem {
    let mut tray = TrayItem::new("Kcptun UI", "main-icon").unwrap();

    let proxy = ev_loop_proxy.clone();
    let _ = tray.add_menu_item("Toggle", move || {
        let _r = proxy.send_event(CustomEvent::ToggleMainWindow);
    });

    let proxy = ev_loop_proxy.clone();
    let _ = tray.add_menu_item("Quit", move || {
        let _r = proxy.send_event(CustomEvent::Quit);
    });

    tray
}
