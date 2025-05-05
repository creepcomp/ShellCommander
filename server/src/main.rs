use std::io::{Read, Result, Write, stdin};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const LISTEN_ADDRESS: &str = "0.0.0.0:443";

fn handle_incoming(listener: &TcpListener, clients: Arc<Mutex<Vec<TcpStream>>>) -> Result<()> {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected: {}", stream.peer_addr()?);
                match clients.lock() {
                    Ok(mut c) => c.push(stream),
                    Err(e) => println!("Error: {e}"),
                }
            }
            Err(e) => eprintln!("Failed to accept client: {}", e),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let clients = Arc::new(Mutex::new(Vec::new()));
    let mut selected: Option<usize> = None;
    let listener = TcpListener::bind(LISTEN_ADDRESS)?;
    println!("Server listening on {LISTEN_ADDRESS}...");

    let _clients = Arc::clone(&clients);
    thread::spawn(move || handle_incoming(&listener, _clients));

    loop {
        let mut command = String::new();
        stdin().read_line(&mut command)?;

        match command.trim() {
            "list" => match clients.lock() {
                Ok(c) => {
                    for (index, client) in c.iter().enumerate() {
                        println!("[{index}] {}", client.peer_addr()?)
                    }
                }
                Err(e) => eprintln!("Error: {e}"),
            },
            c if c.starts_with("select") => match c.split_whitespace().nth(1) {
                Some(s) => match s.parse::<usize>() {
                    Ok(i) => match clients.lock() {
                        Ok(c) => {
                            if i < c.len() {
                                let client = &c[i];
                                selected = Some(i);
                                println!("{} has been selected.", client.peer_addr()?);
                            } else {
                                eprintln!("Error: No client exists at index {}.", i);
                            }
                        }
                        Err(e) => eprintln!("Error: {e}"),
                    },
                    Err(e) => eprintln!("Error: {e}"),
                },
                None => eprintln!("Error: No index provided."),
            },
            "help" => {
                println!("
list: Get a list of connections.
select: Select a connection.
help: show help.
quit: quit/kill the process.
")
            }
            "quit" => break,
            _ => match clients.lock() {
                Ok(c) => match selected {
                    Some(i) => {
                        let mut client = &c[i];
                        client.write_all(command.as_bytes())?;
                        let mut output = Vec::new();
                        loop {
                            let mut b = [0; 256];
                            let n = client.read(&mut b)?;
                            for i in 0..n {
                                output.push(b[i]);
                            }
                            if n < 256 {
                                break;
                            }
                        }
                        match String::from_utf8(output) {
                            Ok(output) => println!("{output}"),
                            Err(e) => eprintln!("Error: {e}"),
                        }
                    }
                    None => println!("No client selected. (use help command)"),
                },
                Err(e) => eprintln!("Error: {e}"),
            },
        };
    }

    println!("Server shutting down.");
    Ok(())
}
