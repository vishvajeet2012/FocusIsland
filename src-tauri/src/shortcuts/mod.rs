use std::sync::Mutex;
use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

fn shortcut(code: Code) -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL.union(Modifiers::ALT)), code)
}

#[derive(Default)]
pub struct ShortcutWarnings(Mutex<Vec<String>>);

impl ShortcutWarnings {
    pub fn push(&self, warning: String) {
        if let Ok(mut warnings) = self.0.lock() {
            if !warnings.contains(&warning) {
                warnings.push(warning);
            }
        }
    }

    pub fn list(&self) -> Vec<String> {
        self.0
            .lock()
            .map(|warnings| warnings.clone())
            .unwrap_or_else(|_| vec!["Shortcut status is unavailable".into()])
    }
}

pub fn plugin() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(|app, shortcut, event| {
            if event.state() != ShortcutState::Pressed {
                return;
            }
            let action = if shortcut == &self::shortcut(Code::Space) {
                Some("toggle")
            } else if shortcut == &self::shortcut(Code::KeyT) {
                Some("quick-task")
            } else if shortcut == &self::shortcut(Code::KeyP) {
                Some("toggle-timer")
            } else {
                None
            };
            if let Some(action) = action {
                crate::windows::show_without_error(app);
                let _ = app.emit("shortcut-action", action);
            }
        })
        .build()
}

pub fn register(app: &tauri::AppHandle, warnings: &ShortcutWarnings) {
    for (shortcut, label) in [
        (shortcut(Code::Space), "Ctrl + Alt + Space"),
        (shortcut(Code::KeyT), "Ctrl + Alt + T"),
        (shortcut(Code::KeyP), "Ctrl + Alt + P"),
    ] {
        if let Err(error) = app.global_shortcut().register(shortcut) {
            warnings.push(format!("{label} is unavailable: {error}"));
        }
    }
}
