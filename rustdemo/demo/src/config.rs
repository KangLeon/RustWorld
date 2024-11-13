use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenv::dotenv().ok();

        Ok(Config {
            database_url: std::env::var("DATABASE_URL")?,
            server_host: std::env::var("SERVER_HOST")?,
            server_port: std::env::var("SERVER_PORT")?.parse()?,
        })
    }
}
