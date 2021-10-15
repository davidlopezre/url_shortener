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

#[cfg(test)]
mod tests {
    use super::Config;
    use std::env;

    #[test]
    fn test_new_config_db_path_env_var() -> Result<(), String> {
        env::set_var("URL_SHORTENER_DB_PATH", "test_path.db");
        let cfg = Config::new();
        assert_eq!("test_path.db", cfg.db_path);
        Ok(())
    }

    #[test]
    fn test_new_config_default_db_path() -> Result<(), String> {
        let cfg = Config::new();
        assert_eq!("url_shortener.db", cfg.db_path);
        Ok(())
    }
}
