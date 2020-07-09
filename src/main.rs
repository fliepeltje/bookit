mod book;
mod config;
mod generics;
use book::{exec_cmd_book, BookingArgs};
use config::{exec_cmd_config, ConfigArgs};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Opt {
    /// Book time for a project alias
    #[structopt(name = "book")]
    Book(BookingArgs),
    /// View and modify bookit configuration
    #[structopt(name = "config")]
    Config(ConfigArgs),
}

fn main() {
    match Opt::from_args() {
        Opt::Book(args) => exec_cmd_book(args),
        Opt::Config(args) => exec_cmd_config(args),
        opt => println!("{:?}", opt),
    };
}
