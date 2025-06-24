use super::theme;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuBuilder, MenuEvent, MenuItem, SubmenuBuilder},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Error, Manager, Theme, Wry,
};

fn menu_init(app: &App) -> Result<Menu<Wry>, Error> {
    let dark = CheckMenuItem::with_id(app, "dark", "Dark", true, theme::dark(app), None::<&str>)?;

    let light =
        CheckMenuItem::with_id(app, "light", "Light", true, theme::light(app), None::<&str>)?;

    let theme_menus = SubmenuBuilder::new(app, "Theme")
        .item(&dark)
        .item(&light)
        .build()?;

    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = MenuBuilder::new(app)
        .item(&theme_menus)
        .separator()
        .item(&quit_i)
        .build()?;

    let handle = move |app_handle: &AppHandle, event: MenuEvent| match event.id.as_ref() {
        "dark" => {
            app_handle.set_theme(Some(Theme::Dark));
            light.set_checked(false).expect("Change check error");
        }
        "light" => {
            app_handle.set_theme(Some(Theme::Light));
            dark.set_checked(false).expect("Change check error");
        }
        "quit" => {
            app_handle.exit(0);
        }
        _ => {}
    };

    app.on_menu_event(handle);

    Ok(menu)
}

fn tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            println!("left click pressed and released");
            let app = tray.app_handle();
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(true) = window.is_minimizable() {
                    window.unminimize().unwrap();
                }
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }

        _ => {
            // println!("unhandled event {event:?}");
        }
    }
}

pub fn init(app: &mut App) -> Result<(), Error> {
    let menu = menu_init(app)?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(tray_icon_event)
        .build(app)?;

    Ok(())
}
