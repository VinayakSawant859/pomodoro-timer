#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod database;

use database::AppSettings;
use std::fs;
use tauri::Manager;

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
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
