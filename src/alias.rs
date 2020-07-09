use crate::generics::{add_subject, delete_subject, update_subject, view_subject, Crud, View};
use crate::utils::slugify;
use crate::Action;
use colored::*;
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structopt::StructOpt;
use toml::{from_str as from_toml, to_string as to_toml};

pub fn exec_cmd_alias(args: AliasArgs) {
    match (args.action, args.slug) {
        (Action::Add, None) => add_subject(Alias::new()),
        (Action::Delete, Some(slug)) => delete_subject::<Alias>(&slug),
        (Action::Update, Some(slug)) => update_subject::<Alias>(&slug),
        (Action::View, maybe_slug) => view_subject::<Alias>(maybe_slug),
        (Action::Add, Some(_)) => println!("The add action does not require a slug"),
        (action, None) => println!("{} requires a slug", action.to_string().bold()),
    }
}

#[derive(StructOpt, Debug)]
pub struct AliasArgs {
    /// Type of operation to perform
    action: Action,
    slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alias {
    pub slug: String,
    pub contractor: String,
    pub short_description: String,
    pub hourly_rate: u8,
}

impl Crud<'_> for Alias {
    const FILE: &'static str = "alias_test.toml";
    type DeserializeErr = toml::de::Error;
    type SerializeErr = toml::ser::Error;

    fn identifier(&self) -> String {
        self.slug.to_owned()
    }

    fn deserialize(tomlstr: String) -> Result<HashMap<String, Alias>, Self::DeserializeErr> {
        from_toml(&tomlstr)
    }

    fn serialize(map: HashMap<String, Alias>) -> Result<String, Self::SerializeErr> {
        to_toml(&map)
    }

    fn interactive_update(&self) -> Self {
        let slug = self.slug.to_owned();
        let contractor = input::<String>()
            .msg(format!("Contractor slug: [{}]", self.contractor))
            .default(self.contractor.clone())
            .get();
        let short_description = input::<String>()
            .msg(format!("Brief description: [{}]", self.short_description))
            .default(self.short_description.clone())
            .get();
        let hourly_rate = input::<u8>()
            .msg(format!("Hourly rate: [{}]", self.hourly_rate))
            .default(self.hourly_rate)
            .get();
        Self {
            slug,
            contractor,
            short_description,
            hourly_rate,
        }
    }
}

impl View for Alias {
    fn format_list_item(&self) -> String {
        format!(
            "{:7} {:7} {:30} {:5}",
            self.slug.red().bold(),
            self.contractor.cyan(),
            self.short_description,
            self.hourly_rate.to_string().green().bold()
        )
    }
}

impl Alias {
    fn new() -> Self {
        let slug = input::<String>()
            .msg("Alias: ")
            .add_test(|x| *x == slugify(x.into()))
            .get();
        let contractor = input::<String>().msg("Contractor slug: ").get();
        let short_description = input::<String>().msg("Brief description: ").get();
        let hourly_rate = input::<u8>().msg("Hourly rate: ").get();
        Self {
            slug,
            contractor,
            short_description,
            hourly_rate,
        }
    }
}
