use std::{io::Read, net::{SocketAddrV4, TcpListener}, thread};
use create::create;
use delete::delete;
use lexer::{scan_tokens, Token, TokenType};

mod create;
mod delete;
mod commands;

pub fn spawn_listener(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).unwrap();

    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = Vec::new();
            let _ = stream.read_to_end(&mut buffer);

            let mut tokens = scan_tokens(buffer).into_iter();

            loop {
                match tokens.next() {
                    Some(token) => match token.token_type {
                        TokenType::CREATE => create(&mut tokens),
                        TokenType::DELETE => delete(&mut tokens),
                        _ => break
                    },
                    None => break
                }
            }
        }
    });
}

fn should_execute(token: Option<Token>) -> bool {
    match token {
        Some(token) => match token.token_type {
            TokenType::SEMICOLON => true,
            _ => false
        },
        None => false
    }
}

