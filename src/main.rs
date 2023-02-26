use std::ffi::OsString;
use std::mem::size_of;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use hotkey;
use winapi::shared::minwindef::{BOOL, DWORD, LPARAM, UINT, WPARAM};
use winapi::shared::windef::{HWND, POINT};
use winapi::um::processthreadsapi::GetCurrentThreadId;
use winapi::um::winbase::{GlobalAlloc, GlobalLock, GlobalUnlock};
use winapi::um::winnt::WCHAR;
use winapi::um::winuser;
use winapi::um::winuser::{AttachThreadInput, CF_UNICODETEXT, CloseClipboard, EM_GETSEL, EmptyClipboard, EnumChildWindows, FindWindowExW, GetClassNameW, GetDlgCtrlID, GetDlgItemTextW, GetFocus, GetForegroundWindow, GetParent, GetWindow, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, GW_CHILD, GW_HWNDNEXT, IsWindowVisible, OpenClipboard, SendMessageW, SetClipboardData, WM_GETTEXT, WM_GETTEXTLENGTH};

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

        let win_vec = get_child_controls(hwnd);
        println!("win_vec:{:?}", win_vec);

        // 获取窗口标题
        let mut title = [0u16; 256];
        GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32);

        println!("title:{:?}", OsString::from_wide(&title));

        OpenClipboard(null_mut());
        EmptyClipboard();

        // 获取当前选中的文本
        let mut buf: [WCHAR; 1024] = [0; 1024];
        let hwnd_edit = GetFocus();
        println!("hwnd_edit:{:?}", hwnd_edit);
        if hwnd_edit.is_null() {
            // 如果没有焦点窗口，则使用前台窗口
            let hwnd_fg = GetForegroundWindow();
            SendMessageW(
                hwnd_fg,
                WM_GETTEXT,
                buf.len() as WPARAM,
                buf.as_mut_ptr() as LPARAM,
            );
        } else {
            SendMessageW(
                hwnd_edit,
                WM_GETTEXT,
                buf.len() as WPARAM,
                buf.as_mut_ptr() as LPARAM,
            );
        }

        println!("buf:{:?}", OsString::from_wide(&buf));

        let len = OsString::from_wide(&buf).len();

        // 分配内存
        let h_global = GlobalAlloc(0x0002, (len + 1) * size_of::<u16>() as usize);
        let data_ptr = GlobalLock(h_global) as *mut u16;

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


fn get_child_controls(hwnd_parent: HWND) -> Vec<(HWND, String, String)> {
    let mut child_controls: Vec<(HWND, String, String)> = Vec::new();

    unsafe {
        let mut hwnd_child = GetWindow(hwnd_parent, GW_CHILD);
        while hwnd_child != null_mut() {
            if IsWindowVisible(hwnd_child) != 0 {
                let mut class_name: [u16; 256] = [0; 256];
                GetClassNameW(hwnd_child, class_name.as_mut_ptr(), class_name.len() as i32);
                let class_name = String::from_utf16_lossy(&class_name);

                let mut text: [u16; 256] = [0; 256];
                let text_len = SendMessageW(
                    hwnd_child,
                    WM_GETTEXTLENGTH,
                    0,
                    0,
                ) + 1;
                SendMessageW(
                    hwnd_child,
                    WM_GETTEXT,
                    text_len as WPARAM,
                    text.as_mut_ptr() as LPARAM,
                );
                let text = String::from_utf16_lossy(&text[..text_len as usize]);

                child_controls.push((hwnd_child, class_name, text));
            }

            hwnd_child = GetWindow(hwnd_child, GW_HWNDNEXT);
        }
    }

    child_controls
}