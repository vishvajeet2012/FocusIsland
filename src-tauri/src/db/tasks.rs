use super::{
    models::{Task, TaskInput, TaskUpdate},
    now_ms,
};
use rusqlite::{params, Connection, Row};

pub fn list(connection: &Connection) -> Result<Vec<Task>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id,title,notes,completed,created_at,updated_at,due_at,reminder_at,estimated_focus_minutes,sort_order,archived_at
             FROM tasks WHERE archived_at IS NULL ORDER BY completed ASC, sort_order ASC, created_at ASC",
        )
        .map_err(db_error)?;
    let tasks = statement
        .query_map([], task_from_row)
        .map_err(db_error)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(db_error)?;
    Ok(tasks)
}

pub fn create(connection: &Connection, input: TaskInput) -> Result<Task, String> {
    let (title, notes, estimate) =
        validate_input(&input.title, input.notes, input.estimated_focus_minutes)?;
    let now = now_ms();
    let sort_order: i64 = connection
        .query_row("SELECT COALESCE(MAX(sort_order), -1) + 1 FROM tasks WHERE archived_at IS NULL AND completed = 0", [], |row| row.get(0))
        .map_err(db_error)?;
    connection
        .execute(
            "INSERT INTO tasks(title,notes,completed,created_at,updated_at,due_at,estimated_focus_minutes,sort_order)
             VALUES(?1,?2,0,?3,?3,?4,?5,?6)",
            params![title, notes, now, input.due_at, estimate, sort_order],
        )
        .map_err(db_error)?;
    get(connection, connection.last_insert_rowid())
}

pub fn update(connection: &Connection, input: TaskUpdate) -> Result<Task, String> {
    let (title, notes, estimate) =
        validate_input(&input.title, input.notes, input.estimated_focus_minutes)?;
    let changed = connection
        .execute(
            "UPDATE tasks SET title=?1, notes=?2, due_at=?3, estimated_focus_minutes=?4, updated_at=?5 WHERE id=?6 AND archived_at IS NULL",
            params![title, notes, input.due_at, estimate, now_ms(), input.id],
        )
        .map_err(db_error)?;
    if changed == 0 {
        return Err("Task not found".into());
    }
    get(connection, input.id)
}

pub fn set_completed(connection: &Connection, id: i64, completed: bool) -> Result<Task, String> {
    let now = now_ms();
    let completed_at = completed.then_some(now);
    let changed = connection
        .execute(
            "UPDATE tasks SET completed=?1, completed_at=?2, updated_at=?3 WHERE id=?4 AND archived_at IS NULL",
            params![completed, completed_at, now, id],
        )
        .map_err(db_error)?;
    if changed == 0 {
        return Err("Task not found".into());
    }
    get(connection, id)
}

pub fn delete(connection: &Connection, id: i64) -> Result<(), String> {
    let changed = connection
        .execute("DELETE FROM tasks WHERE id=?1", [id])
        .map_err(db_error)?;
    if changed == 0 {
        return Err("Task not found".into());
    }
    Ok(())
}

pub fn duplicate(connection: &Connection, id: i64) -> Result<Task, String> {
    let source = get(connection, id)?;
    let suffix = " copy";
    let maximum = 240usize.saturating_sub(suffix.len());
    let mut title: String = source.title.chars().take(maximum).collect();
    title.push_str(suffix);
    create(
        connection,
        TaskInput {
            title,
            notes: source.notes,
            due_at: source.due_at,
            estimated_focus_minutes: source.estimated_focus_minutes,
        },
    )
}

pub fn reorder(connection: &mut Connection, ids: &[i64]) -> Result<(), String> {
    if ids.len() > 10_000 {
        return Err("Too many tasks to reorder".into());
    }
    let transaction = connection.transaction().map_err(db_error)?;
    for (index, id) in ids.iter().enumerate() {
        transaction
            .execute(
                "UPDATE tasks SET sort_order=?1, updated_at=?2 WHERE id=?3 AND completed=0 AND archived_at IS NULL",
                params![index as i64, now_ms(), id],
            )
            .map_err(db_error)?;
    }
    transaction.commit().map_err(db_error)
}

pub fn archive_completed(connection: &Connection) -> Result<i64, String> {
    let now = now_ms();
    connection
        .execute("UPDATE tasks SET archived_at=?1, updated_at=?1 WHERE completed=1 AND archived_at IS NULL", [now])
        .map(|count| count as i64)
        .map_err(db_error)
}

pub fn get(connection: &Connection, id: i64) -> Result<Task, String> {
    connection
        .query_row(
            "SELECT id,title,notes,completed,created_at,updated_at,due_at,reminder_at,estimated_focus_minutes,sort_order,archived_at FROM tasks WHERE id=?1",
            [id],
            task_from_row,
        )
        .map_err(|error| match error {
            rusqlite::Error::QueryReturnedNoRows => "Task not found".into(),
            _ => db_error(error),
        })
}

fn task_from_row(row: &Row<'_>) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        title: row.get(1)?,
        notes: row.get(2)?,
        completed: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
        due_at: row.get(6)?,
        reminder_at: row.get(7)?,
        estimated_focus_minutes: row.get(8)?,
        sort_order: row.get(9)?,
        archived_at: row.get(10)?,
    })
}

fn validate_input(
    title: &str,
    notes: Option<String>,
    estimate: Option<i64>,
) -> Result<(String, Option<String>, Option<i64>), String> {
    let title = title.trim().to_string();
    if title.is_empty() {
        return Err("Task title cannot be empty".into());
    }
    if title.chars().count() > 240 {
        return Err("Task title is limited to 240 characters".into());
    }
    let notes = notes
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    if notes
        .as_ref()
        .is_some_and(|value| value.chars().count() > 4_000)
    {
        return Err("Task notes are limited to 4,000 characters".into());
    }
    if estimate.is_some_and(|minutes| !(1..=180).contains(&minutes)) {
        return Err("Focus estimate must be between 1 and 180 minutes".into());
    }
    Ok((title, notes, estimate))
}

fn db_error(error: rusqlite::Error) -> String {
    format!("Task database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    #[test]
    fn task_crud_survives_repository_round_trip() {
        let database = Database::in_memory().unwrap();
        let connection = database.lock().unwrap();
        let task = create(
            &connection,
            TaskInput {
                title: "Ship release".into(),
                notes: None,
                due_at: None,
                estimated_focus_minutes: Some(25),
            },
        )
        .unwrap();
        assert_eq!(list(&connection).unwrap().len(), 1);
        let completed = set_completed(&connection, task.id, true).unwrap();
        assert!(completed.completed);
        delete(&connection, task.id).unwrap();
        assert!(list(&connection).unwrap().is_empty());
    }
}
