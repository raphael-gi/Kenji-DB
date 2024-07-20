use std::{io::Read, net::{SocketAddrV4, TcpListener}, thread, vec::IntoIter};
use create::create;
use delete::delete;
use lexer::{scan_tokens, Token, TokenType};

mod create;
mod delete;
mod commands;

pub fn spawn_listener(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).unwrap();

    let mut using_database: Option<String> = None;

    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = Vec::new();
            let _ = stream.read_to_end(&mut buffer);

            let mut tokens = scan_tokens(buffer).into_iter();

            loop {
                match tokens.next() {
                    Some(token) => match token.token_type {
                        TokenType::CREATE => create(&mut tokens, &using_database),
                        TokenType::DELETE => delete(&mut tokens),
                        TokenType::USE => set_database(&mut tokens, &mut using_database),
                        _ => break
                    },
                    None => break
                }
            }
        }
    });
}

fn set_database(tokens: &mut IntoIter<Token>, prev_db: &mut Option<String>) {
    let database_name = match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::IDENTIFIER => token.value.unwrap(),
            _ => return
        },
        None => return
    };

    if should_execute(tokens.next()) {
        *prev_db = Some(database_name);
    }
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

