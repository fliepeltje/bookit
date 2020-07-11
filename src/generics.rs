use crate::errors::CliError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs, path};

pub type Result<T, E = CliError> = std::result::Result<T, E>;
type Mapping<T> = HashMap<String, T>;

pub trait Crud<'de>
where
    Self: std::marker::Sized,
    Self: Serialize,
    Self: Deserialize<'de>,
    Self: Clone,
{
    const FILE: &'static str;
    fn identifier(&self) -> String;
    fn deserialize(s: String) -> Result<Mapping<Self>>;
    fn serialize(map: HashMap<String, Self>) -> Result<String>;
    fn interactive_update(&self) -> Self;

    fn path() -> Result<path::PathBuf> {
        let basedir = match env::var("BOOKIT_DIR") {
            Ok(dir) => Ok(dir),
            Err(var_error) => Err(CliError::Env("BOOKIT_DIR".to_string(), var_error)),
        }?;
        Ok(path::Path::new(&basedir).join(Self::FILE))
    }

    fn file_content() -> Result<String> {
        match fs::read_to_string(Self::path()?) {
            Ok(s) => Ok(s),
            Err(io_err) => Err(CliError::Read(io_err)),
        }
    }

    fn mapping() -> Result<Mapping<Self>> {
        let content = Self::file_content()?;
        let map = Crud::deserialize(content)?;
        Ok(map)
    }

    fn commit_map(map: HashMap<String, Self>) -> Result<()> {
        let s = Crud::serialize(map)?;
        match fs::write(Self::path()?, s) {
            Ok(()) => Ok(()),
            Err(io_err) => Err(CliError::Write(io_err)),
        }
    }

    fn add(&self) -> Result<()> {
        let slug = self.identifier();
        Self::write_ok(&slug, false)?;
        let mut mapping = Self::mapping()?;
        mapping.insert(self.identifier(), self.clone());
        Self::commit_map(mapping)?;
        Ok(())
    }

    fn delete(&self) -> Result<()> {
        let slug = self.identifier();
        Self::write_ok(&slug, true)?;
        let mut mapping = Self::mapping()?;
        mapping.remove(&slug);
        Ok(Self::commit_map(mapping)?)
    }

    fn overwrite(&self) -> Result<()> {
        let slug = self.identifier();
        Self::write_ok(&slug, true)?;
        let mut mapping = Self::mapping()?;
        mapping.remove(&slug);
        mapping.insert(slug, self.clone());
        Ok(Self::commit_map(mapping)?)
    }

    fn write_ok(slug: &str, slug_expect: bool) -> Result<()> {
        let map = Self::mapping()?;
        match (slug_expect, map.contains_key(slug)) {
            (true, true) | (false, false) => Ok(()),
            (x, _) => Err(CliError::Slug {
                slug: slug.into(),
                expect: x,
            }),
        }
    }

    fn retrieve(slug: &str) -> Result<Self> {
        let mapping = Self::mapping()?;
        match mapping.get(slug) {
            Some(obj) => Ok(obj.clone()),
            None => Err(CliError::Slug {
                slug: slug.to_owned(),
                expect: true,
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
