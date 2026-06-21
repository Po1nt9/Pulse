use tauri::Manager;

pub mod commands;
pub mod config;
pub mod error;
pub mod http;
pub mod notification;
pub mod providers;
pub mod tray;
pub mod window;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub config: Arc<RwLock<config::AppConfig>>,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn new() -> crate::error::Result<Self> {
        Ok(Self {
            config: Arc::new(RwLock::new(config::read_config_sync()?)),
            http_client: http::create_client(),
        })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
            }
        }))
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            commands::balance::get_balance,
            commands::balance::refresh_all_balances,
            commands::usage::get_usage,
            commands::providers::list_providers,
            commands::providers::add_provider,
            commands::providers::update_provider,
            commands::providers::delete_provider,
            commands::providers::toggle_provider,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::keychain::store_api_key,
            commands::keychain::retrieve_api_key,
            commands::keychain::delete_api_key,
            commands::keychain::has_api_key,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_state = AppState::new()
                .map_err(|e| Box::<dyn std::error::Error>::from(format!("init app state: {e}")))?;
            app.manage(app_state);

            tray::setup_tray(&app_handle)?;

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
