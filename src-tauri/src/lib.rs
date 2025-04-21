mod vibrancy;

#[tauri::command]
fn show_window(window: tauri::Window) {
    window.show().expect("Failed to show window")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(vibrancy::enable_window_vibrancy)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![show_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
