use core::panic;
use url_shortener::config;

mod server;

fn main() {
    let cfg = config::Config::new();
    initialise_app(&cfg).unwrap_or_else(|e| panic!("failed to initialise_app: {}", e));
    server::initialise_server();
    println!("Now listening on localhost:8000");
}

pub fn initialise_app(cfg: &config::Config) -> Result<(), String> {
    let conn = rusqlite::Connection::open(&cfg.db_path);
    if let Err(e) = conn {
        return Err(e.to_string());
    }
    let conn = conn.unwrap();

    if let Err(e) = conn.execute(
        "CREATE TABLE IF NOT EXISTS url (
                  location        TEXT PRIMARY KEY,
                  target          TEXT NOT NULL,
                  created_at      TEXT
                  )",
        [],
    ) {
        return Err(e.to_string());
    }
    Ok(())
}
