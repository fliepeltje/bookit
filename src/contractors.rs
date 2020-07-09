use crate::generics::{add_subject, delete_subject, update_subject, view_subject, Crud, View};
use crate::utils::slugify;
use crate::Action;
use colored::*;
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structopt::StructOpt;
use toml::{from_str as from_toml, to_string as to_toml};

pub fn exec_cmd_contractor(args: ContractorArgs) {
    match (args.action, args.slug) {
        (Action::Add, None) => add_subject(Contractor::new()),
        (Action::Delete, Some(slug)) => delete_subject::<Contractor>(&slug),
        (Action::Update, Some(slug)) => update_subject::<Contractor>(&slug),
        (Action::View, maybe_slug) => view_subject::<Contractor>(maybe_slug),
        (Action::Add, Some(_)) => println!("The add action does not require a slug"),
        (action, None) => println!("{} requires a slug", action.to_string().bold()),
    }
}

#[derive(StructOpt, Debug)]
pub struct ContractorArgs {
    /// Type of operation to perform
    action: Action,
    slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contractor {
    pub slug: String,
    pub name: String,
}

impl Crud<'_> for Contractor {
    const FILE: &'static str = "contractors_test.toml";
    type DeserializeErr = toml::de::Error;
    type SerializeErr = toml::ser::Error;

    fn identifier(&self) -> String {
        self.slug.to_owned()
    }

    fn deserialize(tomlstr: String) -> Result<HashMap<String, Contractor>, Self::DeserializeErr> {
        from_toml(&tomlstr)
    }

    fn serialize(map: HashMap<String, Contractor>) -> Result<String, Self::SerializeErr> {
        to_toml(&map)
    }

    fn interactive_update(&self) -> Self {
        let name = input::<String>()
            .msg("Contractor name: ")
            .default(self.name.clone())
            .get();
        let slug = self.slug.clone();
        Self { name, slug }
    }
}

impl View for Contractor {
    fn format_list_item(&self) -> String {
        format!(
            "{:7} {:10}",
            self.slug.bold().red(),
            self.name.bold().blue()
        )
    }
}

impl Contractor {
    fn new() -> Self {
        let name = input::<String>().msg("Contractor name: ").get();
        let slug = slugify(name.clone());
        let slug_msg = format!(
            "Contractor reference (lowercase and no spaces) [{}]: ",
            &slug
        );
        let slug = input::<String>()
            .add_test(|x| *x == slugify(x.into()))
            .msg(slug_msg)
            .default(slug)
            .get();
        Self { slug, name }
    }
}
