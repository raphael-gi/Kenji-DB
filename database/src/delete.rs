use std::vec::IntoIter;

use lexer::{Token,TokenType};

use crate::{commands, errors::{err, err_semicolon, no_db}, get_name, should_execute};

pub fn delete(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Option<String> {
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::DATABASE => delete_database(tokens),
            TokenType::TABLE => match database {
                Some(database) => delete_table(tokens, database),
                None => return no_db()
            },
            _ => return err("You may only delete a database or table")
        },
        None => return err("Nothing too delete")
    }
}

fn delete_database(tokens: &mut IntoIter<Token>) -> Option<String> {
    let database_name = match get_name(tokens) {
        Ok(name) => name,
        Err(err) => return Some(err)
    };

    if should_execute(tokens.next()) {
        commands::delete_database(database_name);
        return None;
    }

    err_semicolon()
}

fn delete_table(tokens: &mut IntoIter<Token>, database: &String) -> Option<String> {
    let table_name = match get_name(tokens) {
        Ok(name) => name,
        Err(..) => return None
    };

    if should_execute(tokens.next()) {
        commands::delete_table(table_name, database);
        return None;
    }

    err_semicolon()
}

