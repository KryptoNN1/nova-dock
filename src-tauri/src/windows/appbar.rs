//! Dock için AppBar kaydı: ekran alt kenarında yer ayırır.
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::Shell::{
    SHAppBarMessage, ABE_BOTTOM, ABM_NEW, ABM_QUERYPOS, ABM_REMOVE, ABM_SETPOS, APPBARDATA,
};
use windows::Win32::UI::WindowsAndMessaging::WM_USER;

fn data(hwnd: HWND) -> APPBARDATA {
    let mut d = APPBARDATA::default();
    d.cbSize = std::mem::size_of::<APPBARDATA>() as u32;
    d.hWnd = hwnd;
    d.uCallbackMessage = WM_USER + 0x4E; // Aşama 5'te monitör/çalışma alanı değişimi için işlenecek
    d
}

/// `monitor`: monitörün tam alanı (piksel). `height`: ayrılacak yükseklik (piksel).
/// Dönen RECT, sistemin onayladığı gerçek alandır; pencere buraya konumlandırılmalıdır.
pub fn register_bottom(hwnd: HWND, monitor: RECT, height: i32) -> RECT {
    unsafe {
        let mut d = data(hwnd);
        SHAppBarMessage(ABM_NEW, &mut d);
        d.uEdge = ABE_BOTTOM;
        d.rc = RECT { left: monitor.left, right: monitor.right, bottom: monitor.bottom, top: monitor.bottom - height };
        SHAppBarMessage(ABM_QUERYPOS, &mut d);
        d.rc.top = d.rc.bottom - height;
        SHAppBarMessage(ABM_SETPOS, &mut d);
        d.rc
    }
}

pub fn unregister(hwnd: HWND) {
    unsafe { let mut d = data(hwnd); SHAppBarMessage(ABM_REMOVE, &mut d); }
}
