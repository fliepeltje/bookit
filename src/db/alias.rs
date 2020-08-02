use crate::alias::Alias;
use crate::generics::Result;
use rusqlite::{named_params, params, Connection, NO_PARAMS};


pub fn create_alias(conn: &Connection, alias: &Alias) -> Result<()> {
    conn.execute_named(
        "insert into alias (slug, contractor, rate) values (:slug, :contractor, :rate)",
        named_params! {
            ":slug": alias.slug,
            ":contractor": alias.contractor,
            ":rate": alias.hourly_rate
        }
    )?;
    Ok(())
}

pub fn delete_alias(conn: &Connection, alias: &Alias) -> Result<()> {
    conn.execute(
        "delete from alias where slug = 1?",
        params![alias.slug]
    )?;
    Ok(())
}

pub fn get_aliases(conn: &Connection) -> Result<Vec<Alias>> {
    let mut statement = conn.prepare(
        "select a.slug, a.rate, c.slug, c.name from alias a \
        left join contractor c \
        on a.contractor = c.slug"
    )?;
    let fmt = |x: String| format!("alias for {}", x);
    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(Alias {
            slug: row.get(0)?,
            hourly_rate: row.get(1)?,
            contractor: row.get(2)?,
            short_description: fmt(row.get(3)?)
        })
    })?;
    Ok(iter.map(|a| a.unwrap()).collect::<Vec<Alias>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrate;
    use crate::db::contractors::create_contractor;
    use crate::contractors::Contractor;

    #[test]
    fn cant_create_alias_without_contractor() {
        let conn = Connection::open_in_memory().unwrap();
        let conn = migrate(conn).unwrap();
        let alias = Alias {
            slug: "alias".into(),
            hourly_rate: 10,
            contractor: "aliascont".into(),
            short_description: "".into()
        };
        assert!(create_alias(&conn, &alias).is_err());
    }

    #[test]
    fn can_create_and_retrieve_alias_with_contractor() {
        let conn = Connection::open_in_memory().unwrap();
        let conn = migrate(conn).unwrap();
        let cont = Contractor {
            name: "Cont".into(),
            slug: "aliascont".into()
        };
        create_contractor(&conn, &cont).unwrap();
        let alias = Alias {
            slug: "alias".into(),
            hourly_rate: 10,
            contractor: "aliascont".into(),
            short_description: "".into()
        };
        assert!(create_alias(&conn, &alias).is_ok());
        assert!(get_aliases(&conn).is_ok());
    }
}