use crate::db::{self, models::Insights, Database};
use tauri::State;

#[tauri::command]
pub fn get_insights(database: State<'_, Database>, date: String) -> Result<Insights, String> {
    let connection = database.lock()?;
    db::insights::get(&connection, &date)
}
