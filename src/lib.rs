use rusqlite::{Connection, Result, Row};
use chrono::{DateTime, offset::Utc};

#[derive(Debug, PartialEq)]
pub struct Url {
    location: String,
    target: String,
    created_at: DateTime<Utc>
}

impl Url {
    pub fn new(location: String, target: String) -> Url {
        let created_at = Utc::now();
        Url {
            target,
            location,
            created_at
        }
    }

    fn from_row(row: &Row) -> Result<Url> {
        Ok(Url {
            location: row.get(0)?,
            target: row.get(1)?,
            created_at: row.get(2)?
        })
    }

    pub fn fetch_from_db(connection: Connection, location: String) -> Result<Option<Url>> {
        let mut stmt = connection.prepare("SELECT location, target, created_at FROM url WHERE location=?")?;

        let mut url_iter = stmt.query_map([location], |row| {
            Url::from_row(row)
        })?;

       url_iter.next().transpose()
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_fetch_from_db() -> Result<()> {
        let conn = Connection::open("fixtures/test.db")?;
        let mut expected_url = Url::new("test_location_1".to_string(), "test_target_1".to_string());
        expected_url.created_at = Utc.ymd(2021, 6, 8).and_hms_milli(11, 29, 11, 124);
        assert_eq!(Url::fetch_from_db(conn, "test_location_1".to_string())?.unwrap(), expected_url);
        Ok(())
    }
}
