#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod others;
#[cfg(target_os = "windows")]
mod windows;

use std::error::Error;

use tauri::{App, Manager};

pub(super) fn enable_window_vibrancy(app: &mut App) -> Result<(), Box<dyn Error>> {
    let ww = app
        .get_webview_window("main")
        .expect("Failed to get webview window");

    #[cfg(target_os = "macos")]
    let result = macos::enable(&ww);
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let result = others::enable(&ww);
    #[cfg(target_os = "windows")]
    let result = windows::enable(&ww);

    Ok(result?)
}
