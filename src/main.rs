mod book;
use book::{exec_cmd_book, BookingArgs};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct ConfigArgs {}

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
        opt => println!("{:?}", opt),
    };
}
