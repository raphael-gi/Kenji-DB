use std::{io::BufReader, net::{SocketAddrV4, TcpListener}, thread};

pub fn spawn_listener(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).unwrap();

    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let buff_reader = BufReader::new(stream);
            println!("{:?}", buff_reader);
        }
    });
}
