use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs, path};
use structopt::StructOpt;
use toml::{from_str as from_toml, to_string as to_toml};

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

trait ConfigCrud<'de>
where
    Self: std::marker::Sized,
    Self: Serialize,
    Self: Deserialize<'de>,
    Self: Clone,
{
    const FILE: &'static str;
    type DeserializeErr;
    type SerializeErr;

    fn new() -> Self;
    fn identifier(&self) -> String;
    fn deserialize(s: String) -> Result<HashMap<String, Self>, Self::DeserializeErr>;
    fn serialize(map: HashMap<String, Self>) -> Result<String, Self::SerializeErr>;
    fn update(&self) -> Self;

    fn path() -> path::PathBuf {
        let dir = env::var("BOOKIT_DIR").expect("No BOOKIT_DIR specified in environment");
        let dir = path::Path::new(&dir);
        let filepath = dir.join(Self::FILE);
        filepath
    }

    fn file_content() -> Option<String> {
        let path = Self::path();
        match fs::read_to_string(path) {
            Ok(s) => Some(s.clone()),
            Err(_) => None,
        }
    }

    fn mapping() -> HashMap<String, Self> {
        match Self::file_content() {
            Some(s) => match ConfigCrud::deserialize(s) {
                Ok(map) => map,
                Err(_) => panic!("Unable to deserialize object, file might be corrupt"),
            },
            None => HashMap::new(),
        }
    }

    fn commit_map(map: HashMap<String, Self>) -> () {
        match ConfigCrud::serialize(map) {
            Ok(s) => fs::write(Self::path(), s).expect("Unable to write to file"),
            Err(_) => panic!("Unknown error"),
        }
    }

    fn add(&self) -> () {
        let slug = self.identifier();
        if Self::exists(&slug) {
            panic!("Object with given slug already exists")
        } else {
            let mut mapping = Self::mapping();
            mapping.insert(self.identifier(), self.clone());
            Self::commit_map(mapping);
        }
    }

    fn delete(&self) -> () {
        let slug = self.identifier();
        if Self::exists(&slug) {
            let mut mapping = Self::mapping();
            mapping.remove(&slug);
            Self::commit_map(mapping);
        } else {
            panic!("Object with given slug does not exist")
        }
    }

    fn overwrite(&self) -> () {
        let slug = self.identifier();
        if Self::exists(&slug) {
            let mut mapping = Self::mapping();
            mapping.remove(&slug);
            mapping.insert(slug, self.clone());
            Self::commit_map(mapping);
        } else {
            panic!("Object with given slug does not exist")
        }
    }

    fn exists(slug: &str) -> bool {
        let map = Self::mapping();
        map.contains_key(slug)
    }

    fn retrieve(slug: &str) -> Self {
        let mapping = Self::mapping();
        match mapping.get(slug) {
            Some(obj) => obj.clone(),
            None => panic!("Object does not exist"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contractor {
    pub slug: String,
    pub name: String,
}

impl ConfigCrud<'_> for Contractor {
    const FILE: &'static str = "contractors_test.toml";
    type DeserializeErr = toml::de::Error;
    type SerializeErr = toml::ser::Error;

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
        Self { slug, name }
    }

    fn identifier(&self) -> String {
        self.slug.to_owned()
    }

    fn deserialize(tomlstr: String) -> Result<HashMap<String, Contractor>, Self::DeserializeErr> {
        from_toml(&tomlstr)
    }

    fn serialize(map: HashMap<String, Contractor>) -> Result<String, Self::SerializeErr> {
        to_toml(&map)
    }

    fn update(&self) -> Self {
        let name = input::<String>()
            .msg("Contractor name: ")
            .default(self.name.clone())
            .get();
        let slug = self.slug.clone();
        Self { name, slug }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alias {
    pub slug: String,
    pub contractor: String,
    pub short_description: String,
    pub hourly_rate: u8,
}

impl ConfigCrud<'_> for Alias {
    const FILE: &'static str = "alias_test.toml";
    type DeserializeErr = toml::de::Error;
    type SerializeErr = toml::ser::Error;

    fn identifier(&self) -> String {
        self.slug.to_owned()
    }

    fn deserialize(tomlstr: String) -> Result<HashMap<String, Alias>, Self::DeserializeErr> {
        from_toml(&tomlstr)
    }

    fn serialize(map: HashMap<String, Alias>) -> Result<String, Self::SerializeErr> {
        to_toml(&map)
    }

    fn new() -> Self {
        let slug = input::<String>()
            .msg("Alias: ")
            .add_test(|x| *x == slugify(x.into()))
            .get();
        let contractor = input::<String>().msg("Contractor slug: ").get();
        let short_description = input::<String>().msg("Brief description: ").get();
        let hourly_rate = input::<u8>().msg("Hourly rate: ").get();
        Self {
            slug,
            contractor,
            short_description,
            hourly_rate,
        }
    }

    fn update(&self) -> Self {
        let slug = self.slug.to_owned();
        let contractor = input::<String>()
            .msg(format!("Contractor slug: [{}]", self.contractor))
            .default(self.contractor.clone())
            .get();
        let short_description = input::<String>()
            .msg(format!("Brief description: [{}]", self.short_description))
            .default(self.short_description.clone())
            .get();
        let hourly_rate = input::<u8>()
            .msg(format!("Hourly rate: [{}]", self.hourly_rate))
            .default(self.hourly_rate)
            .get();
        Self {
            slug,
            contractor,
            short_description,
            hourly_rate,
        }
    }
}

fn slugify(s: String) -> String {
    s.to_lowercase().split_whitespace().collect()
}

fn add_subject<'de, T>() -> ()
where
    T: ConfigCrud<'de>,
{
    let obj = T::new();
    obj.add();
}

fn update_subject<'de, T>(obj_slug: &str) -> ()
where
    T: ConfigCrud<'de>,
{
    let obj = T::retrieve(obj_slug);
    let obj = obj.update();
    obj.overwrite();
}

fn delete_subject<'de, T>(obj_slug: &str) -> ()
where
    T: ConfigCrud<'de>,
{
    let obj = T::retrieve(obj_slug);
    obj.delete();
}

pub fn exec_cmd_config(args: ConfigArgs) {
    match args {
        ConfigArgs {
            subject,
            action: Action::Create,
            subject_id: None,
        } => match subject {
            Subject::Contractor => add_subject::<Contractor>(),
            Subject::Alias => add_subject::<Alias>(),
        },
        ConfigArgs {
            subject,
            action: Action::Update,
            subject_id: Some(s),
        } => match subject {
            Subject::Contractor => update_subject::<Contractor>(&s),
            Subject::Alias => update_subject::<Alias>(&s),
        },
        ConfigArgs {
            subject,
            action: Action::Delete,
            subject_id: Some(s),
        } => match subject {
            Subject::Contractor => delete_subject::<Contractor>(&s),
            Subject::Alias => delete_subject::<Alias>(&s),
        },
        ConfigArgs {
            subject,
            action: Action::Inspect,
            subject_id: Some(s),
        } => match subject {
            Subject::Contractor => println!("Not implemented"),
            Subject::Alias => println!("Not implemented"),
        },
        _ => println!("Nothing happened"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_ok() {
        assert_eq!(slugify("Upper spaced".into()), String::from("upperspaced"))
    }
}
