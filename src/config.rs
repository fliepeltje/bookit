use chrono::{Local as LocalTime, NaiveDateTime};
use read_input::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Action {
    Create,
    Update,
    Inspect,
    Delete,
}

impl std::str::FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new" => Ok(Self::Create),
            "update" => Ok(Self::Update),
            "show" => Ok(Self::Inspect),
            "delete" => Ok(Self::Delete),
            ctx => Err(format!("Unknown action: {}", ctx)),
        }
    }
}

#[derive(StructOpt, Debug)]
enum Subject {
    Contractor,
    Alias,
}

impl std::str::FromStr for Subject {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "contract" | "contractor" => Ok(Self::Contractor),
            "alias" => Ok(Self::Alias),
            ctx => Err(format!("Unknown config subject: {}", ctx)),
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct ConfigArgs {
    /// Subject of configuration (contractor | alias)
    subject: Subject,
    /// Type of config operation to execute (new | update | show | delete)
    action: Action,
    /// Optional id of subject to manage
    subject_id: Option<String>,
}

trait ConfigCrud {
    fn new() -> Self;
    // fn store(self) -> ();
    // fn retrieve(slug: String) -> Result<Self, String>;
    // fn update(self) -> Self;
    // fn delete(self) -> ();
    // fn inspect(self) -> ();
}

#[derive(Debug)]
pub struct Contractor {
    pub slug: String,
    pub name: String,
    pub added_on: NaiveDateTime,
}

impl ConfigCrud for Contractor {
    fn new() -> Self {
        let name = input::<String>().msg("Contractor name: ").get();
        let slug = slugify(name.clone());
        let slug_msg = format!(
            "Contractor reference (lowercase and no spaces) [{}]: ",
            &slug
        );
        let slug = input::<String>()
            .add_test(|x| *x == slugify(x.into()))
            .msg(slug_msg)
            .default(slug)
            .get();
        let added_on = LocalTime::now().naive_local();
        Self {
            slug,
            name,
            added_on,
        }
    }
}

fn slugify(s: String) -> String {
    s.to_lowercase().split_whitespace().collect()
}

pub fn exec_cmd_config(args: ConfigArgs) {
    match args {
        ConfigArgs {
            subject,
            action: Action::Create,
            subject_id: None,
        } => match subject {
            Subject::Contractor => {
                let contractor = Contractor::new();
                println!("{:?}", contractor)
            }
            Subject::Alias => println!("Nope"),
        },
        _ => println!("Nothing happened"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn slugify_ok() {
        assert_eq!(slugify("Upper spaced".into()), String::from("upperspaced"))
    }
}
