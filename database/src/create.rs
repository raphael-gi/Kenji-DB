use std::vec::IntoIter;

use lexer::{Token,TokenType};

use crate::{commands, should_execute};

pub fn create(tokens: &mut IntoIter<Token>) {
    match tokens.next() {
        Some(token) => {
            match token.token_type {
                TokenType::DATABASE => create_database(tokens),
                TokenType::TABLE => {},
                _ => {},
            }
        },
        None => println!("Nothing to create provided")
    }
}

fn create_database(tokens: &mut IntoIter<Token>) {
    let token = match tokens.next() {
        Some(token) => token,
        None => return
    };

    let database_name = match token.token_type {
        TokenType::IDENTIFIER => token.value.unwrap(),
        _ => return
    };

    if should_execute(tokens.next()) {
        commands::create_database(database_name);
    }
}

