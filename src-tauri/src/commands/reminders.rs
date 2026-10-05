use crate::{
    db::{
        self,
        models::{Reminder, ReminderInput},
        Database,
    },
    scheduler::SchedulerSignal,
};
use tauri::State;

#[tauri::command]
pub fn create_reminder(
    database: State<'_, Database>,
    scheduler: State<'_, SchedulerSignal>,
    input: ReminderInput,
) -> Result<Reminder, String> {
    let mut connection = database.lock()?;
    let reminder = db::reminders::create(&mut connection, input)?;
    drop(connection);
    scheduler.wake();
    Ok(reminder)
}

#[tauri::command]
pub fn delete_reminder(
    database: State<'_, Database>,
    scheduler: State<'_, SchedulerSignal>,
    id: i64,
) -> Result<(), String> {
    let mut connection = database.lock()?;
    db::reminders::delete(&mut connection, id)?;
    drop(connection);
    scheduler.wake();
    Ok(())
}
