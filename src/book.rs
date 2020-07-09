use chrono::{NaiveDate, Weekday};
use std::str::FromStr;
use structopt::StructOpt;
use time::Time;

#[derive(Debug, PartialEq)]
enum TimeArg {
    Minutes(u8),
    Hours(f32),
    Stretch(Time, Time),
}

type TimeArgError = String;

impl FromStr for TimeArg {
    type Err = TimeArgError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("::") {
            match (s.get(..1), s.get(3..)) {
                (Some("h"), Some(hours)) => match hours.parse::<f32>() {
                    Ok(h) => Ok(Self::Hours(h)),
                    Err(ctx) => Err(format!("{}", ctx)),
                },
                (Some(s), Some(stretch)) if s == "s" || s == "t" => {
                    if stretch == "last" {
                        Ok(Self::Stretch(Time::now(), Time::now()))
                    } else {
                        let time = Time::parse(format!("{}:00", stretch), "%T")
                            .expect("incorrect time argument (HH:MM or last)");
                        match s {
                            "s" => Ok(Self::Stretch(time, Time::now())),
                            "t" => Ok(Self::Stretch(Time::now(), time)),
                            _ => Err("unknown stretch directive".into()),
                        }
                    }
                }
                (Some(":"), _) => Err("missing directive".into()),
                (Some(s), _) => Err(format!("unknown directive {}", s)),
                (None, _) => Err("missing time argument".into()),
            }
        } else {
            match s.parse::<u8>() {
                Ok(minutes) => Ok(Self::Minutes(minutes)),
                Err(ctx) => Err(format!("{}", ctx)),
            }
        }
    }
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
    /// Time in minutes or a stretch pattern (e.g. <int> | h::<f64> | <s or t>::HH:MM | s::last)
    time: TimeArg,
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
        assert!(TimeArg::from_str("s::last").is_ok());
        assert!(TimeArg::from_str("s::08:00").is_ok());
        assert!(TimeArg::from_str("h::1.5").is_ok());
        assert!(TimeArg::from_str("60").is_ok());
    }
    #[test]
    fn time_parser_err() {
        assert!(TimeArg::from_str("::lasts").is_err());
        assert!(TimeArg::from_str("::08").is_err());
        assert!(TimeArg::from_str("::08:00a").is_err());
        assert!(TimeArg::from_str("").is_err());
        assert!(TimeArg::from_str("abc").is_err());
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
