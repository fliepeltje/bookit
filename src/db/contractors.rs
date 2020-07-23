use crate::contractors::Contractor;
use crate::db::migrate;
use crate::generics::Result;
use rusqlite::{named_params, params, Connection, NO_PARAMS};

pub fn create_contractor(conn: &Connection, contractor: &Contractor) -> Result<()> {
    conn.execute_named(
        "insert into contractor (slug, name) values (:slug, :name)",
        named_params! {
            ":name": contractor.name,
            ":slug": contractor.slug,
        },
    )?;
    Ok(())
}

pub fn delete_contractor(conn: &Connection, contractor: &Contractor) -> Result<()> {
    conn.execute(
        "delete from contractor where slug = 1?",
        params![contractor.slug],
    )?;
    Ok(())
}

pub fn get_contractors(conn: &Connection) -> Result<Vec<Contractor>> {
    let mut statement = conn.prepare("select slug, name from contractor")?;
    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(Contractor {
            slug: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    Ok(iter.map(|c| c.unwrap()).collect::<Vec<Contractor>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_contractor() {
        let conn = Connection::open_in_memory().unwrap();
        let conn = migrate(conn);
        let contractor = Contractor {
            slug: "cont".into(),
            name: "Contractor".into(),
        };
        assert!(create_contractor(&conn, &contractor).is_ok());
    }
}
