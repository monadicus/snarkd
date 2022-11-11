use clap::{Parser, Subcommand};
use log::error;
use snarkd_client::SnarkdClient;
use url::Url;

/// Snarkd Blockchain Node
#[derive(Parser, Debug)]
#[command(author, version, about = "A CLI for interfacing with snarkd", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "ws://127.0.0.1:5422")]
    endpoint: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Foo,
    Bar { arg: String },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let endpoint_url = match args.endpoint.parse::<Url>() {
        Ok(e) => e,
        Err(e) => {
            error!("failed to parse endpoint url @ {}: {e:?}", args.endpoint);
            std::process::exit(1);
        }
    };

    let client = match SnarkdClient::new(endpoint_url.clone()).await {
        Ok(c) => c,
        Err(e) => {
            error!("failed to open client @ {}: {e:?}", args.endpoint);
            std::process::exit(1);
        }
    };

    match args.command {
        Commands::Foo => println!("output: {}", client.foo().await.expect("error running foo")),
        Commands::Bar { arg } => {
            println!(
                "output: {}",
                client.bar(arg).await.expect("error running foo")
            )
        }
    }
}
