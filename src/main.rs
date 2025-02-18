mod host;
mod client;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::split;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Routes traffic from one port to another
    Host {
        #[arg(short = 'l', long = "listen", value_name = "PORT", help = "Port to listen on")]
        listen_port: u16,

        #[arg(short = 'i', long = "ip", value_name = "ip", help = "ip addr without port")]
        listen_ip: String,

    },

    Connect {
        #[arg(short = 'l', long = "listen", value_name = "PORT", help = "Port to listen on")]
        listen_port: u16,

        #[arg(short = 'i', long = "ip", value_name = "ip", help = "ip addr without port")]
        listen_ip: String,

    },
}


 fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Host { listen_port, listen_ip } => {
            host::main( &listen_ip, listen_port).expect(" host error")
        },
        Commands::Connect {listen_port, listen_ip, } => {
            client::main( &listen_ip, listen_port).expect(" client error ")
        }
    }

    Ok(())
}
