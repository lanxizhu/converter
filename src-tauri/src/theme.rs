use raw_window_handle::HasWindowHandle;
use tauri::{App, Manager, Theme};

#[cfg(target_os = "windows")]
use window_vibrancy::apply_mica;

pub fn get(app: &App) -> Theme {
    let theme = app
        .get_webview_window("main")
        .unwrap()
        .theme()
        .unwrap_or(Theme::Light);
    return theme;
}

pub fn dark(app: &App) -> bool {
    let theme = get(app);
    theme == Theme::Dark
}

pub fn light(app: &App) -> bool {
    let theme = get(app);
    theme == Theme::Light
}

pub fn init(app: &mut App) {
    app.webview_windows().iter().for_each(|window| {
        #[cfg(target_os = "windows")]
        {
            apply_mica(window.1, Some(dark(app)))
                .expect("Unsupported platform! 'apply_mica' is only supported on Windows");
        }
    });
}

pub fn change(window: impl HasWindowHandle, theme: &Theme) {
    let dark = theme == &Theme::Dark;

    #[cfg(target_os = "windows")]
    {
        apply_mica(window, Some(dark))
            .expect("Unsupported platform! 'apply_mica' is only supported on Windows");
    }
}
