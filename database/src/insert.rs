use std::vec::IntoIter;
use lexer::{Token, TokenType};

use crate::errors::no_db;

pub fn insert(tokens: &mut IntoIter<Token>, database: &Option<String>) {
    match database {
        Some(database) => {
            if let Some(table_token) = tokens.next() {
                if !matches!(table_token.token_type, TokenType::IDENTIFIER) {
                    return;
                }
                if let Some(brace_tocken) = tokens.next() {
                    if !matches!(brace_tocken.token_type, TokenType::LEFTBRACE) {
                        return;
                    }
                }
            }
        },
        None => no_db()
    }
}

