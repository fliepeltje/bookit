mod alias;
mod book;
mod contractors;
mod errors;
mod generics;
mod hours;
mod utils;
use alias::{exec_cmd_alias, AliasArgs};
use book::{exec_cmd_book, BookingArgs};
use contractors::{exec_cmd_contractor, ContractorArgs};
use errors::CliError;
use generics::Result;
use hours::{exec_cmd_hours, HourLogArgs};
use structopt::StructOpt;

#[derive(Debug)]
pub enum Action {
    Add,
    Update,
    Delete,
    View,
}

impl std::str::FromStr for Action {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Self::Add),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
            "view" => Ok(Self::View),
            action => Err(CliError::UnknownAction(action.to_string())),
        }
    }
}

impl std::string::ToString for Action {
    fn to_string(&self) -> String {
        let s = match self {
            Action::Add => "add",
            Action::Update => "update",
            Action::Delete => "delete",
            Action::View => "view",
        };
        s.to_owned()
    }
}

#[derive(StructOpt, Debug)]
enum Opt {
    /// Book time for a project alias
    #[structopt(name = "book")]
    Book(BookingArgs),
    /// View or delete existing hourlog item
    #[structopt(name = "hours")]
    Hours(HourLogArgs),
    /// Manage aliases
    #[structopt(name = "alias")]
    Alias(AliasArgs),
    /// Manage contractors
    #[structopt(name = "contractors")]
    Contractors(ContractorArgs),
}

fn main() -> () {
    let r = match Opt::from_args() {
        Opt::Book(args) => exec_cmd_book(args),
        Opt::Alias(args) => exec_cmd_alias(args),
        Opt::Contractors(args) => exec_cmd_contractor(args),
        Opt::Hours(args) => exec_cmd_hours(args),
    };
    match r {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
