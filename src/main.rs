use std::{io::{Read, Write},net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpStream}};

use database::spawn_listener;
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() {
    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7878);
    spawn_listener(socket);

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let input = rl.readline(">> ");

        let mut stream = match TcpStream::connect(socket) {
            Ok(stream) => stream,
            Err(..) => continue
        };

        match input {
            Ok(input) => {
                let _ = stream.write(&input.into_bytes());
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }

        stream.shutdown(Shutdown::Write).expect("Shutdown failed");

        let mut buffer = Vec::new();
        match stream.read_to_end(&mut buffer) {
            Ok(..) => {
                if buffer.len() > 0 {
                    println!("{}", String::from_utf8(buffer).expect("Non utf8 characters provided"));
                }
            }
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
}

