mod alias;
mod contractors;
mod errors;
mod generics;
mod hours;
mod utils;
use contractors::{exec_cmd_contractor, ContractorArgs};
use errors::CliError;
use generics::Result;
use structopt::StructOpt;
#[macro_use]
extern crate pipeline;

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
    /// Manage aliases
    #[structopt(name = "alias")]
    Alias(alias::Cmd),
    /// Manage contractors
    #[structopt(name = "contractors")]
    Contractors(ContractorArgs),
    /// Manage hours
    #[structopt(name = "hours")]
    Hours(hours::Cmd),
}

fn main() -> () {
    let r = match Opt::from_args() {
        Opt::Alias(cmd) => cmd.exec(),
        Opt::Contractors(args) => exec_cmd_contractor(args),
        Opt::Hours(cmd) => cmd.exec(),
    };
    match r {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
