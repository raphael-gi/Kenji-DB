use std::vec::IntoIter;
use lexer::{Token,TokenType};

use crate::{get_name, should_execute};
use crate::io::{create, Table, TableColumn};
use crate::errors::{err_invalid_datatype, err_not_interpreted, err_semicolon, DBError, DBErrorKind};

pub fn create(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Result<(), DBError> {
    let token = tokens.next().ok_or(DBError::new("Nothing to create provided"))?;
    match token.token_type {
        TokenType::DATABASE => create_database(tokens),
        TokenType::TABLE => {
            let db = database.clone().ok_or(DBErrorKind::NotUsingDB.into())?;
            create_table(tokens, db)
        },
        _ => Err(DBError::new("You may only create a database or table")),
    }
}

fn create_database(tokens: &mut IntoIter<Token>) -> Result<(), DBError> {
    let database_name = get_name(tokens)?;

    if !should_execute(tokens.next()) {
        err_semicolon()?
    }

    create::create_database(database_name);
    return Ok(());
}

fn create_table(tokens: &mut IntoIter<Token>, database: String) -> Result<(), DBError> {
    let name = get_name(tokens)?;
    let rows = get_table_rows(tokens)?;

    create::create_table(Table { name, database, rows });
    Ok(())
}

fn get_table_rows(tokens: &mut IntoIter<Token>) -> Result<Vec<TableColumn>, DBError> {
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::LEFTBRACE => {},
            TokenType::SEMICOLON => {
                return Ok(Vec::new());
            },
            _ => return Err(err_not_interpreted(token))
        },
        None => return Err(DBErrorKind::MissingSemicolon.into())
    };
    let mut rows: Vec<TableColumn> = Vec::new();

    loop {
        let first_token = tokens.next().ok_or(DBErrorKind::EndOfStatement.into())?;

        let key = match first_token.token_type {
            TokenType::PK => Some(TokenType::PK),
            TokenType::FK => Some(TokenType::FK),
            _ => None
        };
        let name = match key {
            Some(..) => match tokens.next() {
                Some(token) => match token.token_type {
                    TokenType::IDENTIFIER => token.value.unwrap(),
                    _ => break
                },
                None => break
            },
            None => first_token.value.ok_or(DBError::new("Invalid column name"))?
        };

        let data_type = match tokens.next() {
            Some(token) => match token.token_type {
                TokenType::STR => String::from("STR"),
                TokenType::INT => String::from("INT"),
                _ => return Err(err_invalid_datatype(token))
            },
            None => return Err(DBError::new("No datatype provided"))
        };

        let table_column = TableColumn { key, name, data_type };

        match tokens.next() {
            Some(token) => match token.token_type {
                TokenType::RIGHTBRACE => {
                    rows.push(table_column);
                    break;
                },
                TokenType::COLON => rows.push(table_column),
                _ => return Err(DBError::new(format!("Unexpected token: {:?}", token.token_type)))
            },
            None => return Err(DBErrorKind::EndOfStatement.into())
        };
    }

    if !should_execute(tokens.next()) {
        err_semicolon()?
    }

    Ok(rows)
}

