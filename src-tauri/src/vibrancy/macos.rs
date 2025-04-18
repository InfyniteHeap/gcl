use tauri::WebviewWindow;
use window_vibrancy::{Error, NSVisualEffectMaterial};

pub(super) fn enable(ww: &WebviewWindow) -> Result<(), Error> {
    window_vibrancy::apply_vibrancy(ww, NSVisualEffectMaterial::FullScreenUI, None, None)
}
