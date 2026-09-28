// No console window next to the engine in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicUsize, Ordering};

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

#[cfg(windows)]
mod capture {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetWindowDisplayAffinity, SetWindowDisplayAffinity, WDA_EXCLUDEFROMCAPTURE, WDA_NONE,
    };

    // Tauri exposes HWND from its own `windows` crate version, so only the raw
    // pointer is passed across to avoid coupling to that version.
    pub fn set_excluded(hwnd: *mut core::ffi::c_void, excluded: bool) -> Result<(), String> {
        let affinity = if excluded { WDA_EXCLUDEFROMCAPTURE } else { WDA_NONE };
        unsafe { SetWindowDisplayAffinity(HWND(hwnd), affinity) }.map_err(|e| e.to_string())
    }

    pub fn is_excluded(hwnd: *mut core::ffi::c_void) -> Result<bool, String> {
        let mut affinity = 0u32;
        unsafe { GetWindowDisplayAffinity(HWND(hwnd), &mut affinity) }.map_err(|e| e.to_string())?;
        Ok(affinity == WDA_EXCLUDEFROMCAPTURE.0)
    }
}

fn set_protection(window: &WebviewWindow, enabled: bool) -> Result<(), String> {
    #[cfg(windows)]
    {
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        capture::set_excluded(hwnd.0, enabled)
    }
    #[cfg(not(windows))]
    {
        let _ = (window, enabled);
        Err("capture protection is only supported on Windows".into())
    }
}

/// Windows are created hidden and only shown once excluded from capture, so
/// no frame can leak into a recording before the affinity is applied.
fn protect_and_show(window: &WebviewWindow) -> Result<(), String> {
    set_protection(window, true)?;
    window.show().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_capture_protection(window: WebviewWindow, enabled: bool) -> Result<(), String> {
    set_protection(&window, enabled)
}

#[tauri::command]
fn get_capture_protection(window: WebviewWindow) -> Result<bool, String> {
    #[cfg(windows)]
    {
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        capture::is_excluded(hwnd.0)
    }
    #[cfg(not(windows))]
    {
        let _ = window;
        Ok(false)
    }
}

/// Opens a remote page in a separate WebView2 window to verify that content
/// rendered by WebView2's child processes is excluded from capture as well.
#[tauri::command]
fn open_test_browser(app: AppHandle) -> Result<(), String> {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
    let label = format!("browser-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let url = "https://example.com".parse().map_err(|e| format!("{e}"))?;
    let window = WebviewWindowBuilder::new(&app, label, WebviewUrl::External(url))
        .title("InvisEngine – Test-Browser")
        .inner_size(1000.0, 700.0)
        .visible(false)
        .skip_taskbar(true)
        .build()
        .map_err(|e| e.to_string())?;
    protect_and_show(&window)
}

fn toggle_all_windows(app: &AppHandle) {
    let windows = app.webview_windows();
    let any_visible = windows.values().any(|w| w.is_visible().unwrap_or(false));
    for window in windows.values() {
        let result = if any_visible { window.hide() } else { window.show() };
        if let Err(e) = result {
            eprintln!("failed to toggle window {}: {e}", window.label());
        }
    }
}

fn main() {
    let toggle_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::Space);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if shortcut == &toggle_shortcut && event.state() == ShortcutState::Pressed {
                        toggle_all_windows(app);
                    }
                })
                .build(),
        )
        .setup(move |app| {
            use tauri_plugin_global_shortcut::GlobalShortcutExt;
            app.global_shortcut().register(toggle_shortcut)?;

            for window in app.webview_windows().values() {
                protect_and_show(window)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_capture_protection,
            get_capture_protection,
            open_test_browser
        ])
        .run(tauri::generate_context!())
        .expect("failed to run InvisEngine");
}
