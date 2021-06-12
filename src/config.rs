pub struct Config {
    db_path: String,
}

impl Config {
    pub fn new() -> Config {
        let db_path =
            std::env::var("URL_SHORTENER_DB_PATH").unwrap_or(String::from("url_shortener.db"));
        return Config {
            db_path: db_path.to_string(),
        };
    }
}

pub fn initialise_app(cfg: &Config) -> Result<(), String> {
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
