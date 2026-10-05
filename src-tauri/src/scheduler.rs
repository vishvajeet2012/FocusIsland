use crate::{
    db::{self, Database},
    notifications,
};
use std::{
    sync::{Condvar, Mutex},
    time::Duration,
};
use tauri::{Emitter, Manager};

#[derive(Default)]
pub struct SchedulerSignal {
    changed: Mutex<bool>,
    condition: Condvar,
}

impl SchedulerSignal {
    pub fn wake(&self) {
        if let Ok(mut changed) = self.changed.lock() {
            *changed = true;
            self.condition.notify_one();
        }
    }

    fn wait(&self, duration: Duration) {
        if let Ok(mut changed) = self.changed.lock() {
            if !*changed {
                match self.condition.wait_timeout(changed, duration) {
                    Ok((guard, _)) => changed = guard,
                    Err(_) => return,
                }
            }
            *changed = false;
        }
    }
}

pub fn start(app: tauri::AppHandle) -> Result<(), String> {
    std::thread::Builder::new()
        .name(crate::constants::DEADLINE_THREAD_NAME.into())
        .spawn(move || loop {
            let wait = process_due(&app).unwrap_or(Duration::from_secs(60));
            app.state::<SchedulerSignal>().wait(wait);
        })
        .map(|_| ())
        .map_err(|error| format!("Could not start reminder scheduler: {error}"))
}

fn process_due(app: &tauri::AppHandle) -> Result<Duration, String> {
    let now = db::now_ms();
    let database = app.state::<Database>();
    let mut connection = database.lock()?;
    let settings = db::settings::load(&connection)?;
    let completed_timer = db::timer::reconcile(&mut connection, now)?;
    let due_reminders = db::reminders::due(&connection, now)?;
    for reminder in &due_reminders {
        db::reminders::mark_fired(&connection, reminder)?;
    }
    let next_timer = db::timer::next_due_at(&connection)?;
    let next_reminder = db::reminders::next_due_at(&connection)?;
    drop(connection);

    if let Some(timer) = completed_timer {
        if settings.notifications_enabled {
            let _ = notifications::focus_complete(
                app,
                timer.task_title.as_deref(),
                timer.target_duration_seconds.unwrap_or(0) / 60,
                settings.notification_sounds,
            );
        }
        let _ = app.emit("timer-finished", timer);
    }
    for reminder in due_reminders {
        if settings.notifications_enabled {
            let _ = notifications::reminder(app, &reminder.title, settings.notification_sounds);
        }
        let _ = app.emit(
            "reminder-fired",
            serde_json::json!({ "id": reminder.id, "title": reminder.title }),
        );
    }

    let next = [next_timer, next_reminder].into_iter().flatten().min();
    let milliseconds = next
        .map(|deadline| deadline.saturating_sub(db::now_ms()).max(50) as u64)
        .unwrap_or(24 * 60 * 60 * 1_000);
    Ok(Duration::from_millis(milliseconds))
}
