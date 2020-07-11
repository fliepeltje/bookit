use crate::errors::CliError;
use crate::generics::Result;
use chrono::NaiveTime;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_ok() {
        assert_eq!(slugify("Upper spaced".into()), String::from("upperspaced"))
    }
}
