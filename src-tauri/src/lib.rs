mod global_shortcut;
mod single_instance;
mod splash_screen;
mod theme;
mod tray;
mod window;
mod window_state;

use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tray::init(app).unwrap();
            splash_screen::init(app).unwrap();
            global_shortcut::init(app).unwrap();
            theme::init(app);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(window_state::build())
        .plugin(single_instance::init())
        .plugin(global_shortcut::build())
        .manage(Mutex::new(splash_screen::SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        .on_window_event(window::event)
        .invoke_handler(tauri::generate_handler![greet, splash_screen::set_complete])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
