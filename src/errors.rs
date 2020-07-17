use colored::*;
use rusqlite::Error as RusqError;
use std::env::VarError;
use std::error::Error;

#[derive(Debug)]
pub enum CliError {
    Read(std::io::Error),
    Write(std::io::Error),
    Serialization(String),
    Env(String, VarError),
    Parse { input: String, description: String },
    FilterNoResults,
    CmdError(String),
    BinaryError(String),
    DbError(RusqError),
}

impl Error for CliError {}

fn arg_error(err_type: &str) -> ColoredString {
    format!("\n{}[{} Error]", "└─".red().bold(), err_type)
        .bold()
        .red()
}

fn bin_error(err_type: &str) -> ColoredString {
    format!("[{} Error]", err_type).bold().red()
}

fn warning(err_type: &str) -> ColoredString {
    format!("[{} Warning]", err_type).yellow().bold()
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Env(var, e) => write!(f, "{} {} {}", bin_error("Environment"), var.yellow(), e),
            Self::Serialization(msg) => write!(f, "{} {}", bin_error("Data Transformation"), msg),
            Self::Read(e) | Self::Write(e) => write!(f, "{} {}", bin_error("IO"), e),

            Self::Parse { input, description } => write!(
                f,
                "{} unable to parse {} - {}",
                arg_error("Parse"),
                input.yellow(),
                description
            ),
            Self::FilterNoResults => {
                write!(f, "{} no results based on given filters", warning("Filter"))
            }
            Self::CmdError(msg) => write!(f, "{} {}", arg_error("Usage"), msg),
            Self::BinaryError(msg) => write!(f, "{} {}", bin_error("Internal"), msg),
            Self::DbError(err) => write!(f, "{} {}", bin_error("Database"), err),
        }
    }
}

impl From<toml::ser::Error> for CliError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

impl From<toml::de::Error> for CliError {
    fn from(err: toml::de::Error) -> CliError {
        Self::Serialization(err.to_string())
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> CliError {
        Self::Serialization(err.to_string())
    }
}
