use std::vec::IntoIter;

use lexer::{Token,TokenType};

use crate::{get_name, should_execute};
use crate::io::delete;
use crate::errors::{err_semicolon, DBError, DBErrorKind};

pub fn delete(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Result<(), DBError> {
    let token = tokens.next().ok_or(DBError::new("Nothing too delete"))?;
    match token.token_type {
        TokenType::DATABASE => delete_database(tokens),
        TokenType::TABLE => match database {
            Some(database) => delete_table(tokens, database),
            None => return Err(DBErrorKind::NotUsingDB.into())
        },
        _ => return Err(DBError::new("You may only delete a database or table"))
    }
}

fn delete_database(tokens: &mut IntoIter<Token>) -> Result<(), DBError> {
    let database_name = get_name(tokens)?;

    if !should_execute(tokens.next()) {
        err_semicolon()?;
    }

    delete::delete_database(database_name);
    Ok(())
}

fn delete_table(tokens: &mut IntoIter<Token>, database: &String) -> Result<(), DBError> {
    let table_name = get_name(tokens)?;

    if !should_execute(tokens.next()) {
        err_semicolon()?;
    }

    delete::delete_table(table_name, database);
    Ok(())
}

