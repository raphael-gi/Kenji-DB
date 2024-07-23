use std::vec::IntoIter;

use lexer::{Token,TokenType};

use crate::{commands, err, should_execute};

pub fn list(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Option<String> {
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::TABLES => match database {
                Some(database) => list_tables(tokens, database),
                None => return err("Use a database before listing tables")
            },
            TokenType::DATABASES => list_databases(tokens),
            _ => return err("Can't list what was provided")
        },
        None => return err("Nothing to list provided")
    }

    None
}

fn list_databases(tokens: &mut IntoIter<Token>) {
    if should_execute(tokens.next()) {
        commands::list_databases()
    }
}

fn list_tables(tokens: &mut IntoIter<Token>, database: &String) {
    if should_execute(tokens.next()) {
        commands::list_tables(database)
    }
}

