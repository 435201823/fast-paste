use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use tray_item::TrayItem;

pub fn create_systray() -> TrayItem {
    let mut tray = TrayItem::new("fast-paste", "my-icon-name").unwrap();

    tray.add_menu_item("退出", move || {
        exit(0);
    })
    .unwrap();

    tray
}
