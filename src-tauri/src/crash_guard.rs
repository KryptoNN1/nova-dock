//! Çökme koruması. Ana uygulama `--watchdog <pid> <autohide:0|1>` argümanlarıyla kendi
//! kopyasını başlatır; bu küçük süreç ana süreç bitene kadar bekler ve görev çubuğunu geri yükler.
use crate::windows::taskbar;
use std::os::windows::process::CommandExt;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{OpenProcess, WaitForSingleObject, INFINITE, PROCESS_SYNCHRONIZE};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn spawn_watchdog(was_autohide: bool) {
    if let Ok(exe) = std::env::current_exe() {
        let _ = std::process::Command::new(exe)
            .args(["--watchdog", &std::process::id().to_string(), if was_autohide { "1" } else { "0" }])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }
}

/// `true` dönerse bu süreç watchdog'dur ve iş bitmiştir; normal başlatma yapılmamalı.
pub fn run_if_watchdog() -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) != Some("--watchdog") { return false; }
    let pid: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let autohide = args.get(3).map(String::as_str) == Some("1");
    unsafe {
        if let Ok(h) = OpenProcess(PROCESS_SYNCHRONIZE, false, pid) {
            WaitForSingleObject(h, INFINITE);
            let _ = CloseHandle(h);
        }
    }
    taskbar::restore(autohide);
    true
}
