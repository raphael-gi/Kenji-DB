use std::{io::{Read, Write}, net::{SocketAddrV4, TcpListener}, thread, vec::IntoIter};
use crate::io::database_exists;
use create::create;
use delete::delete;
use desc::desc;
use errors::{err,err_semicolon};
use insert::insert;
use lexer::{scan_tokens, Token, TokenType};
use show::show;

#[cfg(test)]
mod tests;

mod create;
mod delete;
mod insert;
mod show;
mod desc;
mod errors;
mod io;

pub fn spawn_listener(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).unwrap();

    let mut using_database: Option<String> = Some(String::from("store"));

    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let mut buffer = Vec::new();
            let _ = stream.read_to_end(&mut buffer);

            let mut tokens = scan_tokens(buffer).into_iter();

            let mut messages: Vec<String> = Vec::new();

            loop {
                let message = match tokens.next() {
                    Some(token) => match token.token_type {
                        TokenType::CREATE => create(&mut tokens, &using_database),
                        TokenType::DELETE => delete(&mut tokens, &using_database),
                        TokenType::INSERT => insert(&mut tokens, &using_database),
                        TokenType::SHOW => show(&mut tokens, &mut using_database),
                        TokenType::DESC => desc(&mut tokens, &mut using_database),
                        TokenType::USE => set_database(&mut tokens, &mut using_database),
                        _ => break
                    },
                    None => break
                };

                if let Some(message) = message {
                    messages.push(message);
                }
            }

            let _ = stream.write_all(messages.join(",").as_bytes()).unwrap();
        }
    });
}

fn set_database(tokens: &mut IntoIter<Token>, prev_db: &mut Option<String>) -> Option<String> {
    let database_name = match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::IDENTIFIER => token.value.unwrap(),
            _ => return err("Not a valid database name")
        },
        None => return err("Nothing to use provided")
    };

    if !database_exists(&database_name) {
        return err("Database not found");
    }

    if should_execute(tokens.next()) {
        *prev_db = Some(database_name.clone());
        return Some(format!("Using: {}", database_name));
    }

    err_semicolon()
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

fn get_name(tokens: &mut IntoIter<Token>) -> Result<String, String> {
    let token = match tokens.next() {
        Some(token) => token,
        None => return Err(String::from("No name provided"))
    };

    match token.token_type {
        TokenType::IDENTIFIER => Ok(token.value.unwrap()),
        _ => Err(String::from("Provided name isn't valid"))
    }
}

