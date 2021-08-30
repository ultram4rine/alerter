use chrono::Duration;

pub fn format_duration(duration: Duration) -> String {
    let weeks = duration.num_weeks();
    let days = duration.num_days() - weeks * 7;
    let hours = duration.num_hours() - (weeks * 7 + days) * 24;
    let minutes = duration.num_minutes() - (((weeks * 7 + days) * 24) + hours) * 60;
    let seconds =
        duration.num_seconds() - (((((weeks * 7 + days) * 24) + hours) * 60) + minutes) * 60;
    let milliseconds = duration.num_milliseconds()
        - ((((((weeks * 7 + days) * 24) + hours) * 60) + minutes) * 60 + seconds) * 1000;

    let f_weeks = format_unit("week".to_string(), "weeks".to_string(), weeks);
    let f_days = format_unit("day".to_string(), "days".to_string(), days);
    let f_hours = format_unit("hour".to_string(), "hours".to_string(), hours);
    let f_minutes = format_unit("minute".to_string(), "minutes".to_string(), minutes);
    let f_seconds = format_unit("second".to_string(), "seconds".to_string(), seconds);
    let f_milliseconds = if seconds == 0 {
        format_unit(
            "millisecond".to_string(),
            "milliseconds".to_string(),
            milliseconds,
        )
    } else {
        "".to_string()
    };

    format!(
        "{}{}{}{}{}{}",
        f_weeks, f_days, f_hours, f_minutes, f_seconds, f_milliseconds
    )
    .trim()
    .to_string()
}

fn format_unit(unit: String, units: String, count: i64) -> String {
    match count {
        0 => "".to_string(),
        1 => format!(" 1 {}", unit),
        _ => format!(" {} {}", count, units),
    }
}
