use std::{io::Read, net::{SocketAddrV4, TcpListener}, thread};

pub fn spawn_listener(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).unwrap();

    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = Vec::new();
            let _ = stream.read_to_end(&mut buffer);
            let input = match String::from_utf8(buffer.to_vec()) {
                Ok(input) => input,
                Err(..) => continue
            };
            println!("{:?}", input);
        }
    });
}
