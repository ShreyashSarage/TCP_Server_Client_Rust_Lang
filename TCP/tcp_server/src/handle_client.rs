use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // Connection closed by client
                    println!("Client disconnected");
                    break;
                }

                // Process received message
                let received_message = str::from_utf8(&buffer[..bytes_read]).unwrap();
                println!("Received message from client: {}", received_message);

                // Check if client wants to exit
                if received_message.trim() == "exit" {
                    println!("Client requested to exit");
                    break;
                }

                // Send acknowledgment back to client
                let acknowledgment = "Message received successfully";
                stream.write_all(acknowledgment.as_bytes()).unwrap();
                println!("Sent acknowledgment to client");

                // Clear buffer for next message
                buffer = [0; 1024];
            }
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}
