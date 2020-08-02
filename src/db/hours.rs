use crate::hours::HourLog;
use crate::generics::Result;
use chrono::{NaiveDate, NaiveDateTime};
use std::str::FromStr;
use rusqlite::{named_params, params, Connection, NO_PARAMS};


pub fn create_hourlog(conn: &Connection, hours: &HourLog) -> Result<()> {
    conn.execute(
        "insert into timelog (hash, alias, minutes, date, message, ticket, timestamp) \
        values (:hash, :alias, :minutes, :date, :message, :ticket, :timestamp)",
        named_params! {
            ":hash": hours.id.to_owned(),
            ":alias": hours.alias.to_owned(),
            ":minutes": hours.minutes.to_owned(),
            ":date": hours.date.to_string(),
            ":message": hours.message.to_owned(),
            ":ticket": hours.ticket.to_owned(),
            ":timestamp": hours.timestamp.to_string()
        })?;
    Ok(())
}

pub fn delete_hourlog(conn: &Connection, hours: HourLog) -> Result<()> {
    conn.execute(
        "delete from timelog where hash = 1?",
        params![hours.id]
    )?;
    Ok(())
}

pub fn get_hours(conn: &Connection) -> Result<Vec<HourLog>> {
    let mut statement = conn.prepare(
        "select h.alias, h.minutes, h.date, h.message \
        h.ticket, h.branch, h.hash, h.timestamp \
        from timelog h"
    )?;
    let map_date = |x: String| NaiveDate::from_str(&x);
    let map_datetime = |x: String| NaiveDateTime::from_str(&x);
    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(HourLog {
            alias: row.get(0)?,
            minutes: row.get(1)?,
            date: map_date(row.get(2)?).unwrap(),
            message: row.get(3)?,
            ticket: row.get(4)?,
            branch: row.get(5)?,
            id: row.get(6)?,
            timestamp: map_datetime(row.get(7)?).unwrap()
        })
    })?;
    Ok(iter.map(|h| h.unwrap()).collect::<Vec<HourLog>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::contractors::Contractor;
    use crate::alias::Alias;
    use crate::hours::HourLog;
    use chrono::{Local};

    fn generate_valid_alias() -> Alias {
        let contractor = Contractor {
            name: "TestCont".into(),
            slug: "test-cont".into()
        };
        let alias = Alias {
            slug: "test-alias".into(),
            contractor: contractor.slug.to_owned(),
            hourly_rate: 10,
            short_description: "".into()
        };
        let conn = Connection::open_in_memory().unwrap();
        let conn = db::migrate(conn).unwrap();
        db::contractors::create_contractor(&conn, &contractor);
        db::alias::create_alias(&conn, &alias);
        alias
    }

    #[test]
    fn can_generate_hourlog_with_valid_alias() {
        let conn = Connection::open_in_memory().unwrap();
        let conn = db::migrate(conn).unwrap();
        let alias = generate_valid_alias();
        let log = HourLog {
            alias: alias.slug.into(),
            minutes: 30,
            date: Local::now().naive_local().date(),
            message: Some("such important".into()),
            ticket: None,
            branch: None,
            id: "flooby".into(),
            timestamp: Local::now().naive_local()
        };
        assert!(create_hourlog(&conn, &log).is_ok())
    }


}