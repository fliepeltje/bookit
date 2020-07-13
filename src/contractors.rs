use crate::errors::CliError;
use crate::generics::{
    add_subject, delete_subject, update_subject, view_filtered_set, view_subject, Crud, Filter,
    Result, View,
};
use crate::utils::slugify;
use colored::*;
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use structopt::StructOpt;
use toml::{from_str as from_toml, to_string as to_toml};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contractor {
    pub slug: String,
    pub name: String,
}

#[derive(StructOpt, Debug)]
pub enum Cmd {
    /// Create a new contractor interactively
    #[structopt(name = "add")]
    Create,
    /// Update an existing contractor interactively
    #[structopt(name = "update")]
    Update { contractor: Contractor },
    /// View a collection of contractors
    #[structopt(name = "show")]
    Show {
        #[structopt(short = "f")]
        filters: Vec<F>,
        #[structopt(short = "s", default_value = "no_sort")]
        sort: S,
    },
    /// View detailed contractor stats
    #[structopt(name = "detail")]
    Detail { contractor: Contractor },
    /// Delete a contractor
    #[structopt(name = "delete")]
    Delete { contractor: Contractor },
}

impl Cmd {
    pub fn exec(&self) -> Result<()> {
        match self {
            Self::Create => add_subject(Contractor::new()?)?,
            Self::Delete { contractor } => delete_subject::<Contractor>(&contractor.slug)?,
            Self::Update { contractor } => update_subject::<Contractor>(&contractor.slug)?,
            Self::Detail { contractor } => {
                view_subject::<Contractor>(Some(contractor.slug.clone()))?
            }
            Self::Show { filters, sort } => {
                view_filtered_set::<Contractor, F, S>(filters.to_vec(), sort.clone())?
            }
        };
        Ok(())
    }
}

impl Crud for Contractor {
    const FILE: &'static str = "contractors_test.toml";

    fn identifier(&self) -> String {
        self.slug.to_owned()
    }

    fn deserialize(tomlstr: String) -> Result<HashMap<String, Contractor>> {
        Ok(from_toml(&tomlstr)?)
    }

    fn serialize(map: HashMap<String, Contractor>) -> Result<String> {
        Ok(to_toml(&map)?)
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

impl FromStr for Contractor {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self::retrieve(s)?)
    }
}

impl Contractor {
    fn new() -> Result<Self> {
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
        Ok(Self { slug, name })
    }
}

#[derive(Debug, Clone)]
pub enum F {
    NoFilter,
}

impl FromStr for F {
    type Err = CliError;

    fn from_str(input: &str) -> Result<Self> {
        Ok(F::NoFilter)
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

impl Filter<F, S> for Contractor {
    const DEFAULT_SORT: S = S::NoSort;
    const DEFAULT_FILTER: F = F::NoFilter;

    fn get_base_items() -> Result<Vec<Self>> {
        let mapping = Self::mapping()?;
        Ok(mapping.values().cloned().collect::<Vec<Self>>())
    }

    fn filter(items: Vec<Self>, method: F) -> Vec<Self> {
        items
    }

    fn sort(items: Vec<Self>, method: S) -> Vec<Self> {
        items
    }
}
