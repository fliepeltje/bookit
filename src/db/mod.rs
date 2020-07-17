use crate::errors::CliError;
use crate::generics::Result;
use crate::DB_PATH;
use rusqlite::{Connection, Error};
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
        migrate(conn);
    };
    Ok(Connection::open(&db_path)?)
}

fn migrate(mut conn: Connection) -> () {
    refinery::migrations::runner().run(&mut conn).unwrap();
}
