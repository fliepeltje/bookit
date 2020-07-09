use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs, path};

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
    fn deserialize(s: String) -> Result<HashMap<String, Self>, Self::DeserializeErr>;
    fn serialize(map: HashMap<String, Self>) -> Result<String, Self::SerializeErr>;
    fn interactive_update(&self) -> Self;

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
            Some(s) => match Crud::deserialize(s) {
                Ok(map) => map,
                Err(_) => panic!("Unable to deserialize object, file might be corrupt"),
            },
            None => HashMap::new(),
        }
    }

    fn commit_map(map: HashMap<String, Self>) -> () {
        match Crud::serialize(map) {
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

pub fn add_subject<'de, T>(obj: T) -> ()
where
    T: Crud<'de>,
{
    obj.add();
}

pub fn update_subject<'de, T>(obj_slug: &str) -> ()
where
    T: Crud<'de>,
{
    let obj = T::retrieve(obj_slug);
    let obj = obj.interactive_update();
    obj.overwrite();
}

pub fn delete_subject<'de, T>(obj_slug: &str) -> ()
where
    T: Crud<'de>,
{
    let obj = T::retrieve(obj_slug);
    obj.delete();
}
