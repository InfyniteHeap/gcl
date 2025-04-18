use tauri::WebviewWindow;
use window_vibrancy::Error;

// We deliberately leave an empty implementation here
// to make implementations consistent.
pub(super) fn enable(_ww: &WebviewWindow) -> Result<(), Error> {
    Ok(())
}
