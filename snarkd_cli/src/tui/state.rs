use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use futures::FutureExt;
use snarkd_client::{PeerData, PeerMessage, SnarkdClient, Subscription};
use snarkd_common::config::Config;

pub struct App {
    pub should_quit: bool,
    pub config: Config,
    pub client: SnarkdClient,
    pub data: Data,
}

#[derive(Clone)]
pub struct Data {
    pub peers: HashMap<SocketAddr, PeerData>,
}

impl App {
    pub fn new(config: Config, client: SnarkdClient) -> Self {
        Self {
            config,
            client,
            should_quit: false,
            data: Data {
                peers: HashMap::new(),
            },
        }
    }

    pub fn handle_peer_subscription(&mut self, subscription: &mut Subscription<PeerMessage>) {
        if let Some(Some(Ok(msg))) = subscription.next().now_or_never() {
            match msg {
                PeerMessage::Handshake { address, peer } => {
                    self.data.peers.insert(address, peer);
                }
                PeerMessage::Update { address, peer } => {
                    self.data.peers.insert(address, peer);
                }
                PeerMessage::Disconnect(k) => {
                    self.data.peers.remove_entry(&k);
                }
                _ => {}
            }
        }
    }

    pub fn handle_input(&mut self, tick_rate: Duration, last_tick: &mut Instant) -> Result<()> {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('c' | 'd') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.should_quit = true;
                    }
                    KeyCode::Char('q') => {
                        self.should_quit = true;
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
            *last_tick = Instant::now();
        }

        Ok(())
    }
}
