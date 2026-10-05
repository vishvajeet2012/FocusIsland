use tauri_plugin_notification::NotificationExt;

use crate::constants::PRODUCT_NAME;

pub fn focus_complete(
    app: &tauri::AppHandle,
    task_title: Option<&str>,
    minutes: i64,
    sound: bool,
) -> Result<(), String> {
    let body = match task_title {
        Some(title) => format!("“{title}” · {minutes} minutes completed."),
        None => format!("{minutes} minutes completed."),
    };
    let mut notification = app
        .notification()
        .builder()
        .title("Focus session complete")
        .body(body);
    if sound {
        notification = notification.sound("Default");
    }
    notification
        .show()
        .map_err(|error| format!("Could not show focus notification: {error}"))
}

pub fn reminder(app: &tauri::AppHandle, title: &str, sound: bool) -> Result<(), String> {
    let mut notification = app
        .notification()
        .builder()
        .title(format!("{PRODUCT_NAME} reminder"))
        .body(title);
    if sound {
        notification = notification.sound("Default");
    }
    notification
        .show()
        .map_err(|error| format!("Could not show reminder notification: {error}"))
}
