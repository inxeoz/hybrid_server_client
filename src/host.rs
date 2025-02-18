use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr.to_string(),
        Err(_) => "Unknown IP".to_string(),
    };

    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected", peer_addr);
                break;
            }
            Ok(size) => {
                let received_msg = String::from_utf8_lossy(&buffer[..size]);
                println!("Received from {}: {}", peer_addr, received_msg);

                // Echo back the message
                if let Err(e) = stream.write_all(received_msg.as_bytes()) {
                    eprintln!("Failed to send response to {}: {}", peer_addr, e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error with client {}: {}", peer_addr, e);
                break;
            }
        }
    }
}

pub fn host_server(ip_category: &str, port: &u16) {
    let listener = TcpListener::bind(format!("{}:{}", ip_category, port)).expect("Failed to bind port");

    println!("Server listening on port {port}...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");
                thread::spawn(|| handle_client(stream)); // Handle client in a new thread
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}