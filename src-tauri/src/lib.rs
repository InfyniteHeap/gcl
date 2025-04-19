mod vibrancy;

#[tauri::command]
fn show_window(window: tauri::Window) {
    window.show().expect("Failed to show window")
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(vibrancy::enable_window_vibrancy)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![show_window, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
