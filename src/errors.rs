use snafu::Snafu;
use std::env::VarError;

#[derive(Debug, Snafu)]
pub enum CliError {
    UnknownAction { action_input: String },
    MissingEnvVar { msg: String },
    SlugExists { slug: String },
    SlugMissing { slug: String },
    Slug { slug: String, expect: bool },
    IO { msg: String },
}

impl From<VarError> for CliError {
    fn from(err: VarError) -> Self {
        Self::MissingEnvVar {
            msg: err.to_string(),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        Self::IO {
            msg: err.to_string(),
        }
    }
}
