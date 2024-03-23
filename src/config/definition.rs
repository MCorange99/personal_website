use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub debug: bool,
    pub webserver: ConfigWebserver
}

#[derive(Debug, Deserialize)]
pub struct ConfigWebserver {
    pub host: String,
    pub port: u16,
}