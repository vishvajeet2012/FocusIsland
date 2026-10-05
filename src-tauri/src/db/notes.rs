use super::{models::DailyNote, now_ms, validate_date_key};
use rusqlite::{params, Connection};

pub fn get_or_create(connection: &Connection, date: &str) -> Result<DailyNote, String> {
    validate_date_key(date)?;
    if let Some(note) = get(connection, date)? {
        return Ok(note);
    }
    let now = now_ms();
    connection
        .execute("INSERT OR IGNORE INTO daily_notes(date,content,created_at,updated_at) VALUES(?1,'',?2,?2)", params![date, now])
        .map_err(db_error)?;
    get(connection, date)?.ok_or_else(|| "Could not create daily note".into())
}

pub fn save(connection: &Connection, date: &str, content: &str) -> Result<DailyNote, String> {
    validate_date_key(date)?;
    if content.chars().count() > 100_000 {
        return Err("Daily note is limited to 100,000 characters".into());
    }
    let now = now_ms();
    connection
        .execute(
            "INSERT INTO daily_notes(date,content,created_at,updated_at) VALUES(?1,?2,?3,?3)
             ON CONFLICT(date) DO UPDATE SET content=excluded.content, updated_at=excluded.updated_at",
            params![date, content, now],
        )
        .map_err(db_error)?;
    get(connection, date)?.ok_or_else(|| "Could not save daily note".into())
}

fn get(connection: &Connection, date: &str) -> Result<Option<DailyNote>, String> {
    let result = connection.query_row(
        "SELECT id,date,content,created_at,updated_at FROM daily_notes WHERE date=?1",
        [date],
        |row| {
            Ok(DailyNote {
                id: row.get(0)?,
                date: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        },
    );
    match result {
        Ok(note) => Ok(Some(note)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(db_error(error)),
    }
}

fn db_error(error: rusqlite::Error) -> String {
    format!("Note database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    #[test]
    fn daily_note_upsert_restores_content() {
        let database = Database::in_memory().unwrap();
        let connection = database.lock().unwrap();
        save(&connection, "2026-10-04", "A durable thought").unwrap();
        assert_eq!(
            get_or_create(&connection, "2026-10-04").unwrap().content,
            "A durable thought"
        );
    }
}
