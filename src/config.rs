use std::{path::PathBuf, net::{ToSocketAddrs, SocketAddr}};
use serde::Deserialize;
use figment::{Figment, providers::{Serialized, Toml, Format}};
use color_eyre::{Result, eyre::Context};
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mpd: MpdClientConfig,
}

#[derive(Deserialize, Debug)]
pub struct MpdClientConfig {
    pub host: String,
}

impl Config {
    #[instrument]
    pub fn parse(path: &PathBuf) -> Result<Config> {
        let config: Config = Figment::new()
            .merge(Toml::file(path))
            .extract()?;

        Ok(config)
    }
}
