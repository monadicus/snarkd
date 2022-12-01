use crate::tui::TuiArgs;
use clap::{Parser, Subcommand};
use log::error;
use serde_json::json;
use snarkd_client::SnarkdClient;
use snarkd_common::config::load_config;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about = "A CLI for interfacing with snarkd", long_about = None)]
struct Args {
    #[arg(short, long)]
    endpoint: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum PeersCommands {
    Get,
    Listen,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Foo,
    Bar {
        arg: String,
    },
    Tui(TuiArgs),
    Metadata,
    #[command(subcommand)]
    Peers(PeersCommands),
}

mod tui;

#[tokio::main]
async fn main() {
    let config = load_config().unwrap_or_default();
    let args = Args::parse();

    let endpoint_url = args
        .endpoint
        .unwrap_or_else(|| format!("ws://127.0.0.1:{}", config.rpc_port));

    let endpoint_url = match endpoint_url.parse::<Url>() {
        Ok(e) => e,
        Err(e) => {
            error!("failed to parse endpoint url @ {}: {e:?}", endpoint_url);
            std::process::exit(1);
        }
    };

    let client = match SnarkdClient::new(endpoint_url.clone()).await {
        Ok(c) => c,
        Err(e) => {
            error!("failed to open client @ {}: {e:?}", endpoint_url);
            std::process::exit(1);
        }
    };

    match args.command {
        Commands::Foo => println!("{}", json!(client.foo().await.expect("error running foo"))),
        Commands::Bar { arg } => {
            println!(
                "{}",
                json!(client.bar(arg).await.expect("error running foo"))
            )
        }
        Commands::Tui(args) => tui::start(args, config, client)
            .await
            .expect("error running tui"),
        Commands::Metadata => {
            println!(
                "{}",
                json!(client.metadata().await.expect("error getting metadata"))
            )
        }
        Commands::Peers(command) => match command {
            PeersCommands::Get => {
                println!(
                    "{}",
                    json!(client.get_peers().await.expect("error listing peers"))
                )
            }
            PeersCommands::Listen => {
                let mut subscription = client
                    .subscribe_peers()
                    .await
                    .expect("error subscribing to peers");

                while let Some(Ok(msg)) = subscription.next().await {
                    println!("{}", json!(msg));
                }
            }
        },
    }
}
