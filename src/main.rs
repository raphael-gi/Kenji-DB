use std::{io::{stdin, Write}, net::{Ipv4Addr, SocketAddrV4, TcpStream}};

use database::spawn_listener;

fn main() {
    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7878);
    spawn_listener(socket);

    loop {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        let mut stream = TcpStream::connect(socket).unwrap();
        let _ = stream.write(&input.into_bytes());
    }
}

