use std::vec::IntoIter;

use lexer::{Token,TokenType};

use crate::{commands, get_name, should_execute};

pub fn delete(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Option<String> {
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::DATABASE => delete_database(tokens),
            TokenType::TABLE => match database {
                Some(database) => delete_table(tokens, database),
                None => return Some("Not using a database".to_string())
            },
            _ => {}
        },
        None => return Some("Nothing too delete".to_string())
    }

    None
}

fn delete_database(tokens: &mut IntoIter<Token>) {
    let database_name = match get_name(tokens) {
        Ok(name) => name,
        Err(..) => return
    };

    if should_execute(tokens.next()) {
        commands::delete_database(database_name)
    }
}

fn delete_table(tokens: &mut IntoIter<Token>, database: &String) {
    let table_name = match get_name(tokens) {
        Ok(name) => name,
        Err(..) => return
    };

    if should_execute(tokens.next()) {
        commands::delete_table(table_name, database)
    }
}

