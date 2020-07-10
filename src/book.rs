use crate::generics::{add_subject, Result, View};
use crate::hours::HourLog;
use chrono::offset::Local as LocalTime;
use chrono::{Datelike, NaiveDate, NaiveTime, Timelike, Weekday};
use harsh::Harsh;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, PartialEq)]
enum TimeArg {
    Minutes(u8),
    Hours(f32),
    Stretch(NaiveTime, NaiveTime),
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
                    let now = LocalTime::now().naive_local().time();
                    if stretch == "last" {
                        Ok(Self::Stretch(now, now))
                    } else {
                        let fmt_time = format!("{}:00", stretch);
                        let time = NaiveTime::from_str(&fmt_time).expect("");
                        match s {
                            "s" => Ok(Self::Stretch(time, now)),
                            "t" => Ok(Self::Stretch(now, time)),
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

impl From<TimeArg> for u8 {
    fn from(time: TimeArg) -> Self {
        match time {
            TimeArg::Minutes(m) => m,
            TimeArg::Hours(h) => (60.0 * h) as u8,
            TimeArg::Stretch(f, l) => {
                let l_min = (l.hour() * 60) + l.minute();
                let f_min = (f.hour() * 60) + f.minute();
                let duration = l_min - f_min;
                duration as u8
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

impl From<DateArg> for NaiveDate {
    fn from(arg: DateArg) -> Self {
        let now = LocalTime::now().naive_utc();
        let today = now.date();
        match arg {
            DateArg::Today => today,
            DateArg::Yesterday => today.pred(),
            DateArg::Date(date) => date,
            DateArg::Weekday(day) => {
                let current_day = today.weekday();
                let current_week = today.iso_week().week();
                if current_day.num_days_from_monday() > day.num_days_from_monday() {
                    NaiveDate::from_isoywd(today.year(), current_week, day)
                } else {
                    NaiveDate::from_isoywd(today.year(), current_week - 1, day)
                }
            }
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

impl From<BookingArgs> for HourLog {
    fn from(args: BookingArgs) -> Self {
        let now = LocalTime::now().naive_local();
        let encoder = Harsh::builder()
            .salt("bookit")
            .build()
            .expect("could not create encoder");
        let timestamp = now.timestamp();
        Self {
            alias: args.alias,
            minutes: args.time.into(),
            date: args.date.into(),
            message: args.message,
            ticket: args.ticket,
            branch: args.branch,
            id: encoder.encode(&[timestamp as u64]).to_lowercase(),
            timestamp: LocalTime::now().naive_utc(),
        }
    }
}

pub fn exec_cmd_book(args: BookingArgs) -> Result<()> {
    let hours: HourLog = args.into();
    add_subject(hours.clone());
    println!("{}", hours.format_list_item());
    Ok(())
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
