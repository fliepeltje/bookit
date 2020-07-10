use snafu::Snafu;
use std::env::VarError;

#[derive(Debug, Snafu)]
pub enum CliError {
    #[snafu(display(
        "{} is not a valid action. Try: add | view | update | delete",
        action_input
    ))]
    UnknownAction { action_input: String },

    #[snafu(display("{} could not be found in the current environment", var))]
    MissingEnvVar { var: String, source: VarError },
    #[snafu(display("unable to read from file {}", source))]
    ReadFail { source: std::io::Error },
    #[snafu(display("unable to write content to file {}", source))]
    WriteFail { source: std::io::Error },
    #[snafu(display("unable to deserialize file content:\n{}", content))]
    DeserializeFail { content: String },
    #[snafu(display("unable to serialize struct"))]
    SerializeFail,
    #[snafu(display("object with slug {} already exists", slug))]
    SlugExists { slug: String },
    #[snafu(display("object with slug {} doesn't exist", slug))]
    SlugMissing { slug: String },
}
