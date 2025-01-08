use std::vec::IntoIter;

use lexer::{Token,TokenType};

use crate::errors::{DBError, DBErrorKind};
use crate::should_execute;
use crate::io::display;

pub fn show(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Result<(), DBError> {
    let token = tokens.next().ok_or(DBErrorKind::NothingToShow.into())?;

    match token.token_type {
        TokenType::TABLES => {
            let db = database.as_ref().ok_or(DBError::new("Use a database before showing tables"))?;
            show_tables(tokens, db)
        },
        TokenType::DATABASES => show_databases(tokens, database),
        _ => Err(DBErrorKind::CantShow.into())
    }
}

fn show_databases(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Result<(), DBError> {
    if should_execute(tokens.next()) {
        display::show_databases(database);
        return Ok(());
    }

    Err(DBErrorKind::MissingSemicolon.into())
}

fn show_tables(tokens: &mut IntoIter<Token>, database: &String) -> Result<(), DBError> {
    if should_execute(tokens.next()) {
        display::show_tables(database);
        return Ok(());
    }

    Err(DBErrorKind::MissingSemicolon.into())
}

