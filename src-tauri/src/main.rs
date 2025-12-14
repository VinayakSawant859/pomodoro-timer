#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod database;

use database::AppSettings;
use std::fs;
use std::sync::{Arc, Mutex};
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, AppHandle,
};
use tauri::menu::{MenuBuilder, MenuItemBuilder};

// Monk Mode state
#[derive(Default)]
struct MonkModeState {
    enabled: Arc<Mutex<bool>>,
}

impl MonkModeState {
    fn new() -> Self {
        Self {
            enabled: Arc::new(Mutex::new(false)),
        }
    }

    fn is_enabled(&self) -> bool {
        *self.enabled.lock().unwrap()
    }

    fn set_enabled(&self, enabled: bool) {
        *self.enabled.lock().unwrap() = enabled;
    }
}

#[tauri::command]
async fn update_status(app: AppHandle, text: String) -> Result<(), String> {
    // Update window title
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_title(&text)
            .map_err(|e| format!("Failed to set window title: {}", e))?;
    }

    // Update tray tooltip
    if let Some(tray) = app.tray_by_id("main-tray") {
        tray.set_tooltip(Some(&text))
            .map_err(|e| format!("Failed to set tray tooltip: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
async fn set_monk_mode(app: AppHandle, enabled: bool) -> Result<(), String> {
    // Update state
    let monk_mode_state = app.state::<MonkModeState>();
    monk_mode_state.set_enabled(enabled);

    if let Some(window) = app.get_webview_window("main") {
        // Set fullscreen mode
        window
            .set_fullscreen(enabled)
            .map_err(|e| format!("Failed to set fullscreen: {}", e))?;
        
        // Set always-on-top
        window
            .set_always_on_top(enabled)
            .map_err(|e| format!("Failed to set always-on-top: {}", e))?;

        println!("Monk Mode {}: Fullscreen={}, Always-on-top={}", 
                 if enabled { "ACTIVATED 🧘" } else { "Deactivated" }, 
                 enabled, enabled);
    }

    Ok(())
}

#[tauri::command]
async fn get_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let settings_path = app_data_dir.join("settings.json");

    if settings_path.exists() {
        let settings_content = fs::read_to_string(settings_path)
            .map_err(|e| format!("Failed to read settings: {}", e))?;

        serde_json::from_str(&settings_content)
            .map_err(|e| format!("Failed to parse settings: {}", e))
    } else {
        Ok(AppSettings::default())
    }
}

#[tauri::command]
async fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    let settings_path = app_data_dir.join("settings.json");
    let settings_content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(settings_path, settings_content)
        .map_err(|e| format!("Failed to write settings: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let (_audio_stream, audio_handle) = audio::AudioStream::new()
        .expect("Failed to initialize audio system");

    let audio_state = audio::AudioState::new(audio_handle);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .manage(audio_state)
        .invoke_handler(tauri::generate_handler![
            database::add_task,
            database::get_tasks,
            database::complete_task,
            database::update_task,
            database::delete_task,
            database::start_pomodoro_session,
            database::complete_pomodoro_session,
            database::get_task_with_stats,
            database::get_daily_stats,
            database::get_focus_heatmap,
            database::export_data,
            get_settings,
            save_settings,
            update_status,
            set_monk_mode,
            audio::play_sound,
            audio::play_notification_sound,
            audio::set_white_noise,
            audio::get_white_noise_volume,
            audio::set_white_noise_volume,
            audio::is_white_noise_playing
        ])
        .setup(|app| {
            let db_pool = database::initialize_database(&app.handle())
                .map_err(|e| format!("Failed to initialize database: {}", e))?;
            
            app.manage(db_pool);

            // Initialize monk mode state
            let monk_mode_state = MonkModeState::new();
            app.manage(monk_mode_state);

            // Setup system tray
            let show_item = MenuItemBuilder::with_id("show", "Show")
                .build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "Quit")
                .build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[&show_item, &quit_item])
                .build()?;

            let _tray = TrayIconBuilder::with_id("main-tray")
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Pomodoro Timer")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            app.manage(_tray);
            
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Check if monk mode is enabled
                let monk_mode_state = window.state::<MonkModeState>();
                if monk_mode_state.is_enabled() {
                    // Prevent closing in monk mode
                    api.prevent_close();
                    
                    // Log to console - user will see this in dev mode
                    println!("🔒 Monk Mode: Cannot close during focus session!");
                    
                    // Note: Notifications should be triggered from the frontend
                    // when user attempts to close. The Rust API doesn't provide
                    // a simple way to send notifications from event handlers.
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
