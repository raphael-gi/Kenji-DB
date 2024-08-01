use std::vec::IntoIter;
use lexer::{Token,TokenType};

use crate::{get_name, should_execute};
use crate::io::{create, Table, TableColumn};
use crate::errors::{err, err_semicolon, no_db};

pub fn create(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Option<String> {
    match tokens.next() {
        Some(token) => {
            match token.token_type {
                TokenType::DATABASE => create_database(tokens),
                TokenType::TABLE => match database {
                    Some(database) => create_table(tokens, database),
                    None => return no_db()
                },
                _ => return err("You may only create a database or table"),
            }
        },
        None => return err("Nothing to create provided")
    }
}

fn create_database(tokens: &mut IntoIter<Token>) -> Option<String> {
    let database_name = match get_name(tokens) {
        Ok(name) => name,
        Err(err) => return Some(err)
    };

    if should_execute(tokens.next()) {
        create::create_database(database_name);
        return None;
    }

    err_semicolon()
}

fn create_table(tokens: &mut IntoIter<Token>, database: &String) -> Option<String> {
    let table_name = match get_name(tokens) {
        Ok(name) => name,
        Err(err) => return Some(err)
    };

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
            _ => return None
        },
        None => return None
    };

    None
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

