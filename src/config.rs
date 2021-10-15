pub struct Config {
    pub db_path: String,
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
