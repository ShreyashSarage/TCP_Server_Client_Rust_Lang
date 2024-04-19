use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::str::from_utf8;
use serde_json::Value;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            eprintln!("No arguments are provided. No idea on what file to read.");
            return;
        }
        2 => {}
        _ => {
            eprintln!("Why provide extra arguments when work can be done with only one");
            return;
        }
    }

    let file_path = &args[1];
    // Read the contents of the json file
    let config_contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading config.json: {}", e);
            return;
        }
    };

    // Parse the JSON contents
    let config_json: Value = match serde_json::from_str(&config_contents) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing config.json: {}", e);
            return;
        }
    };

    // Extract the file_path from the JSON
    let addr = match config_json["addr"].as_str() {
        Some(path) => path,
        None => {
            eprintln!("No file_path specified in config.json");
            return;
        }
    };

    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");


            loop {
                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read user input");

                // Trim whitespace and convert the user input to bytes
                let user_input = user_input.trim();
                let msg = user_input.as_bytes();

                match stream.write(msg) {
                    Ok(_) => println!("Successfully wrote user input to server"),
                    Err(e) => {
                        println!("Failed to write user input to server: {}", e);
                        break; // Exit loop on write error
                    }
                };

                if user_input.eq_ignore_ascii_case("exit") {
                    println!("Exiting.");
                    break;
                }

                println!("Sent user input to server: {}", user_input);

                let mut response_data = [0 as u8; 1024]; // using 1024 byte buffer for response
                match stream.read(&mut response_data) {
                    Ok(size) => {
                        let received_message =
                            from_utf8(&response_data[..size]).unwrap_or("Invalid UTF-8 data");
                        println!("Received message from server: {}", received_message);
                    }
                    Err(e) => {
                        println!("Failed to receive data from server: {}", e);
                        break; // Exit loop on read error
                    }
                }
                
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}