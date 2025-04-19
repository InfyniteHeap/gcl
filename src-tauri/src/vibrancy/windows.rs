use super::EnableWindowVibrancy;

use tauri::WebviewWindow;
use window_vibrancy::Error;
use windows::UI::Color;
use windows::UI::ViewManagement::{UIColorType, UISettings};
use windows_version::OsVersion;

pub(super) struct Window;

impl EnableWindowVibrancy for Window {
    fn enable(ww: &WebviewWindow) -> Result<(), Error> {
        match OsVersion::current() {
            // Windows 11 and later
            OsVersion {
                major: 10,
                build: _b @ 22000..,
                ..
            } => window_vibrancy::apply_mica(ww, Some(is_dark_mode())),
            // Windows 10 v1809 to 22H2
            OsVersion {
                major: 10,
                build: _b @ 17763..=19045,
                ..
            } => window_vibrancy::apply_acrylic(ww, Some((18, 18, 18, 125))),
            // Windows 10 v1803 and earlier
            _ => window_vibrancy::apply_blur(ww, Some((18, 18, 18, 125))),
        }
    }
}

fn is_dark_mode() -> bool {
    let settings = UISettings::new().expect("Failed to get UI settings");
    let foreground = settings
        .GetColorValue(UIColorType::Foreground)
        .expect("Failed to get foreground color");

    is_color_light(&foreground)
}

fn is_color_light(clr: &Color) -> bool {
    ((5 * clr.G as u16) + (2 * clr.R as u16) + clr.B as u16) > (8 * 128)
}
