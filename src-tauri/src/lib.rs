mod commands;
mod constants;
mod db;
mod notifications;
mod scheduler;
mod shortcuts;
mod tray;
mod windows;

use db::{models::AppSettings, Database};
use scheduler::SchedulerSignal;
use shortcuts::ShortcutWarnings;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ShortcutWarnings::default())
        .manage(SchedulerSignal::default())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(shortcuts::plugin())
        .setup(|app| {
            let warnings = app.state::<ShortcutWarnings>();
            let data_directory = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::env::temp_dir().join(constants::FALLBACK_DATA_DIRECTORY));
            let database_path = data_directory.join(constants::DATABASE_FILENAME);
            let database = match Database::open(&database_path) {
                Ok(database) => database,
                Err(error) => {
                    warnings.push(format!("Persistent storage is unavailable; using temporary memory storage. {error}"));
                    Database::in_memory().map_err(std::io::Error::other)?
                }
            };
            app.manage(database);

            let settings = app.state::<Database>().lock().and_then(|connection| db::settings::load(&connection)).unwrap_or_else(|error| {
                warnings.push(format!("Settings were reset: {error}"));
                AppSettings::default()
            });

            if let Err(error) = tray::create(app.handle()) { warnings.push(error); }
            if let Some(tray) = app.tray_by_id("main-tray") { let _ = tray.set_visible(settings.show_tray_icon); }
            shortcuts::register(app.handle(), &warnings);
            if let Err(error) = scheduler::start(app.handle().clone()) { warnings.push(error); }

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_always_on_top(settings.always_on_top);
            }
            if let Err(error) = windows::set_window_mode(
                app.handle().clone(),
                "collapsed".into(),
                settings.monitor_mode.clone(),
                settings.top_gap,
                settings.island_size.clone(),
            ) {
                warnings.push(error);
            }
            windows::show_without_error(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::bootstrap_app,
            commands::tasks::create_task,
            commands::tasks::update_task,
            commands::tasks::set_task_completed,
            commands::tasks::delete_task,
            commands::tasks::duplicate_task,
            commands::tasks::reorder_tasks,
            commands::tasks::archive_completed_tasks,
            commands::notes::save_daily_note,
            commands::reminders::create_reminder,
            commands::reminders::delete_reminder,
            commands::timer::start_timer,
            commands::timer::pause_timer,
            commands::timer::resume_timer,
            commands::timer::finish_timer,
            commands::timer::reset_timer,
            commands::insights::get_insights,
            commands::settings::save_settings,
            windows::set_window_mode,
            windows::show_main_window,
            windows::begin_window_drag,
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|error| panic!("{} runtime failed: {error}", constants::PRODUCT_NAME));
}
