use std::net::{SocketAddr, ToSocketAddrs};

use color_eyre::Result;
use tracing::error;
use crate::config::Config;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// Owned MPD Client
    pub mpd_client: mpd::Client,
    /// MPD status
    pub mpd_status: mpd::Status,
    /// Reference to config obj
    pub config: &'a Config,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(config: &'a Config) -> Result<Self> {
        let mpd_host = &config.mpd.host;
        let mut mpd_client = mpd::Client::connect(mpd_host).map_err(|e| {
            error!("Failed while connecting to MPD at {mpd_host:?}: {e:?}");
            e
        })?;

        let mpd_status = mpd_client.status().map_err(|e| {
            error!("Failed while getting MPD status: {e:?}");
            e
        })?;

        Ok(Self {
            running: true,
            mpd_client,
            config,
            mpd_status,
        })
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        let mpd_status = self.mpd_client.status().map_err(|e| {
            error!("Failed while getting MPD status: {e:?}");
            e
        }).unwrap();

        self.mpd_status = mpd_status;
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    fn update(&mut self) {

    }
}
