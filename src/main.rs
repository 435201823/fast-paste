use std::ffi::OsString;
use std::mem::size_of;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use hotkey;
use winapi::shared::minwindef::{DWORD, LPARAM, UINT, WPARAM};
use winapi::shared::windef::POINT;
use winapi::um::processthreadsapi::GetCurrentThreadId;
use winapi::um::winbase::{GlobalAlloc, GlobalLock, GlobalUnlock};
use winapi::um::winuser;
use winapi::um::winuser::{AttachThreadInput, CF_UNICODETEXT, CloseClipboard, EM_GETSEL, EmptyClipboard, GetDlgItemTextW, GetFocus, GetForegroundWindow, GetParent, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, OpenClipboard, SendMessageW, SetClipboardData};


fn main() {
    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL,
        '1' as u32,
        || {
            println!("Ctrl-1 pressed");
            copy();
        },
    )
        .unwrap();

    hk.listen();
}

fn copy() {
    unsafe {

        // 获取当前激活窗口的句柄
        let hwnd = GetForegroundWindow();

        // 获取窗口标题
        let mut title = [0u16; 256];
        GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32);

        println!("title:{:?}", OsString::from_wide(&title));

        OpenClipboard(null_mut());
        EmptyClipboard();

        // 获取当前选中的文本
        let mut buf: [u16; 1024] = [0; 1024];
        let mut start = POINT { x: 0, y: 0 };
        let mut end = POINT { x: 0, y: 0 };
        let hwnd_edit = GetFocus();

        println!("hwnd_edit:{:?}", hwnd_edit);

        SendMessageW(
            hwnd_edit,
            EM_GETSEL as UINT,
            &mut start as *mut _ as WPARAM,
            &mut end as *mut _ as LPARAM,
        );
        SendMessageW(
            hwnd_edit,
            0x042e,//EM_GETSELTEXT,
            buf.len() as WPARAM,
            buf.as_mut_ptr() as LPARAM,
        );

        let len = OsString::from_wide(&buf).len();

        // 分配内存
        let h_global = GlobalAlloc(0x0002, (len + 1) * size_of::<u16>() as usize);
        let data_ptr = GlobalLock(h_global) as *mut u16;

        println!("buf:{:?}", buf);

        // 复制数据到内存中
        for i in 0..len {
            *data_ptr.offset(i as isize) = buf[i];
        }

        // 解锁内存
        GlobalUnlock(h_global);

        // 将数据复制到剪贴板中
        SetClipboardData(CF_UNICODETEXT, h_global);

        // 关闭剪贴板
        CloseClipboard();
    }
}