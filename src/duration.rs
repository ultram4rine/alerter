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

    let f_weeks = format_unit("week".to_owned(), "weeks".to_owned(), weeks);
    let f_days = format_unit("day".to_owned(), "days".to_owned(), days);
    let f_hours = format_unit("hour".to_owned(), "hours".to_owned(), hours);
    let f_minutes = format_unit("minute".to_owned(), "minutes".to_owned(), minutes);
    let f_seconds = format_unit("second".to_owned(), "seconds".to_owned(), seconds);
    let f_milliseconds = if seconds == 0 {
        format_unit(
            "millisecond".to_owned(),
            "milliseconds".to_owned(),
            milliseconds,
        )
    } else {
        "".to_owned()
    };

    format!(
        "{}{}{}{}{}{}",
        f_weeks, f_days, f_hours, f_minutes, f_seconds, f_milliseconds
    )
    .trim()
    .to_owned()
}

fn format_unit(unit: String, units: String, count: i64) -> String {
    match count {
        0 => "".to_owned(),
        1 => format!(" 1 {}", unit),
        _ => format!(" {} {}", count, units),
    }
}
