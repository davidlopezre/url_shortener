use core::panic;
use url_shortener::{config, error::Error};

mod server;

fn main() {
    let cfg = config::Config::new();
    initialise_app(&cfg).unwrap_or_else(|e| panic!("failed to initialise_app: {}", e));
    server::initialise_server();
}

pub fn initialise_app(cfg: &config::Config) -> Result<(), Error> {
    let conn = rusqlite::Connection::open(&cfg.db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS url (
                  location        TEXT PRIMARY KEY,
                  target          TEXT NOT NULL,
                  created_at      TEXT
                  )",
        [],
    )?;
    Ok(())
}
