use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub debug: bool,
    pub webserver: ConfigWebserver,
    pub database: ConfigDatabase
}

#[derive(Debug, Deserialize)]
pub struct ConfigWebserver {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ConfigDatabase {
    pub url: String,
}