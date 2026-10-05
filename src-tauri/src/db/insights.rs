use super::{
    models::{DayMetric, Insights},
    validate_date_key,
};
use chrono::{Days, Local, NaiveDate, TimeZone};
use rusqlite::{params, Connection};

pub fn get(connection: &Connection, date: &str) -> Result<Insights, String> {
    validate_date_key(date)?;
    let today =
        NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| "Invalid date".to_string())?;
    let week_start = today
        .checked_sub_days(Days::new(6))
        .ok_or("Date is out of range")?;
    let (today_start, today_end) = bounds(today)?;
    let (week_start_ms, _) = bounds(week_start)?;

    let today_focus_seconds: i64 = connection.query_row(
        "SELECT COALESCE(SUM(duration_seconds),0) FROM focus_sessions WHERE started_at>=?1 AND started_at<?2 AND completed=1",
        params![today_start, today_end], |row| row.get(0),
    ).map_err(db_error)?;
    let sessions_today: i64 = connection.query_row(
        "SELECT COUNT(*) FROM focus_sessions WHERE started_at>=?1 AND started_at<?2 AND completed=1",
        params![today_start, today_end], |row| row.get(0),
    ).map_err(db_error)?;
    let tasks_completed_today: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE completed_at>=?1 AND completed_at<?2",
            params![today_start, today_end],
            |row| row.get(0),
        )
        .map_err(db_error)?;
    let weekly_completed_tasks: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE completed_at>=?1 AND completed_at<?2",
            params![week_start_ms, today_end],
            |row| row.get(0),
        )
        .map_err(db_error)?;
    let (weekly_seconds, average_session_seconds): (i64, i64) = connection.query_row(
        "SELECT COALESCE(SUM(duration_seconds),0),COALESCE(ROUND(AVG(duration_seconds)),0) FROM focus_sessions WHERE started_at>=?1 AND started_at<?2 AND completed=1",
        params![week_start_ms, today_end], |row| Ok((row.get(0)?, row.get(1)?)),
    ).map_err(db_error)?;

    let mut days = Vec::with_capacity(7);
    for offset in 0..7 {
        let day = week_start
            .checked_add_days(Days::new(offset))
            .ok_or("Date is out of range")?;
        let (start, end) = bounds(day)?;
        let focus_seconds: i64 = connection.query_row(
            "SELECT COALESCE(SUM(duration_seconds),0) FROM focus_sessions WHERE started_at>=?1 AND started_at<?2 AND completed=1",
            params![start, end], |row| row.get(0),
        ).map_err(db_error)?;
        let completed_tasks: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM tasks WHERE completed_at>=?1 AND completed_at<?2",
                params![start, end],
                |row| row.get(0),
            )
            .map_err(db_error)?;
        days.push(DayMetric {
            date: day.format("%Y-%m-%d").to_string(),
            label: day.format("%a").to_string(),
            focus_minutes: (focus_seconds + 30) / 60,
            completed_tasks,
        });
    }

    let most_productive_day = days
        .iter()
        .filter(|day| day.focus_minutes > 0)
        .max_by_key(|day| day.focus_minutes)
        .map(|day| day.label.clone());
    let current_streak = streak(connection, today)?;
    Ok(Insights {
        today_focus_seconds,
        tasks_completed_today,
        sessions_today,
        current_streak,
        most_productive_day,
        average_session_seconds,
        weekly_completed_tasks,
        weekly_focus_minutes: (weekly_seconds + 30) / 60,
        days,
    })
}

fn streak(connection: &Connection, today: NaiveDate) -> Result<i64, String> {
    let mut cursor = today;
    let mut count = 0;
    for index in 0..366 {
        let (start, end) = bounds(cursor)?;
        let active: bool = connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM focus_sessions WHERE started_at>=?1 AND started_at<?2 AND completed=1)",
            params![start, end], |row| row.get(0),
        ).map_err(db_error)?;
        if active {
            count += 1;
        } else if index == 0 {
            cursor = cursor
                .checked_sub_days(Days::new(1))
                .ok_or("Date is out of range")?;
            continue;
        } else {
            break;
        }
        cursor = cursor
            .checked_sub_days(Days::new(1))
            .ok_or("Date is out of range")?;
    }
    Ok(count)
}

fn bounds(date: NaiveDate) -> Result<(i64, i64), String> {
    let start_naive = date.and_hms_opt(0, 0, 0).ok_or("Invalid date boundary")?;
    let next = date
        .checked_add_days(Days::new(1))
        .ok_or("Date is out of range")?
        .and_hms_opt(0, 0, 0)
        .ok_or("Invalid date boundary")?;
    let start = Local
        .from_local_datetime(&start_naive)
        .earliest()
        .ok_or("Cannot resolve local date")?
        .timestamp_millis();
    let end = Local
        .from_local_datetime(&next)
        .earliest()
        .ok_or("Cannot resolve local date")?
        .timestamp_millis();
    Ok((start, end))
}

fn db_error(error: rusqlite::Error) -> String {
    format!("Insights database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{now_ms, Database};
    use chrono::Datelike;

    #[test]
    fn completed_task_updates_today_insights() {
        let database = Database::in_memory().unwrap();
        let connection = database.lock().unwrap();
        let now = now_ms();
        connection.execute("INSERT INTO tasks(title,completed,completed_at,created_at,updated_at,sort_order) VALUES('Done',1,?1,?1,?1,0)", [now]).unwrap();
        let local = Local::now();
        let date = format!(
            "{:04}-{:02}-{:02}",
            local.year(),
            local.month(),
            local.day()
        );
        assert_eq!(get(&connection, &date).unwrap().tasks_completed_today, 1);
    }
}
