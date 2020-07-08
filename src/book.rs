use chrono::{NaiveDate, Weekday};
use structopt::StructOpt;
use time::Time;

#[derive(Debug, PartialEq, Eq)]
enum TimeLog {
    Minutes(u8),
    Hours(u8),
    Stretch(Time, Time),
}

#[derive(Debug, PartialEq, Eq)]
enum DateLog {
    Yesterday,
    Weekday(Weekday),
    Date(NaiveDate),
}

#[derive(StructOpt, Debug)]
pub struct BookingArgs {
    /// Project alias
    alias: String,
    /// Time in minutes or a stretch pattern (e.g. <int> | h:<int> | ::HH:MM | ::last)
    #[structopt(parse(try_from_str = parse_time))]
    time: TimeLog,
    /// Date in isoformat or weekday (e.g. "YYYY-MM-DD" | <weekday>)
    #[structopt(short="d", long="date", parse(try_from_str = parse_date))]
    date: Option<DateLog>,
    /// Description of time expenditure (must pass spelling check)
    message: Option<String>,
    /// Reference to work ticket (e.g. "RAS-002")
    ticket: Option<String>,
    /// Reference to git branch for work (e.g. "feature/RAS-002")
    branch: Option<String>,
}

fn parse_time(input: &str) -> Result<TimeLog, String> {
    match input.to_owned().parse::<u8>() {
        Ok(minutes) => Ok(TimeLog::Minutes(minutes)),
        Err(_) => match (input.get(..2), input.get(2..)) {
            // TODO: Implement reading latest log time
            (Some("::"), Some("last")) => Ok(TimeLog::Stretch(Time::now(), Time::now())),
            (Some("::"), Some(time)) => match Time::parse(format!("{}:00", time), "%T") {
                Ok(t) => Ok(TimeLog::Stretch(t, Time::now())),
                Err(ctx) => Err(format!(
                    "{} is an invalid stretch pattern. The correct format is HH:MM",
                    ctx
                )),
            },
            (Some("::"), None) => {
                Err("Stretch pattern requires an argument (last or HH:MM)".into())
            }
            (Some("h:"), Some(hours)) => match hours.parse::<u8>() {
                Ok(h) => Ok(TimeLog::Hours(h)),
                Err(ctx) => Err(format!(
                    "{} for hours. The correct format is 'h:<int>'",
                    ctx
                )),
            },
            (Some("h:"), None) => Err("Hour pattern requires an argument (<int>)".into()),
            (Some(head), Some(tail)) => {
                Err(format!("Unable to parse time value: {}{}", head, tail))
            }
            (_, _) => Err("No value was given".into()),
        },
    }
}

fn parse_date(input: &str) -> Result<DateLog, String> {
    let input = input.to_lowercase();
    let days: Vec<&str> = vec!["mon", "tue", "wed", "thu", "fri", "sat", "sun"];
    match input.as_ref() {
        "yesterday" => Ok(DateLog::Yesterday),
        x if days.contains(&x.get(..3).unwrap_or("")) => {
            Ok(DateLog::Weekday(x.parse::<Weekday>().unwrap()))
        }
        maybe_date => match NaiveDate::parse_from_str(maybe_date, "%Y-%m-%d") {
            Ok(date) => Ok(DateLog::Date(date)),
            Err(ctx) => Err(format!("{} is an invalid pattern", ctx)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn time_parser_ok() {
        assert!(
            parse_time("::last").is_ok(),
            "Fail on parsing last time stretch"
        );
        assert!(
            parse_time("::08:00").is_ok(),
            "Fail on parsing defined time stretch"
        );
        assert_eq!(
            parse_time("h:8"),
            Ok(TimeLog::Hours(8)),
            "Fail on parsing hours"
        );
        assert_eq!(
            parse_time("60"),
            Ok(TimeLog::Minutes(60)),
            "Fail on parsing minutes"
        );
    }
    #[test]
    fn time_parser_err() {
        assert!(parse_time("::lasts").is_err());
        assert!(parse_time("::08").is_err());
        assert!(parse_time("::08:00a").is_err());
        assert!(parse_time("").is_err());
        assert!(parse_time("abc").is_err());
    }

    #[test]
    fn date_parser_ok() {
        assert!(parse_date("yesterday").is_ok(), "Fail on parsing yesterday");
        assert!(
            parse_date("Yesterday").is_ok(),
            "Fail on not parsing capitalized yesterday"
        );
        assert!(
            parse_date("2020-04-20").is_ok(),
            "Fail on parsing correct date"
        );
        assert!(
            parse_date("mon").is_ok(),
            "Fail on parsing valid partial weekday"
        );
        assert!(
            parse_date("monday").is_ok(),
            "Fail on parsing valid weekday"
        );
        assert!(
            parse_date("Monday").is_ok(),
            "Fail on handling weekday capitalization"
        );
    }

    #[test]
    fn date_parser_err() {
        assert!(
            parse_date("yesterdy").is_err(),
            "Fail on misspelled yesterday"
        );
        assert!(parse_date("2020-04-200").is_err(), "Fail on invalid date");
        assert!(parse_date("man").is_err(), "Fail on invalid weekday");
    }

    proptest! {
        #[test]
        fn can_parse_valid_date_pattern(y in 1i32..10000, m in 1u32..13, d in 1u32..28) {
            let s = NaiveDate::from_ymd(y, m, d).to_string();
            assert!(parse_date(&s).is_ok(), "Fail at {}", s);
        }
    }
}
