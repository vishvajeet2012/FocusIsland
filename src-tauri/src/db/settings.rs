use super::{models::AppSettings, now_ms};
use rusqlite::{params, Connection, OptionalExtension};

const SETTINGS_KEY: &str = "app";

pub fn load(connection: &Connection) -> Result<AppSettings, String> {
    let stored: Option<String> = connection
        .query_row(
            "SELECT value FROM settings WHERE key=?1",
            [SETTINGS_KEY],
            |row| row.get(0),
        )
        .optional()
        .map_err(db_error)?;
    match stored {
        Some(json) => match serde_json::from_str::<AppSettings>(&json) {
            Ok(settings) if settings.validate().is_ok() => Ok(settings),
            _ => {
                let defaults = AppSettings::default();
                save(connection, &defaults)?;
                Ok(defaults)
            }
        },
        None => {
            let defaults = AppSettings::default();
            save(connection, &defaults)?;
            Ok(defaults)
        }
    }
}

pub fn save(connection: &Connection, settings: &AppSettings) -> Result<(), String> {
    settings.validate()?;
    let value = serde_json::to_string(settings)
        .map_err(|error| format!("Cannot encode settings: {error}"))?;
    connection
        .execute(
            "INSERT INTO settings(key,value,updated_at) VALUES(?1,?2,?3)
             ON CONFLICT(key) DO UPDATE SET value=excluded.value,updated_at=excluded.updated_at",
            params![SETTINGS_KEY, value, now_ms()],
        )
        .map_err(db_error)?;
    Ok(())
}

fn db_error(error: rusqlite::Error) -> String {
    format!("Settings database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    #[test]
    fn appearance_settings_round_trip() {
        let database = Database::in_memory().unwrap();
        let connection = database.lock().unwrap();
        let mut settings = AppSettings::default();
        settings.task_card_color = "#abcdef".into();
        save(&connection, &settings).unwrap();
        assert_eq!(load(&connection).unwrap().task_card_color, "#abcdef");
    }
}
