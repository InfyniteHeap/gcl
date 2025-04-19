#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod others;
#[cfg(target_os = "windows")]
mod windows;

use tauri::{App, Manager, WebviewWindow};

trait EnableWindowVibrancy {
    fn enable(ww: &WebviewWindow) -> Result<(), window_vibrancy::Error>;
}

pub(super) fn enable_window_vibrancy(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let ww = app
        .get_webview_window("main")
        .expect("Failed to get webview window");

    #[cfg(target_os = "macos")]
    let result = macos::MacOS::enable(&ww);
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let result = others::Others::enable(&ww);
    #[cfg(target_os = "windows")]
    let result = windows::Window::enable(&ww);

    Ok(result?)
}
