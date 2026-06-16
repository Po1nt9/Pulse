use tauri::{AppHandle, Emitter};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton};

pub fn setup_tray(app: &AppHandle) -> Result<(), tauri::Error> {
    let menu = Menu::with_items(app, &[
        &MenuItem::with_id(app, "show", "显示主面板", true, None::<&str>)?,
        &PredefinedMenuItem::separator(app)?,
        &MenuItem::with_id(app, "refresh", "立即刷新", true, None::<&str>)?,
        &PredefinedMenuItem::separator(app)?,
        &MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?,
    ])?;

    let icon = match app.default_window_icon() {
        Some(icon) => icon.clone(),
        None => return Err(tauri::Error::AssetNotFound("default window icon".into())),
    };

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "show" => crate::window::show_window(app),
                "refresh" => {
                    let _ = app.emit("refresh-requested", ());
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button: MouseButton::Left, .. } = event {
                let app = tray.app_handle();
                crate::window::toggle_window(app);
            }
        })
        .build(app)?;

    Ok(())
}
