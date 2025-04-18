use std::vec::IntoIter;

use lexer::{Token, TokenType};

use crate::should_execute;
use crate::io::display;
use crate::errors::{err_semicolon, DBError, DBErrorKind};

pub fn desc(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Result<(), DBError> {
    let db = database.clone().ok_or(DBError::new("test"))?;
    let token = tokens.next().ok_or(DBError::new("Nothing to describe provided"))?;

    match token.token_type {
        TokenType::IDENTIFIER => {
            if !should_execute(tokens.next()) {
                err_semicolon()?;
            }
            display::desc_table(token.value.unwrap(), &db);
            Ok(())
        },
        _ => Err(DBErrorKind::InvalidTableName.into())
    }
}
