use std::borrow::{Borrow, BorrowMut};

use crate::cli::CliArgs;

pub mod definition;

const DEFAULT_CONFIG: &'static str = include_str!("../../config.default.toml");

pub struct ConfigManager {
    inner: definition::Config,
}

impl ConfigManager {
    pub fn parse_and_join(cli: &CliArgs) -> anyhow::Result<Self> {
        let data = if !cli.config.exists() {
            log::warn!("Config doesnt exist, making it.");
            std::fs::write(&cli.config, DEFAULT_CONFIG)?;
            DEFAULT_CONFIG.to_string()
        } else {
            std::fs::read_to_string(&cli.config)?
        };

        let mut inner = toml::from_str::<definition::Config>(&data)?;

        if let Some(host) = &cli.host {
            inner.webserver.host = host.clone();
        }

        if let Some(port) = cli.port {
            inner.webserver.port = port;
        }

        if cli.debug {
            inner.debug = true;
        }


        Ok(Self {
            inner
        })
    }

    pub fn get_ref(&self) -> &definition::Config {
        self.inner.borrow()
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self) -> &mut definition::Config {
        self.inner.borrow_mut()
    }
}



