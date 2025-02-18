use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }
            Ok(n) => {
                let received_msg = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", received_msg);

                // Send a reply
                let response = "Hello from Server!";
                stream.write_all(response.as_bytes()).unwrap();
                println!("Reply sent!");
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

pub fn main(ip:&str, port: u16) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", ip, port) )?;
    println!("Server listening on port {port}...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");
                thread::spawn(move || handle_client(stream)); // Handle each client in a new thread
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
