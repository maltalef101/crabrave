use clap::Parser;
use std::path::PathBuf;
pub use color_eyre::Result;

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

/// Configuration.
pub mod config;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}
