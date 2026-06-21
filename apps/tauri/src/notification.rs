use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

pub fn notify_balance_low(app: &AppHandle, provider_name: &str, remaining: f64) {
    let _ = app.notification()
        .builder()
        .title("Pulse — 余额不足提醒")
        .body(format!("{} 余额剩余 {:.2}，请及时充值", provider_name, remaining))
        .show();
}

pub fn notify_error(app: &AppHandle, provider_name: &str, error: &str) {
    let _ = app.notification()
        .builder()
        .title(format!("Pulse — {} 查询失败", provider_name))
        .body(error.to_string())
        .show();
}

pub fn notify_refresh_complete(app: &AppHandle, count: usize) {
    let _ = app.notification()
        .builder()
        .title("Pulse — 刷新完成")
        .body(format!("已更新 {} 个供应商的余额信息", count))
        .show();
}
