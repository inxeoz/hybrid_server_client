use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener as TokioTcpListener, TcpStream as TokioTcpStream}; // Use Tokio's Tcp types
use tokio::io::split;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wifi-cli")]
#[command(about = "A simple CLI tool to manage Wi-Fi using terminal commands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Route {
        one_end: String,
        second_end: String,
    },
}

/// Forwards the TCP connection and logs traffic.
async fn forward(mut inbound: TokioTcpStream, target_addr: String) {
    match TokioTcpStream::connect(target_addr.clone()).await {
        Ok(mut outbound) => {
            let (mut ri, mut wi) = split(inbound);
            let (mut ro, mut wo) = split(outbound);

            // Task: Forward and print client → ADB
            let client_to_adb = tokio::spawn(async move {
                let mut buffer = [0u8; 1024];
                while let Ok(n) = ri.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    let data = String::from_utf8_lossy(&buffer[..n]);
                    println!("[Client → ADB] {}", data); // Log data
                    if let Err(e) = wo.write_all(&buffer[..n]).await {
                        eprintln!("Error forwarding from client to ADB: {}", e);
                        break;
                    }
                }
            });

            // Task: Forward and print ADB → Client
            let adb_to_client = tokio::spawn(async move {
                let mut buffer = [0u8; 1024];
                while let Ok(n) = ro.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    let data = String::from_utf8_lossy(&buffer[..n]);
                    println!("[ADB → Client] {}", data); // Log data
                    if let Err(e) = wi.write_all(&buffer[..n]).await {
                        eprintln!("Error forwarding from ADB to client: {}", e);
                        break;
                    }
                }
            });

            let _ = tokio::try_join!(client_to_adb, adb_to_client);
        }
        Err(e) => {
            eprintln!("Failed to connect to ADB server at {}: {}", target_addr, e);
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Route { one_end, second_end } => {
            let listener = TokioTcpListener::bind(one_end.clone()).await?;
            println!("ADB proxy running: Listening on {} and forwarding to {}\n", one_end, second_end);

            while let Ok((inbound, _)) = listener.accept().await {
                let second_end_clone = second_end.clone();
                tokio::spawn(async move {
                    forward(inbound, second_end_clone).await;
                });
            }
        }
    }

    Ok(())
}
