use std::vec::IntoIter;

use lexer::{Token,TokenType};

use crate::{err, should_execute};
use crate::io::display;

pub fn show(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Option<String> {
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::TABLES => match database {
                Some(database) => show_tables(tokens, database),
                None => return err("Use a database before showing tables")
            },
            TokenType::DATABASES => show_databases(tokens, database),
            _ => return err("Can't show what was provided")
        },
        None => return err("Nothing to show provided")
    }
}

fn show_databases(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Option<String> {
    if should_execute(tokens.next()) {
        display::show_databases(database);
        return None;
    }

    err("Missing ';'")
}

fn show_tables(tokens: &mut IntoIter<Token>, database: &String) -> Option<String> {
    if should_execute(tokens.next()) {
        display::show_tables(database);
        return None;
    }
    err("Missing ';'")
}

