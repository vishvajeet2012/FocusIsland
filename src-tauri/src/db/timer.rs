use super::{
    models::{TimerSnapshot, TimerStartInput},
    now_ms,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};

pub fn get(connection: &Connection) -> Result<TimerSnapshot, String> {
    connection
        .query_row(
            "SELECT mode,status,started_at,paused_at,accumulated_pause_ms,target_duration_seconds,task_id,task_title,session_started_at FROM timer_state WHERE id=1",
            [],
            |row| Ok(TimerSnapshot {
                mode: row.get(0)?, status: row.get(1)?, started_at: row.get(2)?, paused_at: row.get(3)?,
                accumulated_pause_ms: row.get(4)?, target_duration_seconds: row.get(5)?, task_id: row.get(6)?,
                task_title: row.get(7)?, session_started_at: row.get(8)?,
            }),
        )
        .map_err(db_error)
}

pub fn start(connection: &Connection, input: TimerStartInput) -> Result<TimerSnapshot, String> {
    if !["countdown", "stopwatch"].contains(&input.mode.as_str()) {
        return Err("Timer mode must be countdown or stopwatch".into());
    }
    let current = get(connection)?;
    if current.status == "running" || current.status == "paused" {
        return Err("A focus session is already active".into());
    }
    let duration = if input.mode == "countdown" {
        let seconds = input.duration_seconds.unwrap_or(1_500);
        if !(60..=10_800).contains(&seconds) {
            return Err("Countdown must be between 1 and 180 minutes".into());
        }
        Some(seconds)
    } else {
        None
    };
    let task_title = match input.task_id {
        Some(task_id) => {
            let stored: Option<String> = connection
                .query_row(
                    "SELECT title FROM tasks WHERE id=?1 AND archived_at IS NULL",
                    [task_id],
                    |row| row.get(0),
                )
                .optional()
                .map_err(db_error)?;
            if stored.is_none() {
                return Err("Task not found".into());
            }
            input.task_title.or(stored)
        }
        None => input.task_title,
    };
    if task_title
        .as_ref()
        .is_some_and(|title| title.chars().count() > 240)
    {
        return Err("Timer task title is too long".into());
    }
    let now = now_ms();
    connection.execute(
        "UPDATE timer_state SET mode=?1,status='running',started_at=?2,paused_at=NULL,accumulated_pause_ms=0,target_duration_seconds=?3,task_id=?4,task_title=?5,session_started_at=?2 WHERE id=1",
        params![input.mode, now, duration, input.task_id, task_title],
    ).map_err(db_error)?;
    get(connection)
}

pub fn pause(connection: &Connection) -> Result<TimerSnapshot, String> {
    let changed = connection
        .execute(
            "UPDATE timer_state SET status='paused',paused_at=?1 WHERE id=1 AND status='running'",
            [now_ms()],
        )
        .map_err(db_error)?;
    if changed == 0 {
        return Err("Timer is not running".into());
    }
    get(connection)
}

pub fn resume(connection: &Connection) -> Result<TimerSnapshot, String> {
    let snapshot = get(connection)?;
    if snapshot.status != "paused" {
        return Err("Timer is not paused".into());
    }
    let now = now_ms();
    let extra_pause = now.saturating_sub(snapshot.paused_at.unwrap_or(now));
    connection.execute(
        "UPDATE timer_state SET status='running',paused_at=NULL,accumulated_pause_ms=accumulated_pause_ms+?1 WHERE id=1",
        [extra_pause],
    ).map_err(db_error)?;
    get(connection)
}

pub fn finish(connection: &mut Connection, completed: bool) -> Result<TimerSnapshot, String> {
    let transaction = connection.transaction().map_err(db_error)?;
    let snapshot = get_transaction(&transaction)?;
    if snapshot.status != "running" && snapshot.status != "paused" {
        return Err("There is no active timer to finish".into());
    }
    let now = now_ms();
    let elapsed_seconds = elapsed_seconds(&snapshot, now);
    insert_session(&transaction, &snapshot, now, elapsed_seconds, completed)?;
    transaction
        .execute(
            "UPDATE timer_state SET status='completed',paused_at=?1 WHERE id=1",
            [now],
        )
        .map_err(db_error)?;
    transaction.commit().map_err(db_error)?;
    get(connection)
}

pub fn reset(
    connection: &Connection,
    mode: &str,
    duration_seconds: i64,
) -> Result<TimerSnapshot, String> {
    if !["countdown", "stopwatch"].contains(&mode) {
        return Err("Timer mode must be countdown or stopwatch".into());
    }
    if mode == "countdown" && !(60..=10_800).contains(&duration_seconds) {
        return Err("Countdown must be between 1 and 180 minutes".into());
    }
    let current = get(connection)?;
    if current.status == "running" || current.status == "paused" {
        return Err("Finish the active timer before resetting it".into());
    }
    let target = (mode == "countdown").then_some(duration_seconds);
    connection.execute(
        "UPDATE timer_state SET mode=?1,status='ready',started_at=NULL,paused_at=NULL,accumulated_pause_ms=0,target_duration_seconds=?2,task_id=NULL,task_title=NULL,session_started_at=NULL WHERE id=1",
        params![mode, target],
    ).map_err(db_error)?;
    get(connection)
}

pub fn reconcile(connection: &mut Connection, now: i64) -> Result<Option<TimerSnapshot>, String> {
    let transaction = connection.transaction().map_err(db_error)?;
    let snapshot = get_transaction(&transaction)?;
    if snapshot.mode != "countdown" || snapshot.status != "running" {
        return Ok(None);
    }
    let target = snapshot.target_duration_seconds.unwrap_or(0);
    if elapsed_seconds(&snapshot, now) < target {
        return Ok(None);
    }
    let completed_at = due_at(&snapshot).unwrap_or(now).min(now);
    insert_session(&transaction, &snapshot, completed_at, target, true)?;
    transaction
        .execute(
            "UPDATE timer_state SET status='completed',paused_at=?1 WHERE id=1",
            [completed_at],
        )
        .map_err(db_error)?;
    transaction.commit().map_err(db_error)?;
    Ok(Some(get(connection)?))
}

pub fn next_due_at(connection: &Connection) -> Result<Option<i64>, String> {
    let snapshot = get(connection)?;
    if snapshot.mode == "countdown" && snapshot.status == "running" {
        Ok(due_at(&snapshot))
    } else {
        Ok(None)
    }
}

pub fn elapsed_seconds(snapshot: &TimerSnapshot, now: i64) -> i64 {
    let end = if snapshot.status == "paused" || snapshot.status == "completed" {
        snapshot.paused_at.unwrap_or(now)
    } else {
        now
    };
    snapshot
        .started_at
        .map(|started| {
            end.saturating_sub(started)
                .saturating_sub(snapshot.accumulated_pause_ms)
                / 1_000
        })
        .unwrap_or(0)
        .max(0)
}

fn due_at(snapshot: &TimerSnapshot) -> Option<i64> {
    Some(
        snapshot.started_at?
            + snapshot.accumulated_pause_ms
            + snapshot.target_duration_seconds? * 1_000,
    )
}

fn insert_session(
    transaction: &Transaction<'_>,
    snapshot: &TimerSnapshot,
    ended_at: i64,
    duration_seconds: i64,
    completed: bool,
) -> Result<(), String> {
    let started_at = snapshot
        .session_started_at
        .or(snapshot.started_at)
        .unwrap_or(ended_at);
    transaction.execute(
        "INSERT INTO focus_sessions(task_id,mode,started_at,ended_at,duration_seconds,planned_duration_seconds,completed,created_at) VALUES(?1,?2,?3,?4,?5,?6,?7,?4)",
        params![snapshot.task_id, snapshot.mode, started_at, ended_at, duration_seconds, snapshot.target_duration_seconds, completed],
    ).map_err(db_error)?;
    Ok(())
}

fn get_transaction(transaction: &Transaction<'_>) -> Result<TimerSnapshot, String> {
    transaction.query_row(
        "SELECT mode,status,started_at,paused_at,accumulated_pause_ms,target_duration_seconds,task_id,task_title,session_started_at FROM timer_state WHERE id=1",
        [],
        |row| Ok(TimerSnapshot { mode: row.get(0)?, status: row.get(1)?, started_at: row.get(2)?, paused_at: row.get(3)?, accumulated_pause_ms: row.get(4)?, target_duration_seconds: row.get(5)?, task_id: row.get(6)?, task_title: row.get(7)?, session_started_at: row.get(8)? }),
    ).map_err(db_error)
}

fn db_error(error: rusqlite::Error) -> String {
    format!("Timer database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    #[test]
    fn running_timer_uses_timestamps_and_reconciles_after_delay() {
        let database = Database::in_memory().unwrap();
        let mut connection = database.lock().unwrap();
        start(
            &connection,
            TimerStartInput {
                mode: "countdown".into(),
                duration_seconds: Some(60),
                task_id: None,
                task_title: None,
            },
        )
        .unwrap();
        let simulated_start = now_ms() - 61_000;
        connection
            .execute(
                "UPDATE timer_state SET started_at=?1,session_started_at=?1 WHERE id=1",
                [simulated_start],
            )
            .unwrap();
        let finished = reconcile(&mut connection, now_ms()).unwrap().unwrap();
        assert_eq!(finished.status, "completed");
        let sessions: i64 = connection
            .query_row("SELECT COUNT(*) FROM focus_sessions", [], |row| row.get(0))
            .unwrap();
        assert_eq!(sessions, 1);
    }

    #[test]
    fn paused_time_does_not_count_as_elapsed() {
        let snapshot = TimerSnapshot {
            mode: "countdown".into(),
            status: "paused".into(),
            started_at: Some(1_000),
            paused_at: Some(11_000),
            accumulated_pause_ms: 2_000,
            target_duration_seconds: Some(60),
            task_id: None,
            task_title: None,
            session_started_at: Some(1_000),
        };
        assert_eq!(elapsed_seconds(&snapshot, 50_000), 8);
    }
}
