use crate::errors::CliError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs, path};

type Result<T, E = CliError> = std::result::Result<T, E>;
type Mapping<T> = HashMap<String, T>;

pub trait Crud<'de>
where
    Self: std::marker::Sized,
    Self: Serialize,
    Self: Deserialize<'de>,
    Self: Clone,
{
    const FILE: &'static str;
    type DeserializeErr;
    type SerializeErr;
    fn identifier(&self) -> String;
    fn deserialize(s: String) -> Result<Mapping<Self>, Self::DeserializeErr>;
    fn serialize(map: HashMap<String, Self>) -> Result<String, Self::SerializeErr>;
    fn interactive_update(&self) -> Self;

    fn path() -> Result<path::PathBuf> {
        match env::var("BOOKIT_DIR") {
            Ok(path_str) => Ok(path::Path::new(&path_str).join(Self::FILE)),
            Err(source) => Err(CliError::MissingEnvVar {
                source,
                var: "BOOKIT_DIR".into(),
            }),
        }
    }

    fn file_content() -> Result<String> {
        let f = Self::path()?;
        match fs::read_to_string(f) {
            Ok(s) => Ok(s.clone()),
            Err(err) => Err(CliError::ReadFail { source: err }),
        }
    }

    fn mapping() -> Result<Mapping<Self>> {
        let content = Self::file_content()?;
        match Crud::deserialize(content) {
            Ok(map) => Ok(map),
            Err(_) => Err(CliError::DeserializeFail { content }),
        }
    }

    fn commit_map(map: HashMap<String, Self>) -> Result<()> {
        match Crud::serialize(map) {
            Ok(s) => match fs::write(Self::path()?, s) {
                Ok(_) => Ok(()),
                Err(source) => Err(CliError::WriteFail { source }),
            },
            Err(_) => Err(CliError::SerializeFail),
        }
    }

    fn add(&self) -> Result<()> {
        let slug = self.identifier();
        if Self::exists(&slug) {
            Err(CliError::SlugExists { slug })
        } else {
            let mut mapping = Self::mapping()?;
            mapping.insert(self.identifier(), self.clone());
            Self::commit_map(mapping);
            Ok(())
        }
    }

    fn delete(&self) -> Result<()> {
        let slug = self.identifier();
        if Self::exists(&slug) {
            let mut mapping = Self::mapping()?;
            mapping.remove(&slug);
            Ok(Self::commit_map(mapping)?)
        } else {
            Err(CliError::SlugMissing { slug })
        }
    }

    fn overwrite(&self) -> Result<()> {
        let slug = self.identifier();
        if Self::exists(&slug) {
            let mut mapping = Self::mapping()?;
            mapping.remove(&slug);
            mapping.insert(slug, self.clone());
            Ok(Self::commit_map(mapping)?)
        } else {
            Err(CliError::SlugMissing { slug })
        }
    }

    fn exists(slug: &str) -> bool {
        match Self::mapping() {
            Ok(map) => map.contains_key(slug),
            Err(_) => false,
        }
    }

    fn retrieve(slug: &str) -> Result<Self> {
        let mapping = Self::mapping()?;
        match mapping.get(slug) {
            Some(obj) => Ok(obj.clone()),
            None => Err(CliError::SlugMissing {
                slug: slug.to_owned(),
            }),
        }
    }
}

pub trait View
where
    Self: std::marker::Sized,
{
    fn format_list_item(&self) -> String;

    fn format_detail(&self) -> String {
        self.format_list_item()
    }
    fn format_list(items: Vec<Self>) -> String {
        let v = items
            .iter()
            .map(|x| x.format_list_item())
            .collect::<Vec<String>>();
        v.join("\n")
    }
}

pub fn add_subject<'de, T>(obj: T) -> Result<()>
where
    T: Crud<'de>,
{
    Ok(obj.add()?)
}

pub fn update_subject<'de, T>(obj_slug: &str) -> Result<()>
where
    T: Crud<'de>,
{
    let obj = T::retrieve(obj_slug)?;
    let obj = obj.interactive_update();
    Ok(obj.overwrite()?)
}

pub fn delete_subject<'de, T>(obj_slug: &str) -> Result<()>
where
    T: Crud<'de>,
{
    let obj = T::retrieve(obj_slug)?;
    Ok(obj.delete()?)
}

pub fn view_subject<'de, T>(obj_slug: Option<String>) -> Result<()>
where
    T: Crud<'de>,
    T: View,
{
    match obj_slug {
        Some(slug) => println!("{}", T::retrieve(&slug)?.format_detail()),
        None => {
            let items = T::mapping()?.values().cloned().collect::<Vec<T>>();
            println!("{}", T::format_list(items))
        }
    };
    Ok(())
}
