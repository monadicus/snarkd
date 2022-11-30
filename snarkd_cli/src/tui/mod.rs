use self::util::cleanup_terminal;
use crate::tui::util::{setup_panic_hook, setup_terminal};
use anyhow::Result;
use clap::Args;
use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::EnterAlternateScreen,
};
use futures::FutureExt;
use snarkd_client::{PeerData, SnarkdClient};
use snarkd_common::config::Config;
use std::{
    collections::HashMap,
    io::{self, Write},
    net::SocketAddr,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Margin,
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Tabs},
    Terminal,
};

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

struct App {
    should_quit: bool,
    config: Config,
    client: SnarkdClient,
    data: Data,
}

#[derive(Clone)]
struct Data {
    peers: HashMap<SocketAddr, PeerData>,
}

pub async fn start(args: TuiArgs, config: Config, client: SnarkdClient) -> Result<()> {
    better_panic::install();

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    setup_panic_hook();
    setup_terminal();

    let app = App {
        should_quit: false,
        config,
        client,
        data: Data {
            peers: HashMap::new(),
        },
    };
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
        // terminal.draw(|f| ui::draw(f, &mut app))?;
        terminal.draw(|f| {
            f.render_widget(
                Tabs::new(vec![Spans::from(Span::styled("Peers", Style::default()))])
                    .block(Block::default().borders(Borders::ALL).title("Title")),
                f.size(),
            );
            let items: Vec<ListItem> = app
                .data
                .peers
                .clone()
                .iter()
                .map(|(k, _v)| {
                    ListItem::new(vec![Spans::from(Span::styled(
                        k.to_string(),
                        Style::default(),
                    ))])
                })
                .collect();
            f.render_widget(
                List::new(items),
                f.size().inner(&Margin {
                    vertical: 1,
                    horizontal: 1,
                }),
            )
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if let Some(Some(Ok(msg))) = subscription.next().now_or_never() {
            match msg {
                snarkd_client::PeerMessage::Handshake { address, peer } => {
                    app.data.peers.insert(address, peer);
                }
                snarkd_client::PeerMessage::Update { address, peer } => {
                    app.data.peers.insert(address, peer);
                }
                snarkd_client::PeerMessage::Disconnect(k) => {
                    app.data.peers.remove_entry(&k);
                }
                _ => {}
            }
        }

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('c' | 'd') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    // KeyCode::Char(c) => app.on_key(c),
                    // KeyCode::Left => app.on_left(),
                    // KeyCode::Up => app.on_up(),
                    // KeyCode::Right => app.on_right(),
                    // KeyCode::Down => app.on_down(),
                    _ => {}
                };
            }
        }
        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
