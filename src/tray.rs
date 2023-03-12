use crate::InnerResult;
use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use tray_item::TrayItem;

pub fn create_systray() -> InnerResult<TrayItem> {
    let mut tray = TrayItem::new("fast-paste", "my-icon-name")?;

    tray.add_menu_item("退出", move || {
        exit(0);
    })?;

    Ok(tray)
}
