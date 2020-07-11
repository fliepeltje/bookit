use colored::*;
use std::env::VarError;
use std::error::Error;

#[derive(Debug)]
pub enum CliError {
    UnknownAction(String),
    Slug { slug: String, expect: bool },
    Read(std::io::Error),
    Write(std::io::Error),
    Serialization(String),
    Env(String, VarError),
    Directive { input: String, context: String },
    Parse { input: String, description: String },
}

impl Error for CliError {}

fn error_descriptor(err_type: &str) -> ColoredString {
    return format!("[{} Error]", err_type).bold().red();
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Env(var, e) => write!(
                f,
                "{} {} {}",
                error_descriptor("Environment"),
                var.yellow(),
                e
            ),
            Self::UnknownAction(action) => {
                let actions = format!(
                    "{} | {} | {} | {}",
                    "add".green(),
                    "update".green(),
                    "delete".green(),
                    "view".green()
                );
                write!(
                    f,
                    "\n\n{} {} is not a valid action (actions: {})",
                    error_descriptor("Usage"),
                    action.yellow(),
                    actions
                )
            }
            Self::Slug { slug, expect } => {
                let suffix = if *expect {
                    "was not found but was expected"
                } else {
                    "already exists"
                };
                write!(
                    f,
                    "{} slug {} {}",
                    error_descriptor("Lookup"),
                    slug.yellow().bold(),
                    suffix
                )
            }
            Self::Serialization(msg) => {
                write!(f, "{} {}", error_descriptor("Data Transformation"), msg)
            }
            Self::Read(e) | Self::Write(e) => write!(f, "{} {}", error_descriptor("IO"), e),
            Self::Directive { input, context } => write!(
                f,
                "{} {} {}",
                error_descriptor("Directive"),
                input.yellow(),
                context
            ),
            Self::Parse { input, description } => write!(
                f,
                "{} unable to parse {} - {}",
                error_descriptor("Parse"),
                input.yellow(),
                description
            ),
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
