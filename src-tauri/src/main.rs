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
    pub priority: i32,
    pub estimated_pomodoros: i32,
    pub actual_pomodoros: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PomodoroSession {
    pub id: String,
    pub task_id: Option<String>,
    pub session_type: String, // 'work', 'short_break', 'long_break'
    pub duration_minutes: u32,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub interrupted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub pomodoros_completed: u32,
    pub total_work_time: u32, // in minutes
    pub tasks_completed: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskWithStats {
    pub task: Task,
    pub pomodoro_sessions: Vec<PomodoroSession>,
    pub total_time_spent: u32, // in minutes
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
const DB_VERSION: i32 = 2;

fn init_db(app_data_dir: &PathBuf) -> Result<Connection, Box<dyn std::error::Error>> {
    fs::create_dir_all(app_data_dir)?;
    let db_path = app_data_dir.join("pomodoro.db");
    let conn = Connection::open(db_path)?;
    
    // Create or update database schema
    migrate_database(&conn)?;
    
    Ok(conn)
}

fn migrate_database(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Create version table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS db_version (version INTEGER PRIMARY KEY)",
        [],
    )?;
    
    // Get current version
    let current_version: i32 = conn.query_row(
        "SELECT version FROM db_version ORDER BY version DESC LIMIT 1",
        [],
        |row| row.get(0)
    ).unwrap_or(0);
    
    if current_version < DB_VERSION {
        // Run migrations
        for version in (current_version + 1)..=DB_VERSION {
            match version {
                1 => {
                    // Initial schema
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
                }
                2 => {
                    // Enhanced schema with new tables and columns
                    conn.execute(
                        "ALTER TABLE tasks ADD COLUMN priority INTEGER DEFAULT 0",
                        [],
                    ).ok(); // Ignore if column already exists
                    
                    conn.execute(
                        "ALTER TABLE tasks ADD COLUMN estimated_pomodoros INTEGER DEFAULT 1",
                        [],
                    ).ok();
                    
                    conn.execute(
                        "ALTER TABLE tasks ADD COLUMN actual_pomodoros INTEGER DEFAULT 0",
                        [],
                    ).ok();
                    
                    // Pomodoro sessions table
                    conn.execute(
                        "CREATE TABLE IF NOT EXISTS pomodoro_sessions (
                            id TEXT PRIMARY KEY,
                            task_id TEXT,
                            session_type TEXT NOT NULL CHECK(session_type IN ('work', 'short_break', 'long_break')),
                            duration_minutes INTEGER NOT NULL,
                            started_at TEXT NOT NULL,
                            completed_at TEXT,
                            interrupted BOOLEAN DEFAULT 0,
                            FOREIGN KEY(task_id) REFERENCES tasks(id) ON DELETE SET NULL
                        )",
                        [],
                    )?;
                    
                    // Daily statistics table
                    conn.execute(
                        "CREATE TABLE IF NOT EXISTS daily_stats (
                            date TEXT PRIMARY KEY,
                            pomodoros_completed INTEGER DEFAULT 0,
                            total_work_time INTEGER DEFAULT 0,
                            tasks_completed INTEGER DEFAULT 0,
                            created_at TEXT NOT NULL
                        )",
                        [],
                    )?;
                    
                    // Settings table
                    conn.execute(
                        "CREATE TABLE IF NOT EXISTS settings (
                            key TEXT PRIMARY KEY,
                            value TEXT NOT NULL,
                            updated_at TEXT NOT NULL
                        )",
                        [],
                    )?;
                }
                _ => {}
            }
        }
        
        // Update version
        conn.execute(
            "INSERT OR REPLACE INTO db_version (version) VALUES (?1)",
            [DB_VERSION],
        )?;
    }
    
    Ok(())
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
        priority: 0,
        estimated_pomodoros: 1,
        actual_pomodoros: 0,
    };
    
    conn.execute(
        "INSERT INTO tasks (id, text, completed, created_at, priority, estimated_pomodoros, actual_pomodoros) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        [
            &task.id, 
            &task.text, 
            &task.completed.to_string(), 
            &task.created_at,
            &task.priority.to_string(),
            &task.estimated_pomodoros.to_string(),
            &task.actual_pomodoros.to_string()
        ],
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
        "SELECT id, text, completed, created_at, completed_at, 
                COALESCE(priority, 0), COALESCE(estimated_pomodoros, 1), COALESCE(actual_pomodoros, 0)
         FROM tasks ORDER BY priority DESC, created_at DESC"
    ).map_err(|e| format!("Database error: {}", e))?;
    
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            text: row.get(1)?,
            completed: row.get::<_, i32>(2)? != 0,
            created_at: row.get(3)?,
            completed_at: row.get(4)?,
            priority: row.get::<_, Option<i32>>(5)?.unwrap_or(0),
            estimated_pomodoros: row.get::<_, Option<i32>>(6)?.unwrap_or(1),
            actual_pomodoros: row.get::<_, Option<i32>>(7)?.unwrap_or(0),
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
    
    // Update daily stats when task is completed
    if completed {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        conn.execute(
            "INSERT OR REPLACE INTO daily_stats (date, tasks_completed, created_at) 
             VALUES (?1, COALESCE((SELECT tasks_completed FROM daily_stats WHERE date = ?1), 0) + 1, ?2)",
            [&today, &chrono::Utc::now().to_rfc3339()],
        ).map_err(|e| format!("Database error: {}", e))?;
    }
    
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

// Pomodoro session management commands
#[tauri::command]
async fn start_pomodoro_session(
    app: tauri::AppHandle,
    task_id: Option<String>,
    session_type: String,
    duration_minutes: u32,
) -> Result<PomodoroSession, String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let conn = init_db(&app_data_dir)
        .map_err(|e| format!("Database error: {}", e))?;
    
    let session = PomodoroSession {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: task_id.clone(),
        session_type,
        duration_minutes,
        started_at: chrono::Utc::now().to_rfc3339(),
        completed_at: None,
        interrupted: false,
    };
    
    conn.execute(
        "INSERT INTO pomodoro_sessions (id, task_id, session_type, duration_minutes, started_at) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            &session.id,
            task_id.as_deref().unwrap_or(""),
            &session.session_type,
            &session.duration_minutes.to_string(),
            &session.started_at
        ],
    ).map_err(|e| format!("Database error: {}", e))?;
    
    Ok(session)
}

#[tauri::command]
async fn complete_pomodoro_session(
    app: tauri::AppHandle,
    session_id: String,
    interrupted: bool,
) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let conn = init_db(&app_data_dir)
        .map_err(|e| format!("Database error: {}", e))?;
    
    let completed_at = chrono::Utc::now().to_rfc3339();
    
    conn.execute(
        "UPDATE pomodoro_sessions SET completed_at = ?1, interrupted = ?2 WHERE id = ?3",
        [
            &completed_at,
            &(if interrupted { "1" } else { "0" }).to_string(),
            &session_id
        ],
    ).map_err(|e| format!("Database error: {}", e))?;
    
    // Update task pomodoro count if session was completed and not interrupted
    if !interrupted {
        conn.execute(
            "UPDATE tasks SET actual_pomodoros = actual_pomodoros + 1 
             WHERE id = (SELECT task_id FROM pomodoro_sessions WHERE id = ?1 AND task_id IS NOT NULL)",
            [&session_id],
        ).map_err(|e| format!("Database error: {}", e))?;
        
        // Update daily stats
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        conn.execute(
            "INSERT OR REPLACE INTO daily_stats (date, pomodoros_completed, created_at) 
             VALUES (?1, COALESCE((SELECT pomodoros_completed FROM daily_stats WHERE date = ?1), 0) + 1, ?2)",
            [&today, &chrono::Utc::now().to_rfc3339()],
        ).map_err(|e| format!("Database error: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
async fn get_task_with_stats(
    app: tauri::AppHandle,
    task_id: String,
) -> Result<TaskWithStats, String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let conn = init_db(&app_data_dir)
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Get task
    let mut stmt = conn.prepare(
        "SELECT id, text, completed, created_at, completed_at, 
                COALESCE(priority, 0), COALESCE(estimated_pomodoros, 1), COALESCE(actual_pomodoros, 0)
         FROM tasks WHERE id = ?1"
    ).map_err(|e| format!("Database error: {}", e))?;
    
    let task = stmt.query_row([&task_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            text: row.get(1)?,
            completed: row.get::<_, i32>(2)? != 0,
            created_at: row.get(3)?,
            completed_at: row.get(4)?,
            priority: row.get::<_, Option<i32>>(5)?.unwrap_or(0),
            estimated_pomodoros: row.get::<_, Option<i32>>(6)?.unwrap_or(1),
            actual_pomodoros: row.get::<_, Option<i32>>(7)?.unwrap_or(0),
        })
    }).map_err(|e| format!("Task not found: {}", e))?;
    
    // Get pomodoro sessions
    let mut stmt = conn.prepare(
        "SELECT id, task_id, session_type, duration_minutes, started_at, completed_at, interrupted 
         FROM pomodoro_sessions WHERE task_id = ?1 ORDER BY started_at DESC"
    ).map_err(|e| format!("Database error: {}", e))?;
    
    let session_iter = stmt.query_map([&task_id], |row| {
        Ok(PomodoroSession {
            id: row.get(0)?,
            task_id: row.get(1)?,
            session_type: row.get(2)?,
            duration_minutes: row.get::<_, u32>(3)?,
            started_at: row.get(4)?,
            completed_at: row.get(5)?,
            interrupted: row.get::<_, i32>(6)? != 0,
        })
    }).map_err(|e| format!("Database error: {}", e))?;
    
    let mut sessions = Vec::new();
    let mut total_time = 0u32;
    
    for session in session_iter {
        let session = session.map_err(|e| format!("Database error: {}", e))?;
        if session.completed_at.is_some() && !session.interrupted {
            total_time += session.duration_minutes;
        }
        sessions.push(session);
    }
    
    Ok(TaskWithStats {
        task,
        pomodoro_sessions: sessions,
        total_time_spent: total_time,
    })
}

#[tauri::command]
async fn get_daily_stats(
    app: tauri::AppHandle,
    date: String,
) -> Result<DailyStats, String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let conn = init_db(&app_data_dir)
        .map_err(|e| format!("Database error: {}", e))?;
    
    let stats = conn.query_row(
        "SELECT date, pomodoros_completed, total_work_time, tasks_completed FROM daily_stats WHERE date = ?1",
        [&date],
        |row| {
            Ok(DailyStats {
                date: row.get(0)?,
                pomodoros_completed: row.get(1)?,
                total_work_time: row.get(2)?,
                tasks_completed: row.get(3)?,
            })
        }
    ).unwrap_or_else(|_| DailyStats {
        date,
        pomodoros_completed: 0,
        total_work_time: 0,
        tasks_completed: 0,
    });
    
    Ok(stats)
}

#[tauri::command]
async fn export_data(app: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let _conn = init_db(&app_data_dir)
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Get all data
    let tasks = get_tasks(app.clone()).await?;
    
    let export_data = serde_json::json!({
        "tasks": tasks,
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "version": "2.0"
    });
    
    Ok(export_data.to_string())
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
async fn play_sound(sound_name: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    tokio::spawn(async move {
        if let Err(e) = play_sound_file(sound_name, app_handle).await {
            eprintln!("Failed to play sound: {}", e);
        }
    });
    Ok(())
}

async fn play_sound_file(sound_name: String, app_handle: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    
    // Determine file extension (default to .wav if not specified)
    let file_name = if sound_name.contains('.') {
        sound_name.clone()
    } else {
        format!("{}.wav", sound_name)
    };
    
    // Try multiple paths to find the audio file
    let mut audio_file_path: Option<PathBuf> = None;
    
    // Try 1: Resource directory (production)
    if let Ok(resource_path) = app_handle.path().resource_dir() {
        let path = resource_path.join(&file_name);
        if path.exists() {
            audio_file_path = Some(path);
        }
    }
    
    // Try 2: Static folder in project root (development)
    if audio_file_path.is_none() {
        // Get current executable directory and go to project root
        let current_dir = std::env::current_dir()?;
        let path = current_dir.join("..").join("static").join(&file_name);
        if path.exists() {
            audio_file_path = Some(path);
        }
    }
    
    // Try to read and decode the audio file
    if let Some(path) = audio_file_path {
        if let Ok(file) = std::fs::File::open(&path) {
            let source = Decoder::new(std::io::BufReader::new(file))?;
            sink.append(source);
            sink.sleep_until_end();
        }
    }
    
    Ok(())
}

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
            start_pomodoro_session,
            complete_pomodoro_session,
            get_task_with_stats,
            get_daily_stats,
            export_data,
            get_settings,
            save_settings,
            play_notification_sound,
            play_sound
        ])
        .setup(|app| {
            // Initialize database on app startup
            let app_data_dir = app.path().app_data_dir()
                .map_err(|e| format!("Failed to get app data directory: {}", e))?;
            init_db(&app_data_dir)
                .map_err(|e| format!("Failed to initialize database: {}", e))?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}