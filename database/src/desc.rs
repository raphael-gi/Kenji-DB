use std::vec::IntoIter;

use lexer::{Token, TokenType};

use crate::should_execute;
use crate::io::display;
use crate::errors::{err, err_semicolon};

pub fn desc(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Result<(), String> {
    let database = match database {
        Some(db) => db,
        None => return Err("test".to_string())
    };
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::IDENTIFIER => {
                if should_execute(tokens.next()) {
                    display::desc_table(token.value.unwrap(), database);
                    return Err("".to_string());
                }
                err_semicolon()
            },
            _ => err("Invalid table name")
        },
        None => err("Nothing to describe provided")
    };

    Ok(())
}
