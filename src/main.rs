pub mod simulate;
pub mod core;

use std::borrow::BorrowMut;
use std::ffi::OsString;
use std::mem::size_of;
use std::ops::DerefMut;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use std::sync::Mutex;
use hotkey;
use hotkey::Listener;
use lazy_static::lazy_static;
use crate::core::FastPaste;

lazy_static! {
    static ref FAST_PASTE:Mutex<FastPaste> = Mutex::new(FastPaste::new());
}

fn main() {

    let mut hk = hotkey::Listener::new();
    register_hotkey(&mut hk);
    hk.listen();
}

fn register_hotkey(hotkey_listen:&mut Listener){
    for c in '0'..='9' {
        hotkey_listen.register_hotkey(
            hotkey::modifiers::CONTROL,
            c as u32,
            move || {
                FAST_PASTE.lock().unwrap().paste(c);
            },
        )
            .unwrap();
    }

    for c in '0'..='9' {
        hotkey_listen.register_hotkey(
            hotkey::modifiers::ALT,
            c as u32,
            move || {
                FAST_PASTE.lock().unwrap().copy(c);
            },
        )
            .unwrap();
    }
}