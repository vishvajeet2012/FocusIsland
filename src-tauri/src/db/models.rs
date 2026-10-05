use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub notes: Option<String>,
    pub completed: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub due_at: Option<i64>,
    pub reminder_at: Option<i64>,
    pub estimated_focus_minutes: Option<i64>,
    pub sort_order: i64,
    pub archived_at: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskInput {
    pub title: String,
    pub notes: Option<String>,
    pub due_at: Option<i64>,
    pub estimated_focus_minutes: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdate {
    pub id: i64,
    pub title: String,
    pub notes: Option<String>,
    pub due_at: Option<i64>,
    pub estimated_focus_minutes: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyNote {
    pub id: i64,
    pub date: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reminder {
    pub id: i64,
    pub task_id: Option<i64>,
    pub title: String,
    pub remind_at: i64,
    pub completed: bool,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderInput {
    pub task_id: Option<i64>,
    pub title: String,
    pub remind_at: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerSnapshot {
    pub mode: String,
    pub status: String,
    pub started_at: Option<i64>,
    pub paused_at: Option<i64>,
    pub accumulated_pause_ms: i64,
    pub target_duration_seconds: Option<i64>,
    pub task_id: Option<i64>,
    pub task_title: Option<String>,
    pub session_started_at: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerStartInput {
    pub mode: String,
    pub duration_seconds: Option<i64>,
    pub task_id: Option<i64>,
    pub task_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub theme: String,
    pub workspace_background: String,
    pub task_card_color: String,
    pub timer_card_color: String,
    pub notes_card_color: String,
    pub events_card_color: String,
    pub island_size: String,
    pub expand_on_click: bool,
    pub expand_on_hover: bool,
    pub hover_delay_ms: i64,
    pub always_on_top: bool,
    pub launch_on_startup: bool,
    pub show_tray_icon: bool,
    pub animation_intensity: String,
    pub notifications_enabled: bool,
    pub notification_sounds: bool,
    pub default_timer_minutes: i64,
    pub time_format: String,
    pub collapse_on_focus_loss: bool,
    pub movable: bool,
    pub monitor_mode: String,
    pub top_gap: i64,
    pub first_run: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".into(),
            workspace_background: "#08090a".into(),
            task_card_color: "#c9dcc5".into(),
            timer_card_color: "#d8d0ec".into(),
            notes_card_color: "#eadcae".into(),
            events_card_color: "#c8ddeb".into(),
            island_size: "normal".into(),
            expand_on_click: true,
            expand_on_hover: false,
            hover_delay_ms: 300,
            always_on_top: true,
            launch_on_startup: false,
            show_tray_icon: true,
            animation_intensity: "full".into(),
            notifications_enabled: true,
            notification_sounds: false,
            default_timer_minutes: 25,
            time_format: "12".into(),
            collapse_on_focus_loss: true,
            movable: false,
            monitor_mode: "active".into(),
            top_gap: 6,
            first_run: true,
        }
    }
}

impl AppSettings {
    pub fn validate(&self) -> Result<(), String> {
        validate_choice("theme", &self.theme, &["system", "dark", "light"])?;
        validate_choice(
            "island size",
            &self.island_size,
            &["compact", "normal", "large"],
        )?;
        validate_choice(
            "animation intensity",
            &self.animation_intensity,
            &["full", "reduced", "off"],
        )?;
        validate_choice("time format", &self.time_format, &["12", "24"])?;
        validate_choice("monitor mode", &self.monitor_mode, &["active", "primary"])?;
        for color in [
            &self.workspace_background,
            &self.task_card_color,
            &self.timer_card_color,
            &self.notes_card_color,
            &self.events_card_color,
        ] {
            if !is_hex_color(color) {
                return Err(format!("Invalid color value: {color}"));
            }
        }
        if ![100, 200, 300, 500].contains(&self.hover_delay_ms) {
            return Err("Hover delay must be 100, 200, 300, or 500 milliseconds".into());
        }
        if !(1..=180).contains(&self.default_timer_minutes) {
            return Err("Default timer must be between 1 and 180 minutes".into());
        }
        if ![0, 6, 12].contains(&self.top_gap) {
            return Err("Top gap must be 0, 6, or 12 pixels".into());
        }
        Ok(())
    }
}

fn validate_choice(label: &str, value: &str, choices: &[&str]) -> Result<(), String> {
    if choices.contains(&value) {
        Ok(())
    } else {
        Err(format!("Invalid {label}"))
    }
}

fn is_hex_color(value: &str) -> bool {
    value.len() == 7
        && value.starts_with('#')
        && value.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DayMetric {
    pub date: String,
    pub label: String,
    pub focus_minutes: i64,
    pub completed_tasks: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Insights {
    pub today_focus_seconds: i64,
    pub tasks_completed_today: i64,
    pub sessions_today: i64,
    pub current_streak: i64,
    pub most_productive_day: Option<String>,
    pub average_session_seconds: i64,
    pub weekly_completed_tasks: i64,
    pub weekly_focus_minutes: i64,
    pub days: Vec<DayMetric>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapPayload {
    pub tasks: Vec<Task>,
    pub note: DailyNote,
    pub reminders: Vec<Reminder>,
    pub timer: TimerSnapshot,
    pub settings: AppSettings,
    pub shortcut_warnings: Vec<String>,
}
