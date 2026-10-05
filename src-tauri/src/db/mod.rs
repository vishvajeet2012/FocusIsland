pub mod insights;
pub mod models;
pub mod notes;
pub mod reminders;
pub mod settings;
pub mod tasks;
pub mod timer;

use rusqlite::Connection;
use std::{
    path::Path,
    sync::{Mutex, MutexGuard},
    time::{SystemTime, UNIX_EPOCH},
};

pub struct Database {
    connection: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("Cannot create data directory: {error}"))?;
        }
        let connection =
            Connection::open(path).map_err(|error| format!("Cannot open database: {error}"))?;
        initialize(&connection)?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn in_memory() -> Result<Self, String> {
        let connection = Connection::open_in_memory().map_err(|error| error.to_string())?;
        initialize(&connection)?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn lock(&self) -> Result<MutexGuard<'_, Connection>, String> {
        self.connection
            .lock()
            .map_err(|_| "Local database lock was poisoned".into())
    }
}

fn initialize(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA busy_timeout = 2500;

            CREATE TABLE IF NOT EXISTS schema_migrations (
              version INTEGER PRIMARY KEY,
              applied_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tasks (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              title TEXT NOT NULL,
              notes TEXT,
              completed INTEGER NOT NULL DEFAULT 0,
              completed_at INTEGER,
              created_at INTEGER NOT NULL,
              updated_at INTEGER NOT NULL,
              due_at INTEGER,
              reminder_at INTEGER,
              estimated_focus_minutes INTEGER,
              sort_order INTEGER NOT NULL DEFAULT 0,
              archived_at INTEGER
            );

            CREATE TABLE IF NOT EXISTS daily_notes (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              date TEXT NOT NULL UNIQUE,
              content TEXT NOT NULL DEFAULT '',
              created_at INTEGER NOT NULL,
              updated_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS focus_sessions (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              task_id INTEGER REFERENCES tasks(id) ON DELETE SET NULL,
              mode TEXT NOT NULL CHECK(mode IN ('countdown', 'stopwatch')),
              started_at INTEGER NOT NULL,
              ended_at INTEGER NOT NULL,
              duration_seconds INTEGER NOT NULL,
              planned_duration_seconds INTEGER,
              completed INTEGER NOT NULL DEFAULT 1,
              created_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS reminders (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              task_id INTEGER REFERENCES tasks(id) ON DELETE CASCADE,
              title TEXT NOT NULL,
              remind_at INTEGER NOT NULL,
              completed INTEGER NOT NULL DEFAULT 0,
              created_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS settings (
              key TEXT PRIMARY KEY,
              value TEXT NOT NULL,
              updated_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS timer_state (
              id INTEGER PRIMARY KEY CHECK(id = 1),
              mode TEXT NOT NULL DEFAULT 'countdown',
              status TEXT NOT NULL DEFAULT 'ready',
              started_at INTEGER,
              paused_at INTEGER,
              accumulated_pause_ms INTEGER NOT NULL DEFAULT 0,
              target_duration_seconds INTEGER,
              task_id INTEGER REFERENCES tasks(id) ON DELETE SET NULL,
              task_title TEXT,
              session_started_at INTEGER
            );

            INSERT OR IGNORE INTO timer_state (id, mode, status, target_duration_seconds)
            VALUES (1, 'countdown', 'ready', 1500);

            CREATE INDEX IF NOT EXISTS idx_tasks_completed_created ON tasks(completed, created_at);
            CREATE INDEX IF NOT EXISTS idx_tasks_archived_sort ON tasks(archived_at, sort_order);
            CREATE INDEX IF NOT EXISTS idx_focus_sessions_started ON focus_sessions(started_at);
            CREATE INDEX IF NOT EXISTS idx_reminders_due ON reminders(completed, remind_at);
            INSERT OR IGNORE INTO schema_migrations(version, applied_at) VALUES (1, 0);
            "#,
        )
        .map_err(|error| format!("Database migration failed: {error}"))?;
    Ok(())
}

pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

pub fn validate_date_key(date: &str) -> Result<(), String> {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| "Date must use YYYY-MM-DD format".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::{TaskInput, TimerStartInput};

    #[test]
    fn local_state_survives_database_reopen() {
        let path = std::env::temp_dir().join(format!(
            "focusisland-restart-{}-{}.sqlite3",
            std::process::id(),
            now_ms()
        ));

        {
            let database = Database::open(&path).unwrap();
            let connection = database.lock().unwrap();
            tasks::create(
                &connection,
                TaskInput {
                    title: "Persist across restart".into(),
                    notes: None,
                    due_at: None,
                    estimated_focus_minutes: Some(25),
                },
            )
            .unwrap();
            notes::save(&connection, "2026-10-04", "Still here").unwrap();
            let mut preferences = models::AppSettings::default();
            preferences.task_card_color = "#abcdef".into();
            settings::save(&connection, &preferences).unwrap();
            timer::start(
                &connection,
                TimerStartInput {
                    mode: "countdown".into(),
                    duration_seconds: Some(1_500),
                    task_id: None,
                    task_title: None,
                },
            )
            .unwrap();
            let simulated_start = now_ms() - 5_000;
            connection
                .execute(
                    "UPDATE timer_state SET started_at=?1,session_started_at=?1 WHERE id=1",
                    [simulated_start],
                )
                .unwrap();
        }

        {
            let reopened = Database::open(&path).unwrap();
            let connection = reopened.lock().unwrap();
            assert_eq!(tasks::list(&connection).unwrap().len(), 1);
            assert_eq!(
                notes::get_or_create(&connection, "2026-10-04")
                    .unwrap()
                    .content,
                "Still here"
            );
            assert_eq!(
                settings::load(&connection).unwrap().task_card_color,
                "#abcdef"
            );
            let restored_timer = timer::get(&connection).unwrap();
            assert_eq!(restored_timer.status, "running");
            assert!((5..=6).contains(&timer::elapsed_seconds(&restored_timer, now_ms())));
        }

        for candidate in [
            path.clone(),
            path.with_file_name(format!(
                "{}-wal",
                path.file_name().unwrap().to_string_lossy()
            )),
            path.with_file_name(format!(
                "{}-shm",
                path.file_name().unwrap().to_string_lossy()
            )),
        ] {
            let _ = std::fs::remove_file(candidate);
        }
    }
}
