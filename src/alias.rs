use crate::contractors::Contractor;
use crate::errors::CliError;
use crate::generics::{
    add_subject, delete_subject, update_subject, view_filtered_set, view_subject, Crud, Filter,
    Result, View,
};
use crate::utils::{partition_directive, slugify};
use colored::*;
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use structopt::StructOpt;
use toml::{from_str as from_toml, to_string as to_toml};

enum AliasError {
    InvalidFilterField(String),
}

impl From<AliasError> for CliError {
    fn from(err: AliasError) -> Self {
        match err {
            AliasError::InvalidFilterField(f) => {
                Self::CmdError(format!("cannot filter on {}", f.yellow().bold()))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alias {
    pub slug: String,
    pub contractor: String,
    pub short_description: String,
    pub hourly_rate: u8,
}

#[derive(StructOpt, Debug)]
pub enum Cmd {
    /// Create a new alias interactively
    #[structopt(name = "add")]
    Create,
    /// Update an existing alias interactively
    #[structopt(name = "update")]
    Update { alias: Alias },
    /// View a collection of aliases
    #[structopt(name = "show")]
    Show {
        #[structopt(short = "f")]
        filters: Vec<F>,
        #[structopt(short = "s", default_value = "no_sort")]
        sort: S,
    },
    /// View detailed alias stats
    #[structopt(name = "detail")]
    Detail { alias: Alias },
    /// Delete an alias
    #[structopt(name = "delete")]
    Delete { alias: Alias },
}

impl Cmd {
    pub fn exec(&self) -> Result<()> {
        match self {
            Self::Create => add_subject(Alias::new()?)?,
            Self::Delete { alias } => delete_subject::<Alias>(&alias.slug)?,
            Self::Update { alias } => update_subject::<Alias>(&alias.slug)?,
            Self::Detail { alias } => view_subject::<Alias>(Some(alias.slug.clone()))?,
            Self::Show { filters, sort } => {
                view_filtered_set::<Alias, F, S>(filters.to_vec(), sort.clone())?
            }
        };
        Ok(())
    }
}

impl Crud<'_> for Alias {
    const FILE: &'static str = "alias_test.toml";

    fn identifier(&self) -> String {
        self.slug.to_owned()
    }

    fn deserialize(tomlstr: String) -> Result<HashMap<String, Alias>> {
        Ok(from_toml(&tomlstr)?)
    }

    fn serialize(map: HashMap<String, Alias>) -> Result<String> {
        Ok(to_toml(&map)?)
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

impl FromStr for Alias {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self::retrieve(s)?)
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
    fn new() -> Result<Self> {
        let slug = input::<String>()
            .msg("Alias: ")
            .add_test(|x| *x == slugify(x.into()))
            .get();
        let contractor = input::<String>().msg("Contractor slug: ").get();
        let contractor = Contractor::from_str(&contractor)?;
        let contractor = contractor.slug;
        let short_description = input::<String>().msg("Brief description: ").get();
        let hourly_rate = input::<u8>().msg("Hourly rate: ").get();
        Ok(Self {
            slug,
            contractor,
            short_description,
            hourly_rate,
        })
    }
}

#[derive(Debug, Clone)]
pub enum F {
    NoFilter,
    Contractor(String),
}

impl FromStr for F {
    type Err = CliError;

    fn from_str(input: &str) -> Result<Self> {
        match partition_directive(input)? {
            ("contract", val) => Ok(Self::Contractor(val.to_string())),
            (field, _) => Err(AliasError::InvalidFilterField(field.to_owned()).into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum S {
    NoSort,
}

impl FromStr for S {
    type Err = CliError;

    fn from_str(input: &str) -> Result<Self> {
        Ok(Self::NoSort)
    }
}

impl Filter<F, S> for Alias {
    const DEFAULT_SORT: S = S::NoSort;
    const DEFAULT_FILTER: F = F::NoFilter;

    fn get_base_items() -> Result<Vec<Self>> {
        let mapping = Self::mapping()?;
        Ok(mapping.values().cloned().collect::<Vec<Self>>())
    }

    fn filter(items: Vec<Self>, method: F) -> Vec<Self> {
        match method {
            F::NoFilter => items,
            F::Contractor(contractor) => items
                .into_iter()
                .filter(|item| item.contractor == contractor)
                .collect(),
        }
    }

    fn sort(items: Vec<Self>, method: S) -> Vec<Self> {
        items
    }
}
