use std::vec::IntoIter;
use lexer::{Token,TokenType};

use crate::{commands::{self, Table, TableColumn}, errors::{err, err_semicolon, no_db}, get_name, should_execute};

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
        commands::create_database(database_name);
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
                commands::create_table(Table {
                    name: table_name,
                    database: database.to_string(),
                    rows: get_table_rows(tokens)
                });
            },
            TokenType::SEMICOLON => {
                commands::create_table(Table {
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
        // first value = pk || identifier
        let first_token = match tokens.next() {
            Some(token) => token,
            None => break
        };
        let is_pk = match first_token.token_type {
            TokenType::PK => true,
            _ => false
        };
        let name = match is_pk {
            true => match tokens.next() {
                Some(token) => match token.token_type {
                    TokenType::IDENTIFIER => token.value.unwrap(),
                    _ => break
                },
                None => break
            },
            false => first_token.value.unwrap()
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
                TokenType::SEMICOLON => rows.push(TableColumn {pk: is_pk, name, data_type }),
                _ => break
            },
            None => break
        };
    }

    rows
}


