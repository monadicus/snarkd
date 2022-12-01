use self::{
    state::App,
    util::{cleanup_terminal, setup_panic_hook, setup_terminal},
};
use anyhow::Result;
use clap::Args;
use crossterm::{event::EnableMouseCapture, execute, terminal::EnterAlternateScreen};
use snarkd_client::SnarkdClient;
use snarkd_common::config::Config;
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod state;
mod ui;
mod util;

#[derive(Debug, Args)]
pub struct TuiArgs {
    #[arg(short, long)]
    foo: Option<String>,

    #[arg(short, long, default_value = "true")]
    enhanced_graphics: bool,

    #[arg(short, long, default_value = "32")]
    tick_rate: u64,
}

pub async fn start(args: TuiArgs, config: Config, client: SnarkdClient) -> Result<()> {
    better_panic::install();

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    setup_panic_hook();
    setup_terminal();

    let app = App::new(config, client);

    run_app(&mut terminal, app, Duration::from_millis(args.tick_rate)).await?;

    cleanup_terminal();

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();

    app.data.peers = app.client.get_peers().await?;
    let mut subscription = app.client.subscribe_peers().await?;

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        app.handle_peer_subscription(&mut subscription);
        app.handle_input(tick_rate, &mut last_tick)?;

        if app.should_quit {
            return Ok(());
        }
    }
}
