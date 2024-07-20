use std::vec::IntoIter;

use lexer::{Token,TokenType};

use crate::commands;

pub fn delete(tokens: &mut IntoIter<Token>) {
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::DATABASE => delete_database(tokens),
            TokenType::TABLE => {},
            _ => {}
        },
        None => print!("Nothing too delete")
    }
}

fn delete_database(tokens: &mut IntoIter<Token>) {
    let token = match tokens.next() {
        Some(token) => token,
        None => return
    };

    let database_name = match token.token_type {
        TokenType::IDENTIFIER => token.value.unwrap(),
        _ => return
    };

    commands::delete_database(database_name)
}

