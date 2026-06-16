use tauri::{AppHandle, Manager, PhysicalPosition, WebviewWindow};

use crate::AppState;

fn position_near_tray(window: &WebviewWindow) {
    if let Ok(Some(monitor)) = window.current_monitor() {
        let scale = monitor.scale_factor();
        let available = monitor.work_area();
        let window_size = match window.outer_size() {
            Ok(s) => s,
            Err(_) => return,
        };
        let margin = (16.0 * scale) as i32;
        let x = available.position.x + available.size.width as i32 - window_size.width as i32 - margin;
        let y = available.position.y + available.size.height as i32 - window_size.height as i32 - margin;
        let _ = window.set_position(tauri::Position::Physical(PhysicalPosition::new(x, y)));
    }
}

fn apply_known_position(window: &WebviewWindow, pos: (i32, i32)) {
    let _ = window.set_position(tauri::Position::Physical(PhysicalPosition::new(pos.0, pos.1)));
}

pub fn show_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        position_near_tray(&window);
    }
}

pub fn hide_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

pub fn toggle_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => { let _ = window.hide(); }
            Ok(false) => {
                let _ = window.show();
                let _ = window.set_focus();

                if let Some(state) = app.try_state::<AppState>() {
                    let stored = {
                        let cfg = state.config.blocking_read();
                        cfg.settings.window_position
                    };
                    match stored {
                        Some(pos) => apply_known_position(&window, pos),
                        None => position_near_tray(&window),
                    }
                } else {
                    position_near_tray(&window);
                }
            }
            Err(_) => {}
        }
    }
}
