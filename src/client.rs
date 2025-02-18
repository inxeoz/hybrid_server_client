use std::net::TcpStream;
use std::io::{self, Write, Read};

pub fn connect_server(ip: &str, port: &u16)  {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)) // Replace with your server's IP
        .expect("Failed to connect to server");

    println!("Connected to server!");

    loop {
        let mut input = String::new();
        print!("Enter message: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            break;
        }

        stream.write_all(input.as_bytes()).unwrap();

        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..size]);
        println!("Server replied: {}", response);
    }
}
