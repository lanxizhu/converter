use super::theme;
use super::window_state;
use tauri::{Window, WindowEvent};

pub fn event(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::Destroyed => {
            if window.label() == "main" {
                window_state::save(window);
            }
        }
        WindowEvent::ThemeChanged(theme) => {
            theme::change(window, theme);
        }
        _ => {}
    }
}
