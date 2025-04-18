use std::vec::IntoIter;
use lexer::{Token,TokenType};

use crate::{get_name, should_execute};
use crate::io::{create, Table, TableColumn};
use crate::errors::{err_semicolon, DBError, DBErrorKind};

pub fn create(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Result<(), DBError> {
    match tokens.next() {
        Some(token) => {
            match token.token_type {
                TokenType::DATABASE => create_database(tokens),
                TokenType::TABLE => match database {
                    Some(database) => create_table(tokens, database),
                    None => Err(DBErrorKind::NotUsingDB.into())
                },
                _ => Err(DBError::new("You may only create a database or table")),
            }
        },
        None => Err(DBError::new("Nothing to create provided"))
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

fn create_table(tokens: &mut IntoIter<Token>, database: &String) -> Result<(), DBError> {
    let table_name = get_name(tokens)?;

    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::LEFTBRACE => {
                create::create_table(Table {
                    name: table_name,
                    database: database.to_string(),
                    rows: get_table_rows(tokens)
                })
            },
            TokenType::SEMICOLON => {
                create::create_table(Table {
                    name: table_name,
                    database: database.to_string(),
                    rows: Vec::new()
                });
            },
            _ => return Ok(())
        },
        None => return Ok(())
    };

    Ok(())
}

fn get_table_rows(tokens: &mut IntoIter<Token>) -> Vec<TableColumn> {
    let mut rows: Vec<TableColumn> = Vec::new();

    loop {
        let first_token = match tokens.next() {
            Some(token) => token,
            None => break
        };
        let key = match first_token.token_type {
            TokenType::RIGHTBRACE => break,
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
            None => first_token.value.unwrap()
        };

        let data_type = match tokens.next() {
            Some(token) => match token.token_type {
                TokenType::STR => String::from("STR"),
                TokenType::INT => String::from("INT"),
                _ => break
            },
            None => break
        };

        match tokens.next() {
            Some(token) => match token.token_type {
                TokenType::SEMICOLON => rows.push(TableColumn {key, name, data_type }),
                _ => break
            },
            None => break
        };
    }

    rows
}

