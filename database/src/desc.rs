use std::vec::IntoIter;

use lexer::{Token, TokenType};

use crate::should_execute;
use crate::io::display;
use crate::errors::{err, err_semicolon, no_db};

pub fn desc(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Option<String> {
    let database = match database {
        Some(db) => db,
        None => return no_db()
    };
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::IDENTIFIER => {
                if should_execute(tokens.next()) {
                    display::desc_table(token.value.unwrap(), database);
                    return None;
                }
                err_semicolon()
            },
            _ => err("Invalid table name")
        },
        None => err("Nothing to describe provided")
    }
}
