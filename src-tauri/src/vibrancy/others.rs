use super::EnableWindowVibrancy;

use tauri::WebviewWindow;
use window_vibrancy::Error;

pub(super) struct Others;

impl EnableWindowVibrancy for Others {
    fn enable(_ww: &WebviewWindow) -> Result<(), Error> {
        Ok(())
    }
}
