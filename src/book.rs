use crate::errors::CliError;
use crate::generics::{add_subject, Result, View};
use crate::hours::HourLog;
use crate::utils::{parse_date as util_date, parse_time};
use chrono::offset::Local as LocalTime;
use chrono::{NaiveDate, NaiveTime};
use harsh::Harsh;
use std::convert::TryFrom;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
enum Directive {
    Minutes(u8),
    Hours(f32),
    Since(NaiveTime),
    Until(NaiveTime),
}

impl FromStr for Directive {
    type Err = CliError;
    fn from_str(input: &str) -> Result<Self> {
        let directive = input.get(..3);
        let arg = input.get(3..);
        if directive.is_none() || arg.is_none() {
            Err(CliError::Directive {
                input: input.into(),
                context: "directive incomplete (use <directive>::<argument>)".into(),
            })
        } else if input.contains("::") {
            let directive = directive.unwrap();
            let arg = arg.unwrap();
            match directive {
                "h::" => match arg.parse::<f32>() {
                    Ok(h) => Ok(Self::Hours(h)),
                    Err(_) => Err(CliError::Parse {
                        input: input.into(),
                        description: format!(
                            "value should be an integer or a floating point number"
                        ),
                    }),
                },
                "s::" => Ok(Directive::Since(parse_time(input)?)),
                "t::" => Ok(Directive::Until(parse_time(input)?)),
                _ => Err(CliError::Directive {
                    input: input.into(),
                    context: "Unknown directive".into(),
                }),
            }
        } else {
            match input.parse::<u8>() {
                Ok(minutes) => Ok(Directive::Minutes(minutes)),
                Err(_) => Err(CliError::Directive {
                    input: input.into(),
                    context: "not a valid integer (e.g. 60)".into(),
                }),
            }
        }
    }
}

impl From<Directive> for u8 {
    fn from(directive: Directive) -> Self {
        match directive {
            Directive::Minutes(m) => m,
            Directive::Hours(h) => (60.0 * h) as u8,
            Directive::Since(t) => (LocalTime::now().naive_local().time() - t).num_minutes() as u8,
            Directive::Until(t) => (t - LocalTime::now().naive_local().time()).num_minutes() as u8,
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct BookingArgs {
    /// Project alias
    alias: String,
    /// Time in minutes or a stretch pattern (e.g. <int> | h::<f64> | <s or t>::HH:MM | s::last)
    time: Directive,
    /// Date in isoformat or weekday (e.g. "YYYY-MM-DD" | <weekday>)
    #[structopt(short = "d", long = "date", default_value = "today", parse(try_from_str = util_date))]
    date: NaiveDate,
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

impl TryFrom<BookingArgs> for HourLog {
    type Error = CliError;

    fn try_from(args: BookingArgs) -> Result<Self> {
        let now = LocalTime::now().naive_local();
        let encoder = Harsh::builder()
            .salt("bookit")
            .build()
            .expect("could not create encoder");
        let timestamp = now.timestamp();
        Ok(Self {
            alias: args.alias,
            minutes: args.time.into(),
            date: args.date,
            message: args.message,
            ticket: args.ticket,
            branch: args.branch,
            id: encoder.encode(&[timestamp as u64]).to_lowercase(),
            timestamp: LocalTime::now().naive_local(),
        })
    }
}

pub fn exec_cmd_book(args: BookingArgs) -> Result<()> {
    let hours = HourLog::try_from(args)?;
    add_subject(hours.clone())?;
    println!("{}", hours.format_list_item());
    Ok(())
}
