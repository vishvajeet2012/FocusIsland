use crate::{
    db::{self, models::AppSettings, Database},
    shortcuts::ShortcutWarnings,
};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
pub fn save_settings(
    app: AppHandle,
    database: State<'_, Database>,
    warnings: State<'_, ShortcutWarnings>,
    settings: AppSettings,
) -> Result<AppSettings, String> {
    settings.validate()?;
    let connection = database.lock()?;
    db::settings::save(&connection, &settings)?;
    drop(connection);

    if let Some(window) = app.get_webview_window("main") {
        if let Err(error) = window.set_always_on_top(settings.always_on_top) {
            warnings.push(format!("Always-on-top could not be changed: {error}"));
        }
    }
    if let Some(tray) = app.tray_by_id("main-tray") {
        if let Err(error) = tray.set_visible(settings.show_tray_icon) {
            warnings.push(format!("Tray visibility could not be changed: {error}"));
        }
    }
    let autostart_result = if settings.launch_on_startup {
        app.autolaunch().enable()
    } else {
        app.autolaunch().disable()
    };
    if let Err(error) = autostart_result {
        warnings.push(format!("Launch on startup could not be changed: {error}"));
    }
    Ok(settings)
}
