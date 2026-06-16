use tauri::{AppHandle, Manager, PhysicalPosition};

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
                position_near_tray(&window);
            }
            Err(_) => {}
        }
    }
}

fn position_near_tray(window: &tauri::WebviewWindow) {
    if let Ok(Some(monitor)) = window.current_monitor() {
        // Use scale_factor to convert physical ↔ logical if needed
        let scale = monitor.scale_factor();

        // work_area excludes the taskbar/dock on supported platforms
        let available = monitor.work_area();
        let window_size = match window.outer_size() {
            Ok(s) => s,
            Err(_) => return,
        };

        // Default: bottom-right corner with 16px margin
        // On Windows the system tray is typically at bottom-right,
        // and the available area already accounts for taskbar size/position.
        let margin = (16.0 * scale) as i32;
        let x = available.position.x + available.size.width as i32 - window_size.width as i32 - margin;
        let y = available.position.y + available.size.height as i32 - window_size.height as i32 - margin;

        let _ = window.set_position(tauri::Position::Physical(PhysicalPosition::new(x, y)));
    }
}
