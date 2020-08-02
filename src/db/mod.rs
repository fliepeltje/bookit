mod alias;
mod contractors;
mod hours;
use crate::errors::CliError;
use crate::generics::Result;
use crate::DB_PATH;
use rusqlite::{Connection, Error, NO_PARAMS};
use std::path::Path;

impl From<Error> for CliError {
    fn from(err: Error) -> Self {
        Self::DbError(err)
    }
}

mod refinery {
    use refinery::embed_migrations;
    embed_migrations!("./src/db/migrations");
}

fn establish_connection() -> Result<Connection> {
    let db_path = Path::new(&DB_PATH);
    if !db_path.exists() {
        let conn = Connection::open(&db_path)?;
        migrate(conn)?;
    };
    Ok(Connection::open(&db_path)?)
}

fn set_db_config(conn: &Connection) -> Result<()> {
    conn.execute("PRAGMA foreign_keys=ON", NO_PARAMS)?;
    Ok(())
}

fn migrate(mut conn: Connection) -> Result<Connection> {
    refinery::migrations::runner().run(&mut conn).unwrap();
    set_db_config(&conn)?;
    Ok(conn)
}

pub trait Crud
where
    Self: std::marker::Sized,
{
    fn create(self) -> Result<()>;
    fn update(&self) -> Result<()>;
    fn delete(self) -> Result<()>;
    fn retrieve(lookup: &str) -> Result<Self>;
    fn retrieve_all() -> Result<Vec<Self>>;

    fn conn() -> Result<Connection> {
        establish_connection()
    }
}
