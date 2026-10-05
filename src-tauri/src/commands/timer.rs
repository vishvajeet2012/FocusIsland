use crate::{
    db::{
        self,
        models::{TimerSnapshot, TimerStartInput},
        Database,
    },
    scheduler::SchedulerSignal,
};
use tauri::State;

#[tauri::command]
pub fn start_timer(
    database: State<'_, Database>,
    scheduler: State<'_, SchedulerSignal>,
    input: TimerStartInput,
) -> Result<TimerSnapshot, String> {
    let connection = database.lock()?;
    let timer = db::timer::start(&connection, input)?;
    drop(connection);
    scheduler.wake();
    Ok(timer)
}

#[tauri::command]
pub fn pause_timer(
    database: State<'_, Database>,
    scheduler: State<'_, SchedulerSignal>,
) -> Result<TimerSnapshot, String> {
    let connection = database.lock()?;
    let timer = db::timer::pause(&connection)?;
    drop(connection);
    scheduler.wake();
    Ok(timer)
}

#[tauri::command]
pub fn resume_timer(
    database: State<'_, Database>,
    scheduler: State<'_, SchedulerSignal>,
) -> Result<TimerSnapshot, String> {
    let connection = database.lock()?;
    let timer = db::timer::resume(&connection)?;
    drop(connection);
    scheduler.wake();
    Ok(timer)
}

#[tauri::command]
pub fn finish_timer(
    database: State<'_, Database>,
    scheduler: State<'_, SchedulerSignal>,
    completed: bool,
) -> Result<TimerSnapshot, String> {
    let mut connection = database.lock()?;
    let timer = db::timer::finish(&mut connection, completed)?;
    drop(connection);
    scheduler.wake();
    Ok(timer)
}

#[tauri::command]
pub fn reset_timer(
    database: State<'_, Database>,
    scheduler: State<'_, SchedulerSignal>,
    mode: String,
    duration_seconds: i64,
) -> Result<TimerSnapshot, String> {
    let connection = database.lock()?;
    let timer = db::timer::reset(&connection, &mode, duration_seconds)?;
    drop(connection);
    scheduler.wake();
    Ok(timer)
}
