use crate::db::{self, models::DailyNote, Database};
use tauri::State;

#[tauri::command]
pub fn save_daily_note(
    database: State<'_, Database>,
    date: String,
    content: String,
) -> Result<DailyNote, String> {
    let connection = database.lock()?;
    db::notes::save(&connection, &date, &content)
}
