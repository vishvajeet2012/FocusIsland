use crate::{
    db::{self, models::BootstrapPayload, Database},
    shortcuts::ShortcutWarnings,
};
use tauri::State;

#[tauri::command]
pub fn bootstrap_app(
    date: String,
    database: State<'_, Database>,
    warnings: State<'_, ShortcutWarnings>,
) -> Result<BootstrapPayload, String> {
    db::validate_date_key(&date)?;
    let mut connection = database.lock()?;
    let _ = db::timer::reconcile(&mut connection, db::now_ms())?;
    Ok(BootstrapPayload {
        tasks: db::tasks::list(&connection)?,
        note: db::notes::get_or_create(&connection, &date)?,
        reminders: db::reminders::list_pending(&connection)?,
        timer: db::timer::get(&connection)?,
        settings: db::settings::load(&connection)?,
        shortcut_warnings: warnings.list(),
    })
}
