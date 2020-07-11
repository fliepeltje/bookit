use crate::errors::CliError;
use crate::generics::Result;
use chrono::{Local as LocalTime, NaiveDate, NaiveTime, Weekday, Datelike};
use std::str::FromStr;

pub fn slugify(s: String) -> String {
    s.to_lowercase().split_whitespace().collect()
}

pub fn parse_time(time_str: &str) -> Result<NaiveTime> {
    let fmt_time = format!("{}:00", time_str);
    match NaiveTime::from_str(&fmt_time) {
        Ok(t) => Ok(t),
        Err(_) => Err(CliError::Parse {
            input: time_str.into(),
            description: "value should be formatted to HH:MM".into(),
        }),
    }
}

pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    let input = date_str.to_lowercase();
    if !input.contains("-") {
        let today = LocalTime::now().naive_local().date();
        match input.as_ref() {
            "today" => Ok(today),
            "yesterday" => Ok(today.pred()),
            maybe_day => match maybe_day.parse::<Weekday>(){
                Ok(day) => if today.weekday().num_days_from_monday() > day.num_days_from_monday() {
                    Ok(NaiveDate::from_isoywd(today.year(), today.iso_week().week(), day))
                } else {
                    Ok(NaiveDate::from_isoywd(today.year(),  today.iso_week().week() - 1, day))
                },
                Err(_) => Err(CliError::Parse{
                    input: date_str.into(),
                    description: "should be a relative definition of date ( today | yesterday | <day of week> (e.g. 'mon' or 'monday')".into()
                })
            } 
        }
    } else {
        match NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
            Ok(date) => Ok(date),
            Err(_) => Err(CliError::Parse {
                input: input,
                description: "should be in YYYY-MM-DD format".into(),
            }),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::*;

    #[test]
    fn slugify_ok() {
        assert_eq!(slugify("Upper spaced".into()), String::from("upperspaced"))
    }

    proptest! {
        #[test]
        fn can_parse_valid_date_pattern(y in 1i32..10000, m in 1u32..13, d in 1u32..28) {
            let s = NaiveDate::from_ymd(y, m, d).to_string();
            assert!(parse_date(&s).is_ok(), "Fail at {}", s);
        }

        #[test]
        fn can_parse_valid_time_pattern(h in 0..23, m in 0..59) {
            let h = if h < 10 {
                format!("0{}", h)
            } else {
                format!("{}", h)
            };
            let m = if m < 10 {
                format!("0{}", m)
            } else {
                format!("{}", m)
            };
            let time = format!("{}:{}", h, m);
            assert!(parse_time(&time).is_ok())
        }
    }

}
