use tauri::{plugin::TauriPlugin, App, AppHandle, Manager, Wry};
use tauri_plugin_global_shortcut::{
    Builder, Code, Error, GlobalShortcutExt, Modifiers, Shortcut, ShortcutEvent, ShortcutState,
};

fn esc_shortcut() -> Shortcut {
    Shortcut::new(Some(Modifiers::empty()), Code::Escape)
}

pub fn init(app: &mut App) -> Result<(), Error> {
    app.global_shortcut().register(esc_shortcut())?;

    Ok(())
}

fn handler(app: &AppHandle, shortcut: &Shortcut, event: ShortcutEvent) {
    if shortcut == &esc_shortcut() {
        match event.state() {
            ShortcutState::Pressed => {}
            ShortcutState::Released => {
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(true) = window.is_focused() {
                        window.hide().unwrap();
                    }
                }
            }
        }
    }
}

pub fn build() -> TauriPlugin<Wry> {
    Builder::new().with_handler(handler).build()
}
