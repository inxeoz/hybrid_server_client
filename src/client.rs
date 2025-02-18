use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

pub fn main(ip:&str, port: u16) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;
    println!("Connected to server!");

    let mut buffer = [0; 1024];

    loop {
        // Send a message to the server
        let message = "Hello from Client!";
        stream.write_all(message.as_bytes())?;
        println!("Sent: {}", message);

        // Wait for a response
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                println!("Server replied: {}", String::from_utf8_lossy(&buffer[..n]));
            }
            Ok(_) => {
                println!("Server disconnected.");
                break;
            }
            Err(e) => {
                eprintln!("Error reading from server: {}", e);
                break;
            }
        }

        thread::sleep(Duration::from_secs(3)); // Wait before sending the next message
    }

    Ok(())
}
