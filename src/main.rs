mod book;
use book::BookingArgs;
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
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
