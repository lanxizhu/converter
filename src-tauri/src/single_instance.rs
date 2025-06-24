use tauri::{plugin::TauriPlugin, Manager, Wry};

pub fn init() -> TauriPlugin<Wry> {
    tauri_plugin_single_instance::init(|app, _args, _cwd| {
        let window = app.get_webview_window("main").expect("no main window");
        if let Ok(true) = window.is_minimized() {
            window.unminimize().unwrap();
        }
        if let Ok(false) = window.is_visible() {
            window.show().unwrap();
        }
        window.set_focus().unwrap();
    })
}
