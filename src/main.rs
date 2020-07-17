mod alias;
mod contractors;
mod db;
mod errors;
mod generics;
mod hours;
mod utils;
use structopt::StructOpt;
#[macro_use]
extern crate pipeline;

pub const BOOKIT_PATH: &'static str = "BOOKIT_PATH";
pub const DB_PATH: &'static str = "BOOKIT_DB_PATH";

#[derive(StructOpt, Debug)]
enum Opt {
    /// Manage aliases
    #[structopt(name = "alias")]
    Alias(alias::Cmd),
    /// Manage contractors
    #[structopt(name = "contractors")]
    Contractors(contractors::Cmd),
    /// Manage hours
    #[structopt(name = "hours")]
    Hours(hours::Cmd),
}

fn main() -> () {
    let r = match Opt::from_args() {
        Opt::Alias(cmd) => cmd.exec(),
        Opt::Contractors(cmd) => cmd.exec(),
        Opt::Hours(cmd) => cmd.exec(),
    };
    match r {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
