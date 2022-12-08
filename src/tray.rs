use tray_item::TrayItem;

pub fn make_tray() -> TrayItem {
    let mut tray = TrayItem::new("Kcptun UI", "main-icon").unwrap();
    
    tray.add_menu_item("Hello", || {
        println!("hello");
    })
    .unwrap();

    tray
}
