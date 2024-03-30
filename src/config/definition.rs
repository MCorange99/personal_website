use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub debug: bool,
    pub webserver: ConfigWebserver,
    pub database: ConfigDatabase,
    pub redirect: Vec<ConfigRedirect>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigWebserver {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigDatabase {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigRedirect {
    pub from: String,
    pub to: String,
}