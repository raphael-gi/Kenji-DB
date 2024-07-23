use std::vec::IntoIter;

use lexer::{Token,TokenType};

use crate::{commands, err, should_execute};

pub fn show(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Option<String> {
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::TABLES => match database {
                Some(database) => show_tables(tokens, database),
                None => return err("Use a database before showing tables")
            },
            TokenType::DATABASES => show_databases(tokens),
            _ => return err("Can't show what was provided")
        },
        None => return err("Nothing to show provided")
    }
}

fn show_databases(tokens: &mut IntoIter<Token>) -> Option<String> {
    if should_execute(tokens.next()) {
        commands::show_databases();
        return None;
    }

    err("Missing ';'")
}

fn show_tables(tokens: &mut IntoIter<Token>, database: &String) -> Option<String> {
    if should_execute(tokens.next()) {
        commands::show_tables(database);
        return None;
    }
    err("Missing ';'")
}

