pub mod core;
pub mod error;
pub mod simulate;
pub mod tray;

use crate::core::FastPaste;
use crate::error::InnerResult;
use crate::tray::create_systray;
use hotkey;
use hotkey::Listener;
use lazy_static::lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref FAST_PASTE: Mutex<FastPaste> = Mutex::new(FastPaste::new());
}

fn main() -> InnerResult<()> {
    //tray_item必须在程序结束前生命周期不结束
    let _tray_item = create_systray();

    let mut hk = hotkey::Listener::new();
    register_hotkey(&mut hk)?;
    hk.listen();

    Ok(())
}

fn register_hotkey(hotkey_listen: &mut Listener) -> InnerResult<()> {
    for c in '0'..='9' {
        hotkey_listen.register_hotkey(hotkey::modifiers::CONTROL, c as u32, move || {
            FAST_PASTE.lock().unwrap().paste(c).unwrap();
        })?;
    }

    for c in '0'..='9' {
        hotkey_listen.register_hotkey(
            hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
            c as u32,
            move || {
                FAST_PASTE.lock().unwrap().copy(c).unwrap();
            },
        )?;
    }

    Ok(())
}

//TODO 还需要调试
//
// fn hide_console() {
//     unsafe {
//         // Detach the current console if it exists
//         FreeConsole();
//     }
// }
