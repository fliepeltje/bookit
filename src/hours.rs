use crate::alias::Alias;
use crate::errors::CliError;
use crate::generics::{
    add_subject, delete_subject, view_filtered_set, view_subject, Crud, Filter, Result, View,
};
use crate::utils::parse_date;
use crate::utils::parse_time;
use chrono::{Local, NaiveDate, NaiveDateTime};
use colored::*;
use harsh::Harsh;
use serde::{Deserialize, Serialize};
use serde_json::de::from_str as from_json;
use serde_json::ser::to_string as to_json;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
enum CmdError {
    NoHours,
    NoTime,
    Hasher,
    InvalidHours(String),
    InvalidMinutes(String),
    InvalidTime(String),
}

impl From<CmdError> for CliError {
    fn from(err: CmdError) -> CliError {
        match err {
            CmdError::Hasher => {
                CliError::BinaryError("Unable to initialize hash function".to_string())
            }
            CmdError::NoTime => CliError::CmdError(
                "No time specified after directive (use '<s | t>::08:00'".to_string(),
            ),
            CmdError::NoHours => CliError::CmdError("No hours specified (use 'h:1.5')".to_string()),
            CmdError::InvalidHours(h) => CliError::CmdError(format!(
                "could not parse hours {} (use a float or integer)",
                h.yellow().bold()
            )),
            CmdError::InvalidMinutes(m) => CliError::CmdError(format!(
                "could not parse minutes {} (use an integer)",
                m.yellow().bold()
            )),
            CmdError::InvalidTime(t) => {
                CliError::CmdError(format!("unable to interpret time: {}", t.yellow().bold()))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HourLog {
    pub alias: String,
    pub minutes: u32,
    pub date: NaiveDate,
    pub message: Option<String>,
    pub ticket: Option<String>,
    pub branch: Option<String>,
    pub id: String,
    pub timestamp: NaiveDateTime,
}

#[derive(StructOpt, Debug, Clone)]
pub struct CreateArgs {
    alias: Alias,
    /// Time in minutes or a stretch pattern (e.g. <int> | h::<f64> | <s or t>::HH:MM | s::last)
    #[structopt(name="time", parse(try_from_str = interpret_time))]
    time: u32,
    /// Date in isoformat or weekday (e.g. "YYYY-MM-DD" | <weekday>)
    #[structopt(short = "d", long = "date", default_value = "today", parse(try_from_str = parse_date))]
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

#[derive(StructOpt, Debug)]
pub enum Cmd {
    /// View a detailed hour booking
    #[structopt(name = "detail")]
    Detail { slug: String },
    /// View a collection of hours
    #[structopt(name = "show")]
    Show {
        #[structopt(short = "f")]
        filters: Vec<F>,
        #[structopt(short = "s", default_value = "no_sort")]
        sort: S,
    },
    /// Delete an hour booking by hash
    #[structopt(name = "delete")]
    Delete { slug: String },
    /// Add an hour booking
    #[structopt(name = "book")]
    Create(CreateArgs),
}

fn interpret_time(time_str: &str) -> Result<u32> {
    let res = match time_str {
        time_str if time_str.starts_with("h::") => {
            if let Some(maybe_h) = time_str.get(3..) {
                match maybe_h.parse::<f32>() {
                    Ok(h) => Ok((60.0 * h) as u32),
                    Err(_) => Err(CmdError::InvalidHours(maybe_h.to_owned())),
                }
            } else {
                Err(CmdError::NoHours)
            }
        }
        time_str if time_str.starts_with("s::") => {
            if let Some(maybe_t) = time_str.get(3..) {
                let t = parse_time(maybe_t)?;
                Ok((Local::now().naive_local().time() - t).num_minutes() as u32)
            } else {
                Err(CmdError::NoTime)
            }
        }
        time_str if time_str.starts_with("t::") => {
            if let Some(maybe_t) = time_str.get(3..) {
                let t = parse_time(maybe_t)?;
                Ok((t - Local::now().naive_local().time()).num_minutes() as u32)
            } else {
                Err(CmdError::NoTime)
            }
        }
        time_str if !time_str.contains("::") => {
            if let Ok(minutes) = time_str.parse::<u32>() {
                Ok(minutes)
            } else {
                Err(CmdError::InvalidMinutes(time_str.to_owned()))
            }
        }
        time_str => Err(CmdError::InvalidTime(time_str.to_owned())),
    };
    match res {
        Ok(res) => Ok(res),
        Err(err) => Err(err.into()),
    }
}

impl Cmd {
    pub fn exec(&self) -> Result<()> {
        match self {
            Self::Delete { slug } => delete_subject::<HourLog>(&slug)?,
            Self::Detail { slug } => view_subject::<HourLog>(Some(slug.to_owned()))?,
            Self::Show { filters, sort } => {
                let sort = sort.clone();
                view_filtered_set::<HourLog, F, S>(filters.to_vec(), sort)?
            }
            Self::Create(args) => add_subject::<HourLog>(HourLog::try_from(args.clone())?)?,
        };
        Ok(())
    }
}

impl TryFrom<CreateArgs> for HourLog {
    type Error = CliError;

    fn try_from(args: CreateArgs) -> Result<Self> {
        let now = Local::now().naive_local();
        let encoder = Harsh::builder()
            .salt("bookit")
            .build()
            .or(Err(CmdError::Hasher))?;
        let hash = encoder.encode(&[now.timestamp() as u64]).to_lowercase();
        let hours = Self {
            alias: args.alias.slug,
            minutes: args.time,
            date: args.date,
            message: args.message,
            ticket: args.ticket,
            branch: args.branch,
            id: hash,
            timestamp: now,
        };
        Ok(hours)
    }
}

impl View for HourLog {
    fn format_list_item(&self) -> String {
        let alias = format!("<{}>", &self.alias);
        let ticket = match &self.ticket {
            Some(t) => format!("[{}] ", t),
            None => "".into(),
        };
        let msg = match &self.message {
            Some(m) => m,
            None => "No message",
        };
        let description = format!("{}{}", ticket.bold(), msg);
        let minutes = format!("({} minutes)", &self.minutes);
        format!(
            "* {:7} - {} {} {}",
            &self.id.red().bold(),
            description,
            alias.purple().bold(),
            minutes.green()
        )
    }
}

impl Crud<'_> for HourLog {
    const FILE: &'static str = "hourstest.json";

    fn identifier(&self) -> String {
        self.id.clone()
    }

    fn deserialize(s: String) -> Result<HashMap<String, Self>> {
        Ok(from_json(&s)?)
    }

    fn serialize(map: HashMap<String, Self>) -> Result<String> {
        Ok(to_json(&map)?)
    }

    fn interactive_update(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Debug)]
pub enum F {
    NoFilter,
    ByAlias(String),
}

impl FromStr for F {
    type Err = CliError;

    fn from_str(input: &str) -> Result<Self> {
        match input {
            "nofilter" => Ok(Self::NoFilter),
            input if input.starts_with("alias::") => match input.get(7..) {
                Some(alias) => Ok(Self::ByAlias(alias.into())),
                None => Err(CliError::Directive {
                    input: input.into(),
                    context: "missing alias".into(),
                }),
            },
            input if input.contains("::") => Err(CliError::Directive {
                input: input.into(),
                context: "Cannot filter on given field".into(),
            }),
            _ => Err(CliError::Directive {
                input: input.into(),
                context: "Invalid filter query".into(),
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub enum S {
    NoSort,
    ByTimestamp,
}

impl FromStr for S {
    type Err = CliError;

    fn from_str(input: &str) -> Result<Self> {
        match input {
            "no_sort" => Ok(Self::NoSort),
            "ts" | "timestamp" => Ok(Self::ByTimestamp),
            _ => Err(CliError::InvalidSortQuery {
                input: input.into(),
            }),
        }
    }
}

impl Filter<F, S> for HourLog {
    const DEFAULT_SORT: S = S::NoSort;
    const DEFAULT_FILTER: F = F::NoFilter;

    fn get_base_items() -> Result<Vec<Self>> {
        let mapping = Self::mapping()?;
        Ok(mapping.values().cloned().collect::<Vec<Self>>())
    }

    fn filter(items: Vec<Self>, method: F) -> Vec<Self> {
        match method {
            F::NoFilter => items,
            F::ByAlias(alias) => items
                .into_iter()
                .filter(|item| item.alias == alias)
                .collect(),
        }
    }

    fn sort(items: Vec<Self>, method: S) -> Vec<Self> {
        let items = match method {
            S::ByTimestamp => {
                let mut items = items;
                items.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                items
            }
            S::NoSort => items,
        };
        items
    }
}
