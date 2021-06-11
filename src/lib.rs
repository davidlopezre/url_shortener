use chrono::{offset::Utc, DateTime};
use rusqlite::{params, Connection, Result, Row};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Url {
    location: String,
    target: String,
    created_at: DateTime<Utc>,
}

impl Url {
    pub fn new(location: String, target: String) -> Url {
        let created_at = Utc::now();
        Url {
            target,
            location,
            created_at,
        }
    }

    pub fn post_to_db(&self, connection: &Connection) -> Result<()> {
        let mut stmt =
            connection.prepare("INSERT INTO url(location, target, created_at) VALUES(?, ?, ?)")?;
        let result = stmt.execute(params![self.location, self.target, self.created_at]);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn from_row(row: &Row) -> Result<Url> {
        Ok(Url {
            location: row.get(0)?,
            target: row.get(1)?,
            created_at: row.get(2)?,
        })
    }

    pub fn fetch_from_db(connection: &Connection, location: String) -> Result<Option<Url>> {
        let mut stmt =
            connection.prepare("SELECT location, target, created_at FROM url WHERE location=?")?;

        let mut url_iter = stmt.query_map([location], |row| Url::from_row(row))?;

        url_iter.next().transpose()
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use scopeguard::defer;

    use super::*;
    use std::fs;
    use std::iter::repeat_with;

    fn initialise_test_db() -> (String, Box<dyn FnOnce()>) {
        let test_file_slug :String = repeat_with(fastrand::alphanumeric).take(10).collect();
        let test_file_name: String = format!("fixtures/{}", test_file_slug);
        match fs::copy("fixtures/test.db", &test_file_name) {
            Ok(_) => {
                return (test_file_name.clone(), Box::new(|| match fs::remove_file(test_file_name) {
                    Err(_) => panic!("failed to cleanup"),
                    _ => return,
                }));
            }
            Err(e) => panic!("failed to set up test : {}", e),
        }
    }

    #[test]
    fn test_fetch_from_db() -> Result<()> {
        let (test_db, cleanup) = initialise_test_db();
        defer!(cleanup());
        
        let conn = Connection::open(test_db)?;
        let mut expected_url = Url::new("test_location_1".to_string(), "test_target_1".to_string());
        expected_url.created_at = Utc.ymd(2021, 6, 8).and_hms_milli(11, 29, 11, 124);
        assert_eq!(
            Url::fetch_from_db(&conn, "test_location_1".to_string())?.unwrap(),
            expected_url
        );
        Ok(())
    }

    #[test]
    fn test_post_to_db() -> Result<()> {
        let (test_db, cleanup) = initialise_test_db();
        defer!(cleanup());

        let conn = Connection::open(test_db)?;
        let url = Url::new("test_location_2".to_string(), "test_target_2".to_string());
        let result = url.post_to_db(&conn);
        assert_eq!(result, Ok(()));
        Url::fetch_from_db(&conn, "test_location_2".to_string())?.unwrap();
        Ok(())
    }

    #[test]
    fn test_bad_post_to_db() -> Result<()> {
        let (test_db, cleanup) = initialise_test_db();
        defer!(cleanup());

        let conn = Connection::open(test_db)?;
        // existing entry, can't post because of constraint
        let url = Url::new("test_location_1".to_string(), "test_target_1".to_string());
        if let Ok(_) = url.post_to_db(&conn) {
            panic!("should have returned an error");
        }
        Ok(())
    }
}
