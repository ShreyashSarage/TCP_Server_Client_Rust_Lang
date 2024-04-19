use std::thread;
use std::net::TcpListener;
mod handle_client;



fn main() {
    let listener: TcpListener = match TcpListener::bind("0.0.0.0:3333"){
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // accept connections and processs them, spwaning a new thread for each one 

    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
        Ok(stream) => {
            println!("New Connection: {}", stream.peer_addr().unwrap());
            thread::spawn(move ||{
                // connection succeeded
                handle_client::handle_client(stream);
            });
        }
            Err(e) => {
                eprintln!("Error: {}", e);
                /* Connection Failed */
            }
        }
        }
        drop(listener);
    }
