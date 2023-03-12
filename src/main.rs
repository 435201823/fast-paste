pub mod core;
pub mod error;
pub mod simulate;
pub mod tray;

use crate::core::FastPaste;
use crate::tray::create_systray;
use hotkey;
use hotkey::Listener;
use lazy_static::lazy_static;
use std::borrow::BorrowMut;
use std::ffi::OsString;
use std::fs::File;
use std::mem::size_of;
use std::ops::DerefMut;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use std::sync::{mpsc, Mutex};
use tray_item::TrayItem;
use winapi::um::wincon::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS};

lazy_static! {
    static ref FAST_PASTE: Mutex<FastPaste> = Mutex::new(FastPaste::new());
}

fn main() {
    hide_console();

    //tray_item必须在程序结束前生命周期不结束
    let _tray_item = create_systray();

    let mut hk = hotkey::Listener::new();
    register_hotkey(&mut hk);
    hk.listen();
}

fn register_hotkey(hotkey_listen: &mut Listener) {
    for c in '0'..='9' {
        hotkey_listen
            .register_hotkey(hotkey::modifiers::CONTROL, c as u32, move || {
                FAST_PASTE.lock().unwrap().paste(c);
            })
            .unwrap();
    }

    for c in '0'..='9' {
        hotkey_listen
            .register_hotkey(
                hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
                c as u32,
                move || {
                    FAST_PASTE.lock().unwrap().copy(c);
                },
            )
            .unwrap();
    }
}

fn hide_console() {
    unsafe {
        // Detach the current console if it exists
        FreeConsole();

        // // Allocate a new console
        // AllocConsole();
        //
        // // Attach the new console to the parent process (i.e. this process)
        // AttachConsole(ATTACH_PARENT_PROCESS);
    }
}
