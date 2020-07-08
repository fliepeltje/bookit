use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct BookingArgs {
    /// Project alias
    alias: String,
    /// Time in minutes or a stretch pattern (e.g. ::HH:MM | ::last)
    time: u8,
    /// Date in isoformat or weekday (e.g. "YYYY-MM-DD" | <weekday>)
    date: Option<String>,
    /// Description of time expenditure (must pass spelling check)
    message: Option<String>,
    /// Reference to work ticket (e.g. "RAS-002")
    ticket: Option<String>,
    /// Reference to git branch for work (e.g. "feature/RAS-002")
    branch: Option<String>,
}
