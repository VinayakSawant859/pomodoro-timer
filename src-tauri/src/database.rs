use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

pub type DbPool = Pool<SqliteConnectionManager>;

const DB_VERSION: i32 = 2;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PomodoroSession {
    pub id: String,
    pub task_id: Option<String>,
    pub session_type: String,
    pub duration_minutes: u32,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub interrupted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyStats {
    pub date: String,
    pub pomodoros_completed: u32,
    pub total_work_time: u32,
    pub tasks_completed: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskWithStats {
    pub task: Task,
    pub pomodoro_sessions: Vec<PomodoroSession>,
    pub total_time_spent: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeatmapPoint {
    pub date: String,
    pub count: u32,
    pub level: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub fn initialize_database(app_handle: &AppHandle) -> Result<DbPool, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    let db_path = app_data_dir.join("pomodoro.db");
    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager).map_err(|e| format!("Failed to create connection pool: {}", e))?;

    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;
    migrate_database(&conn)?;

    Ok(pool)
}

fn migrate_database(conn: &rusqlite::Connection) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS db_version (version INTEGER PRIMARY KEY)",
        [],
    )
    .map_err(|e| format!("Failed to create version table: {}", e))?;

    let current_version: i32 = conn
        .query_row(
            "SELECT version FROM db_version ORDER BY version DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if current_version < DB_VERSION {
        for version in (current_version + 1)..=DB_VERSION {
            match version {
                1 => {
                    conn.execute(
                        "CREATE TABLE IF NOT EXISTS tasks (
                            id TEXT PRIMARY KEY,
                            text TEXT NOT NULL,
                            completed BOOLEAN NOT NULL DEFAULT 0,
                            created_at TEXT NOT NULL,
                            completed_at TEXT
                        )",
                        [],
                    )
                    .map_err(|e| format!("Failed to create tasks table: {}", e))?;
                }
                2 => {
                    conn.execute(
                        "ALTER TABLE tasks ADD COLUMN priority INTEGER DEFAULT 0",
                        [],
                    )
                    .ok();

                    conn.execute(
                        "ALTER TABLE tasks ADD COLUMN estimated_pomodoros INTEGER DEFAULT 1",
                        [],
                    )
                    .ok();

                    conn.execute(
                        "ALTER TABLE tasks ADD COLUMN actual_pomodoros INTEGER DEFAULT 0",
                        [],
                    )
                    .ok();

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
                    )
                    .map_err(|e| format!("Failed to create pomodoro_sessions table: {}", e))?;

                    conn.execute(
                        "CREATE TABLE IF NOT EXISTS daily_stats (
                            date TEXT PRIMARY KEY,
                            pomodoros_completed INTEGER DEFAULT 0,
                            total_work_time INTEGER DEFAULT 0,
                            tasks_completed INTEGER DEFAULT 0,
                            created_at TEXT NOT NULL
                        )",
                        [],
                    )
                    .map_err(|e| format!("Failed to create daily_stats table: {}", e))?;

                    conn.execute(
                        "CREATE TABLE IF NOT EXISTS settings (
                            key TEXT PRIMARY KEY,
                            value TEXT NOT NULL,
                            updated_at TEXT NOT NULL
                        )",
                        [],
                    )
                    .map_err(|e| format!("Failed to create settings table: {}", e))?;
                }
                _ => {}
            }
        }

        conn.execute(
            "INSERT OR REPLACE INTO db_version (version) VALUES (?1)",
            [DB_VERSION],
        )
        .map_err(|e| format!("Failed to update version: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn add_task(state: State<'_, DbPool>, text: String) -> Result<Task, String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

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
        params![
            &task.id,
            &task.text,
            &task.completed,
            &task.created_at,
            &task.priority,
            &task.estimated_pomodoros,
            &task.actual_pomodoros
        ],
    )
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(task)
}

#[tauri::command]
pub async fn get_tasks(state: State<'_, DbPool>) -> Result<Vec<Task>, String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, text, completed, created_at, completed_at, 
                    COALESCE(priority, 0), COALESCE(estimated_pomodoros, 1), COALESCE(actual_pomodoros, 0)
             FROM tasks ORDER BY priority DESC, created_at DESC",
        )
        .map_err(|e| format!("Database error: {}", e))?;

    let task_iter = stmt
        .query_map([], |row| {
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
        })
        .map_err(|e| format!("Database error: {}", e))?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task.map_err(|e| format!("Database error: {}", e))?);
    }

    Ok(tasks)
}

#[tauri::command]
pub async fn complete_task(
    state: State<'_, DbPool>,
    task_id: String,
    completed: bool,
) -> Result<(), String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    let completed_at = if completed {
        Some(chrono::Utc::now().to_rfc3339())
    } else {
        None
    };

    conn.execute(
        "UPDATE tasks SET completed = ?1, completed_at = ?2 WHERE id = ?3",
        params![completed, completed_at, task_id],
    )
    .map_err(|e| format!("Database error: {}", e))?;

    if completed {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        conn.execute(
            "INSERT OR REPLACE INTO daily_stats (date, tasks_completed, created_at) 
             VALUES (?1, COALESCE((SELECT tasks_completed FROM daily_stats WHERE date = ?1), 0) + 1, ?2)",
            params![&today, &chrono::Utc::now().to_rfc3339()],
        )
        .map_err(|e| format!("Database error: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn update_task(
    state: State<'_, DbPool>,
    task_id: String,
    text: String,
    priority: i32,
    estimated_pomodoros: i32,
) -> Result<(), String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    conn.execute(
        "UPDATE tasks SET text = ?1, priority = ?2, estimated_pomodoros = ?3 WHERE id = ?4",
        params![text, priority, estimated_pomodoros, task_id],
    )
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_task(state: State<'_, DbPool>, task_id: String) -> Result<(), String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    conn.execute("DELETE FROM tasks WHERE id = ?1", params![task_id])
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn start_pomodoro_session(
    state: State<'_, DbPool>,
    task_id: Option<String>,
    session_type: String,
    duration_minutes: u32,
) -> Result<String, String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    let session_id = uuid::Uuid::new_v4().to_string();
    let started_at = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO pomodoro_sessions (id, task_id, session_type, duration_minutes, started_at, interrupted) 
         VALUES (?1, ?2, ?3, ?4, ?5, 0)",
        params![session_id, task_id, session_type, duration_minutes, started_at],
    )
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(session_id)
}

#[tauri::command]
pub async fn complete_pomodoro_session(
    state: State<'_, DbPool>,
    session_id: String,
    was_completed: bool,
    was_interrupted: bool,
) -> Result<(), String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    let completed_at = if was_completed {
        Some(chrono::Utc::now().to_rfc3339())
    } else {
        None
    };

    conn.execute(
        "UPDATE pomodoro_sessions SET completed_at = ?1, interrupted = ?2 WHERE id = ?3",
        params![completed_at, was_interrupted, session_id],
    )
    .map_err(|e| format!("Database error: {}", e))?;

    let session_info: (String, Option<String>) = conn
        .query_row(
            "SELECT session_type, task_id FROM pomodoro_sessions WHERE id = ?1",
            params![session_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("Database error: {}", e))?;

    let (session_type, task_id) = session_info;

    if session_type == "work" && was_completed && !was_interrupted {
        if let Some(tid) = task_id {
            conn.execute(
                "UPDATE tasks SET actual_pomodoros = actual_pomodoros + 1 WHERE id = ?1",
                params![tid],
            )
            .map_err(|e| format!("Database error: {}", e))?;
        }

        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        conn.execute(
            "INSERT INTO daily_stats (date, pomodoros_completed, total_work_time, created_at)
             VALUES (?1, 1, ?2, ?3)
             ON CONFLICT(date) DO UPDATE SET
                pomodoros_completed = pomodoros_completed + 1,
                total_work_time = total_work_time + ?2",
            params![&today, 25, &chrono::Utc::now().to_rfc3339()],
        )
        .map_err(|e| format!("Database error: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_task_with_stats(
    state: State<'_, DbPool>,
    task_id: String,
) -> Result<TaskWithStats, String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    let task: Task = conn
        .query_row(
            "SELECT id, text, completed, created_at, completed_at, 
                    COALESCE(priority, 0), COALESCE(estimated_pomodoros, 1), COALESCE(actual_pomodoros, 0)
             FROM tasks WHERE id = ?1",
            params![task_id],
            |row| {
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
            },
        )
        .map_err(|e| format!("Database error: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, task_id, session_type, duration_minutes, started_at, completed_at, interrupted
             FROM pomodoro_sessions WHERE task_id = ?1 ORDER BY started_at DESC",
        )
        .map_err(|e| format!("Database error: {}", e))?;

    let session_iter = stmt
        .query_map(params![task_id], |row| {
            Ok(PomodoroSession {
                id: row.get(0)?,
                task_id: row.get(1)?,
                session_type: row.get(2)?,
                duration_minutes: row.get(3)?,
                started_at: row.get(4)?,
                completed_at: row.get(5)?,
                interrupted: row.get::<_, i32>(6)? != 0,
            })
        })
        .map_err(|e| format!("Database error: {}", e))?;

    let mut pomodoro_sessions = Vec::new();
    let mut total_time_spent = 0u32;

    for session in session_iter {
        let session = session.map_err(|e| format!("Database error: {}", e))?;
        if session.session_type == "work" && !session.interrupted {
            total_time_spent += session.duration_minutes;
        }
        pomodoro_sessions.push(session);
    }

    Ok(TaskWithStats {
        task,
        pomodoro_sessions,
        total_time_spent,
    })
}

#[tauri::command]
pub async fn get_daily_stats(state: State<'_, DbPool>) -> Result<Vec<DailyStats>, String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT date, pomodoros_completed, total_work_time, tasks_completed 
             FROM daily_stats ORDER BY date DESC LIMIT 30",
        )
        .map_err(|e| format!("Database error: {}", e))?;

    let stats_iter = stmt
        .query_map([], |row| {
            Ok(DailyStats {
                date: row.get(0)?,
                pomodoros_completed: row.get(1)?,
                total_work_time: row.get(2)?,
                tasks_completed: row.get(3)?,
            })
        })
        .map_err(|e| format!("Database error: {}", e))?;

    let mut stats = Vec::new();
    for stat in stats_iter {
        stats.push(stat.map_err(|e| format!("Database error: {}", e))?);
    }

    Ok(stats)
}

#[tauri::command]
pub async fn get_daily_stats_by_date(
    state: State<'_, DbPool>,
    date: String,
) -> Result<DailyStats, String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT date, pomodoros_completed, total_work_time, tasks_completed 
             FROM daily_stats WHERE date = ?1",
        )
        .map_err(|e| format!("Database error: {}", e))?;

    let result = stmt.query_row([&date], |row| {
        Ok(DailyStats {
            date: row.get(0)?,
            pomodoros_completed: row.get(1)?,
            total_work_time: row.get(2)?,
            tasks_completed: row.get(3)?,
        })
    });

    match result {
        Ok(stats) => Ok(stats),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // No stats for this date, return empty stats
            Ok(DailyStats {
                date,
                pomodoros_completed: 0,
                total_work_time: 0,
                tasks_completed: 0,
            })
        }
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[tauri::command]
pub async fn get_focus_heatmap(
    state: State<'_, DbPool>,
    days: Option<u32>,
) -> Result<Vec<HeatmapPoint>, String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    let days_limit = days.unwrap_or(365);
    let start_date = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(days_limit as i64))
        .unwrap_or_else(chrono::Utc::now)
        .format("%Y-%m-%d")
        .to_string();

    let mut stmt = conn
        .prepare(
            "SELECT DATE(started_at) as date, COUNT(*) as count
             FROM pomodoro_sessions 
             WHERE session_type = 'work' 
               AND interrupted = 0 
               AND completed_at IS NOT NULL
               AND DATE(started_at) >= ?1
             GROUP BY DATE(started_at)
             ORDER BY date ASC",
        )
        .map_err(|e| format!("Database error: {}", e))?;

    let heatmap_iter = stmt
        .query_map(params![start_date], |row| {
            let count: u32 = row.get(1)?;
            let level = match count {
                0 => 0,
                1..=2 => 1,
                3..=5 => 2,
                6..=9 => 3,
                _ => 4,
            };
            Ok(HeatmapPoint {
                date: row.get(0)?,
                count,
                level,
            })
        })
        .map_err(|e| format!("Database error: {}", e))?;

    let mut heatmap = Vec::new();
    for point in heatmap_iter {
        heatmap.push(point.map_err(|e| format!("Database error: {}", e))?);
    }

    Ok(heatmap)
}

#[tauri::command]
pub async fn export_data(state: State<'_, DbPool>) -> Result<serde_json::Value, String> {
    let pool = state.inner();
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;

    let mut tasks_stmt = conn
        .prepare("SELECT * FROM tasks ORDER BY created_at DESC")
        .map_err(|e| format!("Database error: {}", e))?;

    let tasks_iter = tasks_stmt
        .query_map([], |row| {
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
        })
        .map_err(|e| format!("Database error: {}", e))?;

    let mut tasks = Vec::new();
    for task in tasks_iter {
        tasks.push(task.map_err(|e| format!("Database error: {}", e))?);
    }

    let mut sessions_stmt = conn
        .prepare("SELECT * FROM pomodoro_sessions ORDER BY started_at DESC")
        .map_err(|e| format!("Database error: {}", e))?;

    let sessions_iter = sessions_stmt
        .query_map([], |row| {
            Ok(PomodoroSession {
                id: row.get(0)?,
                task_id: row.get(1)?,
                session_type: row.get(2)?,
                duration_minutes: row.get(3)?,
                started_at: row.get(4)?,
                completed_at: row.get(5)?,
                interrupted: row.get::<_, i32>(6)? != 0,
            })
        })
        .map_err(|e| format!("Database error: {}", e))?;

    let mut sessions = Vec::new();
    for session in sessions_iter {
        sessions.push(session.map_err(|e| format!("Database error: {}", e))?);
    }

    let mut stats_stmt = conn
        .prepare("SELECT * FROM daily_stats ORDER BY date DESC")
        .map_err(|e| format!("Database error: {}", e))?;

    let stats_iter = stats_stmt
        .query_map([], |row| {
            Ok(DailyStats {
                date: row.get(0)?,
                pomodoros_completed: row.get(1)?,
                total_work_time: row.get(2)?,
                tasks_completed: row.get(3)?,
            })
        })
        .map_err(|e| format!("Database error: {}", e))?;

    let mut daily_stats = Vec::new();
    for stat in stats_iter {
        daily_stats.push(stat.map_err(|e| format!("Database error: {}", e))?);
    }

    Ok(serde_json::json!({
        "tasks": tasks,
        "pomodoro_sessions": sessions,
        "daily_stats": daily_stats,
        "exported_at": chrono::Utc::now().to_rfc3339()
    }))
}
