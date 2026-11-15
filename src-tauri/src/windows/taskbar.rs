//! Windows görev çubuğunu geri alınabilir şekilde gizler.
//! Yöntem: (1) otomatik gizle durumunu açar, (2) Shell_TrayWnd / Shell_SecondaryTrayWnd
//! pencerelerini ShowWindow ile gizler. Geri almak için ikisi de eski haline döner.
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::Shell::{
    SHAppBarMessage, ABM_GETSTATE, ABM_SETSTATE, ABS_ALWAYSONTOP, ABS_AUTOHIDE, APPBARDATA,
};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, FindWindowW, ShowWindow, SW_HIDE, SW_SHOWNA,
};

fn appbar_data() -> APPBARDATA {
    let mut d = APPBARDATA::default();
    d.cbSize = std::mem::size_of::<APPBARDATA>() as u32;
    unsafe { d.hWnd = FindWindowW(w!("Shell_TrayWnd"), PCWSTR::null()).unwrap_or_default(); }
    d
}

/// Görev çubuğunun şu anki durum bayraklarını döndürür (ABS_*).
pub fn current_state() -> u32 {
    let mut d = appbar_data();
    unsafe { SHAppBarMessage(ABM_GETSTATE, &mut d) as u32 }
}

pub fn was_autohide(state: u32) -> bool { state & ABS_AUTOHIDE != 0 }

fn set_autohide(on: bool) {
    let mut d = appbar_data();
    let flags = if on { ABS_AUTOHIDE | ABS_ALWAYSONTOP } else { ABS_ALWAYSONTOP };
    d.lParam = LPARAM(flags as isize);
    unsafe { SHAppBarMessage(ABM_SETSTATE, &mut d); }
}

fn for_each_taskbar(f: impl Fn(HWND)) {
    unsafe {
        if let Ok(h) = FindWindowW(w!("Shell_TrayWnd"), PCWSTR::null()) { f(h); }
        let mut prev = HWND::default();
        while let Ok(h) = FindWindowExW(None, prev, w!("Shell_SecondaryTrayWnd"), PCWSTR::null()) {
            f(h);
            prev = h;
        }
    }
}

/// Görev çubuğunu gizler. Çağıran, geri almak için önceden `current_state()` almalıdır.
pub fn hide() {
    set_autohide(true);
    for_each_taskbar(|h| unsafe { let _ = ShowWindow(h, SW_HIDE); });
}

/// Görev çubuğunu kullanıcının önceki ayarına döndürür. Birden fazla kez çağrılması güvenlidir.
pub fn restore(was_autohide: bool) {
    set_autohide(was_autohide);
    for_each_taskbar(|h| unsafe { let _ = ShowWindow(h, SW_SHOWNA); });
}
