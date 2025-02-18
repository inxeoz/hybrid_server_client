
mod client;
mod host;

use clap::{Parser, Subcommand};
use std::process::Command;

/// Wi-Fi Manager CLI
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Connect {
        ip: String,
        port: u16,
    },

    Host {
        ip: String,
        port: u16,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Connect { ip, port } => {
            client::connect_server(ip, port);
        },
        Commands::Host { port , ip} => {
            host::host_server(ip, port);
        }
    }
}

