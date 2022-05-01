use chrono::Duration;

/// Converts `chrono::Duration` to human-readable format.
///
/// # Arguments
///
/// * `duration` - A `Duration` to format.
///
/// # Examples
///
/// ```rust
/// // Format duration of two weeks:
/// let time1 = DateTime::parse_from_rfc3339("1990-12-15T22:00:00Z").unwrap();
/// let time2 = DateTime::parse_from_rfc3339("1990-12-01T22:00:00Z").unwrap();
/// assert_eq!(
///     format_duration(time1.signed_duration_since(time2)),
///     "2 weeks"
/// );
/// ```
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

/// Formats unit.
///
/// # Arguments
///
/// * `unit` - A singular pronunciation of unit.
/// * `units` - A plural pronunciation of unit.
/// * `count` - A count of units.
fn format_unit(unit: String, units: String, count: i64) -> String {
    match count {
        0 => "".to_owned(),
        1 => format!(" 1 {}", unit),
        _ => format!(" {} {}", count, units),
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use crate::duration::format_duration;

    #[test]
    fn test_format_duration_weeks() {
        let time1 = DateTime::parse_from_rfc3339("1990-12-15T22:00:00Z").unwrap();
        let time2 = DateTime::parse_from_rfc3339("1990-12-01T22:00:00Z").unwrap();
        assert_eq!(
            format_duration(time1.signed_duration_since(time2)),
            "2 weeks"
        );
    }

    #[test]
    fn test_format_duration_days() {
        let time1 = DateTime::parse_from_rfc3339("1990-12-03T22:00:00Z").unwrap();
        let time2 = DateTime::parse_from_rfc3339("1990-12-01T22:00:00Z").unwrap();
        assert_eq!(
            format_duration(time1.signed_duration_since(time2)),
            "2 days"
        );
    }

    #[test]
    fn test_format_duration_hour() {
        let time1 = DateTime::parse_from_rfc3339("1990-12-01T23:00:00Z").unwrap();
        let time2 = DateTime::parse_from_rfc3339("1990-12-01T22:00:00Z").unwrap();
        assert_eq!(
            format_duration(time1.signed_duration_since(time2)),
            "1 hour"
        );
    }

    #[test]
    fn test_format_duration_minutes() {
        let time1 = DateTime::parse_from_rfc3339("1990-12-01T22:30:00Z").unwrap();
        let time2 = DateTime::parse_from_rfc3339("1990-12-01T22:00:00Z").unwrap();
        assert_eq!(
            format_duration(time1.signed_duration_since(time2)),
            "30 minutes"
        );
    }

    #[test]
    fn test_format_duration_seconds() {
        let time1 = DateTime::parse_from_rfc3339("1990-12-01T22:00:15Z").unwrap();
        let time2 = DateTime::parse_from_rfc3339("1990-12-01T22:00:00Z").unwrap();
        assert_eq!(
            format_duration(time1.signed_duration_since(time2)),
            "15 seconds"
        );
    }

    #[test]
    fn test_format_duration_milliseconds() {
        let time1 = DateTime::parse_from_rfc3339("1990-12-01T22:00:00.500Z").unwrap();
        let time2 = DateTime::parse_from_rfc3339("1990-12-01T22:00:00.000Z").unwrap();
        assert_eq!(
            format_duration(time1.signed_duration_since(time2)),
            "500 milliseconds"
        );
    }

    #[test]
    fn test_format_duration_full() {
        // Milliseconds ignored if seconds not 0.
        let time1 = DateTime::parse_from_rfc3339("1990-12-17T23:30:15.500Z").unwrap();
        let time2 = DateTime::parse_from_rfc3339("1990-12-01T22:00:00.000Z").unwrap();
        assert_eq!(
            format_duration(time1.signed_duration_since(time2)),
            "2 weeks 2 days 1 hour 30 minutes 15 seconds"
        );
    }
}
