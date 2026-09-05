use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub server_addr: String
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be initialized"),
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string())

        }
    }
}