use std::vec::IntoIter;

use lexer::{Token,TokenType};

use crate::{commands, should_execute};

pub fn create(tokens: &mut IntoIter<Token>, database: &Option<String>) {
    match tokens.next() {
        Some(token) => {
            match token.token_type {
                TokenType::DATABASE => create_database(tokens),
                TokenType::TABLE => match database {
                    Some(database) => create_table(tokens, database),
                    None => println!("Not using a database")
                },
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

fn create_table(tokens: &mut IntoIter<Token>, database: &String) {
    let table_name = match get_name(tokens) {
        Ok(name) => name,
        Err(..) => return
    };

    if should_execute(tokens.next()) {
        commands::create_table(table_name, database)
    }
}

fn get_name(tokens: &mut IntoIter<Token>) -> Result<String, ()> {
    let token = match tokens.next() {
        Some(token) => token,
        None => return Err(())
    };

    match token.token_type {
        TokenType::IDENTIFIER => Ok(token.value.unwrap()),
        _ => Err(())
    }
}

