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
    Serialization { msg: String },
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

impl From<toml::ser::Error> for CliError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serialization {
            msg: err.to_string(),
        }
    }
}

impl From<toml::de::Error> for CliError {
    fn from(err: toml::de::Error) -> CliError {
        Self::Serialization {
            msg: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> CliError {
        Self::Serialization {
            msg: err.to_string(),
        }
    }
}
