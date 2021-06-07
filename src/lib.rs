use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Url {
    location: String,
    target: String,
    created_at: Option<String>,
}

impl Url {
    pub fn new(location: String, target: String) -> Url {
        let created_at = Some("".to_string());
        Url {
            target,
            location,
            created_at,
        }
    }

    pub fn fetch_from_db(connection: Connection, location: String) -> Result<Option<Url>> {
        let mut stmt = connection.prepare("SELECT location, target, created_at FROM url WHERE location=?")?;

        let mut url_iter = stmt.query_map([location], |row| {
            Ok(Url::new(row.get(0)?, row.get(1)?))
        })?;

       url_iter.next().transpose()
    }
}
