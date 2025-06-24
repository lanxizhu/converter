use tauri::{plugin::TauriPlugin, AppHandle, Manager, Window, Wry};
use tauri_plugin_window_state::{AppHandleExt, Builder, StateFlags, WindowExt};

fn get_flags() -> StateFlags {
    StateFlags::all() & !StateFlags::VISIBLE
}

pub fn restore(app: AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.restore_state(get_flags()).unwrap();
    println!("restored {:?} window state...", window.label());
}

pub fn save(window: &Window) {
    println!("{:?} window destroyed, saving state...", window.label());
    if let Err(err) = window.app_handle().save_window_state(get_flags()) {
        eprintln!("Error saving window state: {:?}", err);
    }
}

pub fn build() -> TauriPlugin<Wry> {
    Builder::default()
        .with_state_flags(get_flags())
        .skip_initial_state("splashscreen")
        .build()
}
