use rusqlite::{Connection, Result};
use url_shortener::Url;


fn main() -> Result<()> {
    let conn = Connection::open("url_shortener.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS url (
                  location        TEXT PRIMARY KEY,
                  target          TEXT NOT NULL,
                  created_at      TEXT
                  )",
        [],
    )?;

   if let Ok(Some(url))  = Url::fetch_from_db(&conn, "some_location".to_string()) {
        println!("found url: {:?}", url)
   } else {
       println!("none found")
   }

    Ok(())
}