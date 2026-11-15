mod crash_guard;
mod windows;

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Manager, PhysicalPosition, PhysicalSize};
use ::windows::Win32::Foundation::{HWND, RECT};

static ORIGINAL_AUTOHIDE: AtomicBool = AtomicBool::new(false);
const DOCK_RESERVED_PX: i32 = 96;

fn restore_taskbar_now() {
    windows::taskbar::restore(ORIGINAL_AUTOHIDE.load(Ordering::SeqCst));
}

#[tauri::command]
fn quit_and_restore(app: tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("dock") {
        if let Ok(h) = w.hwnd() { windows::appbar::unregister(HWND(h.0)); }
    }
    restore_taskbar_now();
    app.exit(0);
}

pub fn run() {
    // Watchdog modunda arayüz açılmaz.
    if crash_guard::run_if_watchdog() { return; }

    // Panik olsa bile görev çubuğunu geri getir.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| { restore_taskbar_now(); default_hook(info); }));

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![quit_and_restore])
        .setup(|app| {
            let win = app.get_webview_window("dock").expect("dock penceresi yok");
            let monitor = win.primary_monitor()?.expect("birincil monitör yok");
            let pos = monitor.position();
            let size = monitor.size();

            // 1) Önce eski durumu kaydet, sonra watchdog'u başlat, en son gizle.
            let state = windows::taskbar::current_state();
            let autohide = windows::taskbar::was_autohide(state);
            ORIGINAL_AUTOHIDE.store(autohide, Ordering::SeqCst);
            crash_guard::spawn_watchdog(autohide);

            // 2) AppBar ile alt kenarda yer ayır.
            let hwnd = HWND(win.hwnd()?.0);
            let rc = windows::appbar::register_bottom(hwnd, RECT {
                left: pos.x, top: pos.y, right: pos.x + size.width as i32, bottom: pos.y + size.height as i32,
            }, DOCK_RESERVED_PX);

            win.set_position(PhysicalPosition::new(rc.left, rc.top))?;
            win.set_size(PhysicalSize::new((rc.right - rc.left) as u32, (rc.bottom - rc.top) as u32))?;
            win.show()?;

            // 3) Orijinal görev çubuğunu gizle.
            windows::taskbar::hide();
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                if let Ok(h) = window.hwnd() { windows::appbar::unregister(HWND(h.0)); }
                restore_taskbar_now();
            }
        })
        .run(tauri::generate_context!())
        .expect("NovaDock başlatılamadı");
}
