#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use rusqlite::Connection;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub completed: bool,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub work_duration: u32,
    pub break_duration: u32,
    pub long_break_duration: u32,
    pub sessions_until_long_break: u32,
    pub sound_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            work_duration: 25,
            break_duration: 5,
            long_break_duration: 15,
            sessions_until_long_break: 4,
            sound_enabled: true,
        }
    }
}

// Database functions
fn init_db(app_data_dir: &PathBuf) -> Result<Connection, Box<dyn std::error::Error>> {
    fs::create_dir_all(app_data_dir)?;
    let db_path = app_data_dir.join("pomodoro.db");
    let conn = Connection::open(db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            text TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            completed_at TEXT
        )",
        [],
    )?;
    
    Ok(conn)
}

// Task management commands
#[tauri::command]
async fn add_task(
    app: tauri::AppHandle,
    text: String,
) -> Result<Task, String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let conn = init_db(&app_data_dir)
        .map_err(|e| format!("Database error: {}", e))?;
    
    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        text,
        completed: false,
        created_at: chrono::Utc::now().to_rfc3339(),
        completed_at: None,
    };
    
    conn.execute(
        "INSERT INTO tasks (id, text, completed, created_at) VALUES (?1, ?2, ?3, ?4)",
        [&task.id, &task.text, &task.completed.to_string(), &task.created_at],
    ).map_err(|e| format!("Database error: {}", e))?;
    
    Ok(task)
}

#[tauri::command]
async fn get_tasks(app: tauri::AppHandle) -> Result<Vec<Task>, String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let conn = init_db(&app_data_dir)
        .map_err(|e| format!("Database error: {}", e))?;
    
    let mut stmt = conn.prepare(
        "SELECT id, text, completed, created_at, completed_at FROM tasks ORDER BY created_at DESC"
    ).map_err(|e| format!("Database error: {}", e))?;
    
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            text: row.get(1)?,
            completed: row.get::<_, i32>(2)? != 0,
            created_at: row.get(3)?,
            completed_at: row.get(4)?,
        })
    }).map_err(|e| format!("Database error: {}", e))?;
    
    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task.map_err(|e| format!("Database error: {}", e))?);
    }
    
    Ok(tasks)
}

#[tauri::command]
async fn complete_task(
    app: tauri::AppHandle,
    task_id: String,
    completed: bool,
) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let conn = init_db(&app_data_dir)
        .map_err(|e| format!("Database error: {}", e))?;
    
    let completed_at = if completed {
        Some(chrono::Utc::now().to_rfc3339())
    } else {
        None
    };
    
    conn.execute(
        "UPDATE tasks SET completed = ?1, completed_at = ?2 WHERE id = ?3",
        [
            &(if completed { "1" } else { "0" }),
            &completed_at.as_deref().unwrap_or(""),
            task_id.as_str()
        ],
    ).map_err(|e| format!("Database error: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn update_task(
    app: tauri::AppHandle,
    task_id: String,
    text: String,
) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let conn = init_db(&app_data_dir)
        .map_err(|e| format!("Database error: {}", e))?;
    
    conn.execute(
        "UPDATE tasks SET text = ?1 WHERE id = ?2",
        [&text, &task_id],
    ).map_err(|e| format!("Database error: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn delete_task(
    app: tauri::AppHandle,
    task_id: String,
) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let conn = init_db(&app_data_dir)
        .map_err(|e| format!("Database error: {}", e))?;
    
    conn.execute(
        "DELETE FROM tasks WHERE id = ?1",
        [&task_id],
    ).map_err(|e| format!("Database error: {}", e))?;
    
    Ok(())
}

// Settings management
#[tauri::command]
async fn get_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let app_data_dir = app.path().app_data_dir()
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
async fn save_settings(
    app: tauri::AppHandle,
    settings: AppSettings,
) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir()
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

// Audio functions
#[tauri::command]
async fn play_notification_sound(sound_type: String) -> Result<(), String> {
    tokio::spawn(async move {
        if let Err(e) = play_sound_internal(sound_type).await {
            eprintln!("Failed to play sound: {}", e);
        }
    });
    Ok(())
}

async fn play_sound_internal(sound_type: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    
    // Generate simple beep sounds
    let sound_data = match sound_type.as_str() {
        "work_complete" => generate_work_complete_sound(),
        "break_complete" => generate_break_complete_sound(),
        "tick" => generate_tick_sound(),
        _ => generate_tick_sound(),
    };
    
    let cursor = Cursor::new(sound_data);
    let source = Decoder::new(cursor)?;
    sink.append(source);
    
    sink.sleep_until_end();
    Ok(())
}

// Simple sound generation functions (placeholder - you would implement actual sound generation)
fn generate_work_complete_sound() -> Vec<u8> {
    // This is a placeholder - in a real app, you'd generate or load actual audio data
    vec![0; 1000] // Placeholder empty audio data
}

fn generate_break_complete_sound() -> Vec<u8> {
    vec![0; 1000] // Placeholder empty audio data
}

fn generate_tick_sound() -> Vec<u8> {
    vec![0; 100] // Placeholder empty audio data
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            add_task,
            get_tasks,
            complete_task,
            update_task,
            delete_task,
            get_settings,
            save_settings,
            play_notification_sound
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}