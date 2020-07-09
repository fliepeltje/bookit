mod alias;
mod book;
mod contractors;
mod generics;
mod utils;
use alias::{exec_cmd_alias, AliasArgs};
use book::{exec_cmd_book, BookingArgs};
use contractors::{exec_cmd_contractor, ContractorArgs};
use structopt::StructOpt;

#[derive(Debug)]
pub enum Action {
    Add,
    Update,
    Delete,
    View,
}

impl std::str::FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Self::Add),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
            "view" => Ok(Self::View),
            _ => Err("nope".into()),
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
    /// Manage aliases
    #[structopt(name = "alias")]
    Alias(AliasArgs),
    /// Manage contractors
    #[structopt(name = "contractors")]
    Contractors(ContractorArgs),
}

fn main() {
    match Opt::from_args() {
        Opt::Book(args) => exec_cmd_book(args),
        Opt::Alias(args) => exec_cmd_alias(args),
        Opt::Contractors(args) => exec_cmd_contractor(args),
    };
}
