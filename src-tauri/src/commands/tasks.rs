use crate::db::{
    self,
    models::{Task, TaskInput, TaskUpdate},
    Database,
};
use tauri::State;

#[tauri::command]
pub fn create_task(database: State<'_, Database>, input: TaskInput) -> Result<Task, String> {
    let connection = database.lock()?;
    db::tasks::create(&connection, input)
}

#[tauri::command]
pub fn update_task(database: State<'_, Database>, input: TaskUpdate) -> Result<Task, String> {
    let connection = database.lock()?;
    db::tasks::update(&connection, input)
}

#[tauri::command]
pub fn set_task_completed(
    database: State<'_, Database>,
    id: i64,
    completed: bool,
) -> Result<Task, String> {
    let connection = database.lock()?;
    db::tasks::set_completed(&connection, id, completed)
}

#[tauri::command]
pub fn delete_task(database: State<'_, Database>, id: i64) -> Result<(), String> {
    let connection = database.lock()?;
    db::tasks::delete(&connection, id)
}

#[tauri::command]
pub fn duplicate_task(database: State<'_, Database>, id: i64) -> Result<Task, String> {
    let connection = database.lock()?;
    db::tasks::duplicate(&connection, id)
}

#[tauri::command]
pub fn reorder_tasks(database: State<'_, Database>, ids: Vec<i64>) -> Result<(), String> {
    let mut connection = database.lock()?;
    db::tasks::reorder(&mut connection, &ids)
}

#[tauri::command]
pub fn archive_completed_tasks(database: State<'_, Database>) -> Result<i64, String> {
    let connection = database.lock()?;
    db::tasks::archive_completed(&connection)
}
