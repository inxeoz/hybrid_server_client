use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::split;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Route {
        listen_port_a: String,
        forward_port_b: String,
    },
}

/// Forwards the TCP connection while logging data
async fn forward(mut inbound: TcpStream, target_addr: String, direction: &str) {
    match TcpStream::connect(&target_addr).await {
        Ok(mut outbound) => {
            let (mut ri, mut wi) = split(inbound);
            let (mut ro, mut wo) = split(outbound);

            let dir_clone = direction.to_string();

            // Task: Forward & log incoming → outgoing
            let forward_task = tokio::spawn(async move {
                let mut buffer = [0u8; 1024];
                while let Ok(n) = ri.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    println!("[{}] Received ({} bytes): {}", dir_clone, n, String::from_utf8_lossy(&buffer[..n]));

                    if let Err(e) = wo.write_all(&buffer[..n]).await {
                        eprintln!("[{}] Error forwarding: {}", dir_clone, e);
                        break;
                    }
                }
            });

            // Task: Forward & log outgoing → incoming
            let return_task = tokio::spawn(async move {
                let mut buffer = [0u8; 1024];
                while let Ok(n) = ro.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    println!("[{}] Sent ({} bytes): {}", dir_clone, n, String::from_utf8_lossy(&buffer[..n]));

                    if let Err(e) = wi.write_all(&buffer[..n]).await {
                        eprintln!("[{}] Error sending back: {}", dir_clone, e);
                        break;
                    }
                }
            });

            let _ = tokio::try_join!(forward_task, return_task);
        }
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", target_addr, e);
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Route { listen_port_a, forward_port_b } => {
            let listen_a = listen_port_a.clone();
            let forward_b = forward_port_b.clone();

            // Task 1: Listen on Port A and forward to Port B
            let listener_a = TcpListener::bind(&listen_port_a).await?;
            println!("Listening on {} → Forwarding to {}", listen_port_a, forward_port_b);

            let handle_a = tokio::spawn(async move {
                while let Ok((inbound, _)) = listener_a.accept().await {
                    let forward_b_clone = forward_b.clone();
                    tokio::spawn(async move {
                        forward(inbound, forward_b_clone, "A→B").await;
                    });
                }
            });

            // Task 2: Listen on Port B and send requests to Port A
            let listener_b = TcpListener::bind(&forward_port_b).await?;
            println!("Listening on {} → Sending to {}", forward_port_b, listen_port_a);

            let handle_b = tokio::spawn(async move {
                while let Ok((inbound, _)) = listener_b.accept().await {
                    let listen_a_clone = listen_a.clone();
                    tokio::spawn(async move {
                        forward(inbound, listen_a_clone, "B→A").await;
                    });
                }
            });

            let _ = tokio::try_join!(handle_a, handle_b);
        }
    }

    Ok(())
}
