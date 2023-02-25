use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use hotkey;
use winapi::um::processthreadsapi::GetCurrentThreadId;
use winapi::um::winuser::{AttachThreadInput, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId};

fn main() {

    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
        '1' as u32,
        hotkey_callback,
    )
        .unwrap();

    hk.listen();
}

fn hotkey_callback() {
    println!("asdff");
    paste_text("asdf");
}

fn paste_text(text: &str) {
    // 将文本输入到当前焦点窗口中
    // 这里使用了第三方库 `winapi`
    use winapi::um::winuser::{GetForegroundWindow, SendMessageW};
    use winapi::shared::windef::HWND;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let hwnd: HWND = unsafe { get_current_editor_hwnd().unwrap() };
    let text: Vec<u16> = OsStr::new(text).encode_wide().chain(Some(0).into_iter()).collect();
    unsafe {
        SendMessageW(hwnd, winapi::um::winuser::WM_SETTEXT, 0, text.as_ptr() as isize);
    }
}

fn get_current_editor_hwnd() -> Option<winapi::shared::windef::HWND> {
    unsafe {
        let hwnd = GetForegroundWindow();

        let mut thread_id = GetCurrentThreadId();
        let this_thread_id = GetWindowThreadProcessId(hwnd, std::ptr::null_mut());
        AttachThreadInput(this_thread_id, thread_id, 1);

        let mut buf = vec![0u16; 256];
        let len = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
        buf.truncate(len as usize);

        let window_text = OsString::from_wide(&buf).to_string_lossy().to_string();
        println!("当前编辑框内容：{}" , window_text);
        Some(hwnd)
    }
}
