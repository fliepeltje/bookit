use crate::errors::CliError;
use crate::generics::{
    delete_subject, view_filtered_set, view_subject, Crud, Filter, Result, View,
};
use crate::Action;
use chrono::{NaiveDate, NaiveDateTime};
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::de::from_str as from_json;
use serde_json::ser::to_string as to_json;
use std::collections::HashMap;
use std::str::FromStr;
use structopt::StructOpt;

pub fn exec_cmd_hours(args: HourLogArgs) -> Result<()> {
    match (args.action, args.slug) {
        (Action::View, slug) => view_subject::<HourLog>(slug)?,
        (Action::Delete, Some(slug)) => delete_subject::<HourLog>(&slug)?,
        (Action::Delete, None) => println!("delete requires a slug to be specified"),
        (action, _) => println!(
            "{} is not a valid action for this object",
            action.to_string().bold()
        ),
    };
    Ok(())
}

#[derive(StructOpt, Debug)]
pub struct HourLogArgs {
    pub action: Action,
    pub slug: Option<String>,
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
        };
        Ok(())
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
enum F {
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
enum S {
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
