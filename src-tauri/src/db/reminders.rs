use super::{
    models::{Reminder, ReminderInput},
    now_ms,
};
use rusqlite::{params, Connection};

pub fn list_pending(connection: &Connection) -> Result<Vec<Reminder>, String> {
    let mut statement = connection
        .prepare("SELECT id,task_id,title,remind_at,completed,created_at FROM reminders WHERE completed=0 ORDER BY remind_at ASC")
        .map_err(db_error)?;
    let reminders = statement
        .query_map([], |row| {
            Ok(Reminder {
                id: row.get(0)?,
                task_id: row.get(1)?,
                title: row.get(2)?,
                remind_at: row.get(3)?,
                completed: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(db_error)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(db_error)?;
    Ok(reminders)
}

pub fn create(connection: &mut Connection, input: ReminderInput) -> Result<Reminder, String> {
    let title = input.title.trim();
    if title.is_empty() || title.chars().count() > 240 {
        return Err("Reminder title must be between 1 and 240 characters".into());
    }
    if input.remind_at <= now_ms() {
        return Err("Reminder time must be in the future".into());
    }
    if input.remind_at > now_ms() + 10 * 366 * 24 * 60 * 60 * 1_000 {
        return Err("Reminder time is too far in the future".into());
    }
    let transaction = connection.transaction().map_err(db_error)?;
    if let Some(task_id) = input.task_id {
        let exists: bool = transaction
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM tasks WHERE id=?1)",
                [task_id],
                |row| row.get(0),
            )
            .map_err(db_error)?;
        if !exists {
            return Err("Task not found".into());
        }
    }
    let created_at = now_ms();
    transaction
        .execute("INSERT INTO reminders(task_id,title,remind_at,completed,created_at) VALUES(?1,?2,?3,0,?4)", params![input.task_id, title, input.remind_at, created_at])
        .map_err(db_error)?;
    let id = transaction.last_insert_rowid();
    if let Some(task_id) = input.task_id {
        transaction
            .execute(
                "UPDATE tasks SET reminder_at=?1,updated_at=?2 WHERE id=?3",
                params![input.remind_at, created_at, task_id],
            )
            .map_err(db_error)?;
    }
    transaction.commit().map_err(db_error)?;
    Ok(Reminder {
        id,
        task_id: input.task_id,
        title: title.into(),
        remind_at: input.remind_at,
        completed: false,
        created_at,
    })
}

pub fn delete(connection: &mut Connection, id: i64) -> Result<(), String> {
    let transaction = connection.transaction().map_err(db_error)?;
    let task_id: Option<i64> = transaction
        .query_row("SELECT task_id FROM reminders WHERE id=?1", [id], |row| {
            row.get(0)
        })
        .unwrap_or(None);
    let changed = transaction
        .execute("DELETE FROM reminders WHERE id=?1", [id])
        .map_err(db_error)?;
    if changed == 0 {
        return Err("Reminder not found".into());
    }
    if let Some(task_id) = task_id {
        transaction
            .execute(
                "UPDATE tasks SET reminder_at=NULL,updated_at=?1 WHERE id=?2",
                params![now_ms(), task_id],
            )
            .map_err(db_error)?;
    }
    transaction.commit().map_err(db_error)
}

pub fn due(connection: &Connection, now: i64) -> Result<Vec<Reminder>, String> {
    let mut statement = connection
        .prepare("SELECT id,task_id,title,remind_at,completed,created_at FROM reminders WHERE completed=0 AND remind_at<=?1 ORDER BY remind_at ASC")
        .map_err(db_error)?;
    let reminders = statement
        .query_map([now], |row| {
            Ok(Reminder {
                id: row.get(0)?,
                task_id: row.get(1)?,
                title: row.get(2)?,
                remind_at: row.get(3)?,
                completed: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(db_error)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(db_error)?;
    Ok(reminders)
}

pub fn mark_fired(connection: &Connection, reminder: &Reminder) -> Result<(), String> {
    connection
        .execute(
            "UPDATE reminders SET completed=1 WHERE id=?1",
            [reminder.id],
        )
        .map_err(db_error)?;
    if let Some(task_id) = reminder.task_id {
        connection
            .execute(
                "UPDATE tasks SET reminder_at=NULL,updated_at=?1 WHERE id=?2",
                params![now_ms(), task_id],
            )
            .map_err(db_error)?;
    }
    Ok(())
}

pub fn next_due_at(connection: &Connection) -> Result<Option<i64>, String> {
    connection
        .query_row(
            "SELECT MIN(remind_at) FROM reminders WHERE completed=0",
            [],
            |row| row.get(0),
        )
        .map_err(db_error)
}

fn db_error(error: rusqlite::Error) -> String {
    format!("Reminder database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    #[test]
    fn reminder_is_persisted_and_ordered() {
        let database = Database::in_memory().unwrap();
        let mut connection = database.lock().unwrap();
        let reminder = create(
            &mut connection,
            ReminderInput {
                task_id: None,
                title: "Stand up".into(),
                remind_at: now_ms() + 60_000,
            },
        )
        .unwrap();
        assert_eq!(list_pending(&connection).unwrap()[0].id, reminder.id);
    }
}
