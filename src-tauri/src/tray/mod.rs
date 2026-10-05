use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Emitter,
};

use crate::constants::PRODUCT_NAME;

pub fn create(app: &tauri::AppHandle) -> Result<(), String> {
    let open = MenuItem::with_id(
        app,
        "open",
        format!("Open {PRODUCT_NAME}"),
        true,
        None::<&str>,
    )
    .map_err(menu_error)?;
    let start = MenuItem::with_id(
        app,
        "start-focus",
        "Start 25 minute focus",
        true,
        None::<&str>,
    )
    .map_err(menu_error)?;
    let toggle = MenuItem::with_id(
        app,
        "toggle-timer",
        "Pause / Resume timer",
        true,
        None::<&str>,
    )
    .map_err(menu_error)?;
    let new_task =
        MenuItem::with_id(app, "new-task", "New task", true, None::<&str>).map_err(menu_error)?;
    let progress = MenuItem::with_id(app, "progress", "Today's progress", true, None::<&str>)
        .map_err(menu_error)?;
    let settings =
        MenuItem::with_id(app, "settings", "Settings", true, None::<&str>).map_err(menu_error)?;
    let separator = PredefinedMenuItem::separator(app).map_err(menu_error)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).map_err(menu_error)?;
    let menu = Menu::with_items(
        app,
        &[
            &open, &start, &toggle, &new_task, &progress, &settings, &separator, &quit,
        ],
    )
    .map_err(menu_error)?;

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .tooltip(PRODUCT_NAME)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            let id = event.id.as_ref();
            if id == "quit" {
                app.exit(0);
                return;
            }
            crate::windows::show_without_error(app);
            let _ = app.emit("tray-action", id);
        })
        .on_tray_icon_event(|tray, event| {
            if matches!(
                event,
                TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                }
            ) {
                let app = tray.app_handle();
                crate::windows::show_without_error(app);
                let _ = app.emit("shortcut-action", "toggle");
            }
        });
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder
        .build(app)
        .map_err(|error| format!("Could not create tray icon: {error}"))?;
    Ok(())
}

fn menu_error(error: tauri::Error) -> String {
    format!("Could not create tray menu: {error}")
}
