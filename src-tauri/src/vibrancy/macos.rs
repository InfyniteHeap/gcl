use super::EnableWindowVibrancy;

use tauri::WebviewWindow;
use window_vibrancy::{Error, NSVisualEffectMaterial};

pub(super) struct MacOS;

impl EnableWindowVibrancy for MacOS {
    fn enable(ww: &WebviewWindow) -> Result<(), Error> {
        window_vibrancy::apply_vibrancy(ww, NSVisualEffectMaterial::FullScreenUI, None, None)
    }
}
