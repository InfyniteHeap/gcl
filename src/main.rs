#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use windows_reactor::*;

fn main() {
    App::run_component::<app::Launcher>(()).expect("Failed to launch this app");
}
