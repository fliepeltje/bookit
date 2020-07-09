use crate::generics::{delete_subject, view_subject, Crud, View};
use crate::Action;
use chrono::{NaiveDate, NaiveDateTime};
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::de::from_str as from_json;
use serde_json::error::Error as JsonError;
use serde_json::ser::to_string as to_json;
use std::collections::HashMap;
use structopt::StructOpt;

pub fn exec_cmd_hours(args: HourLogArgs) {
    match (args.action, args.slug) {
        (Action::View, slug) => view_subject::<HourLog>(slug),
        (Action::Delete, Some(slug)) => delete_subject::<HourLog>(&slug),
        (Action::Delete, None) => println!("delete requires a slug to be specified"),
        (action, _) => println!(
            "{} is not a valid action for this object",
            action.to_string().bold()
        ),
    }
}

#[derive(StructOpt, Debug)]
pub struct HourLogArgs {
    pub action: Action,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HourLog {
    pub alias: String,
    pub minutes: u8,
    pub date: NaiveDate,
    pub message: Option<String>,
    pub ticket: Option<String>,
    pub branch: Option<String>,
    pub id: String,
    pub timestamp: NaiveDateTime,
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
    type SerializeErr = JsonError;
    type DeserializeErr = JsonError;

    fn identifier(&self) -> String {
        self.id.clone()
    }

    fn deserialize(s: String) -> Result<HashMap<String, Self>, Self::DeserializeErr> {
        from_json(&s)
    }

    fn serialize(map: HashMap<String, Self>) -> Result<String, Self::SerializeErr> {
        to_json(&map)
    }

    fn interactive_update(&self) -> Self {
        self.clone()
    }
}
