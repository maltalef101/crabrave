use crabrave::app::App;
use crabrave::event::{Event, EventHandler};
use crabrave::handler::handle_key_events;
use crabrave::tui::Tui;
use crabrave::config::Config;
use crabrave::Args;
use color_eyre::Result;
use clap::Parser;
use std::{io, fs, env};
use std::path::PathBuf;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tracing::{error, info, debug};

fn main() -> Result<()> {
    let args = Args::parse();

    let config_path: PathBuf = if let Some(path) = args.config {
        path.try_exists().expect("Cannot check existence of file {path:?}");
        path
    } else {
        let mut env_home = env::var("HOME").map_err(|e| {
            error!("Failed while evaluating environment variable: {e:?}");
            e
        })?;
        env_home.push('/');

        let env_config_home = if let Ok(v) = env::var("XDG_CONFIG_HOME") {
            v
        } else {
            ".config/".into()
        };

        let mut path: String = Default::default();
        path.push_str(&env_home);
        path.push_str(&env_config_home);
        path.push_str("crabrave");
        path.push_str("config.toml");
        path.into()
    };

    let config: Config = Config::parse(&config_path).map_err(|e| {
        error!("Failed to parse config: {e:?}");
        e
    })?;

    info!("Loaded config at {config_path:?}");

    let mut app = App::new(&config).map_err(|e| {
        error!("Failed to create application: {e:?}");
        e
    })?;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
