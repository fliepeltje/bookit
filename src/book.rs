use chrono::{NaiveDate, Weekday};
use ispell::SpellLauncher;
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
type DateArgError = String;

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
enum DateArg {
    Today,
    Yesterday,
    Weekday(Weekday),
    Date(NaiveDate),
}

impl FromStr for DateArg {
    type Err = DateArgError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: String = s.to_lowercase();
        let is_date_notation = input.contains("-");
        match (input.as_ref(), is_date_notation) {
            ("today", false) => Ok(Self::Today),
            ("yesterday", false) => Ok(Self::Yesterday),
            (day, false) => match day.parse::<Weekday>() {
                Ok(day) => Ok(Self::Weekday(day)),
                Err(_) => Err("invalid weekday".into()),
            },
            (date, true) => match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
                Ok(date) => Ok(DateArg::Date(date)),
                Err(ctx) => Err(format!("{} is an invalid date", ctx)),
            },
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct BookingArgs {
    /// Project alias
    alias: String,
    /// Time in minutes or a stretch pattern (e.g. <int> | h::<f64> | <s or t>::HH:MM | s::last)
    time: TimeArg,
    /// Date in isoformat or weekday (e.g. "YYYY-MM-DD" | <weekday>)
    #[structopt(short = "d", long = "date", default_value = "today")]
    date: DateArg,
    /// Description of time expenditure (must pass spelling check)
    #[structopt(short = "m", long = "message")]
    message: Option<String>,
    /// Reference to work ticket (e.g. "RAS-002")
    #[structopt(short = "t", long = "ticket")]
    ticket: Option<String>,
    /// Reference to git branch for work (e.g. "feature/RAS-002")
    #[structopt(short = "b", long = "branch")]
    branch: Option<String>,
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
        assert!(DateArg::from_str("yesterday").is_ok());
        assert!(DateArg::from_str("Yesterday").is_ok());
        assert!(DateArg::from_str("2020-04-20").is_ok());
        assert!(DateArg::from_str("mon").is_ok());
        assert!(DateArg::from_str("monday").is_ok());
        assert!(DateArg::from_str("Monday").is_ok());
    }

    #[test]
    fn date_parser_err() {
        assert!(DateArg::from_str("yesterdy").is_err());
        assert!(DateArg::from_str("2020-04-200").is_err());
        assert!(DateArg::from_str("man").is_err());
    }

    proptest! {
        #[test]
        fn can_parse_valid_date_pattern(y in 1i32..10000, m in 1u32..13, d in 1u32..28) {
            let s = NaiveDate::from_ymd(y, m, d).to_string();
            assert!(DateArg::from_str(&s).is_ok(), "Fail at {}", s);
        }
    }
}
