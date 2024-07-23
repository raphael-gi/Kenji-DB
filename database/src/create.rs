use std::vec::IntoIter;
use lexer::{Token,TokenType};

use crate::{commands::{self, Table, TableColumn}, errors::no_db, get_name, should_execute};

pub fn create(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Option<String> {
    match tokens.next() {
        Some(token) => {
            match token.token_type {
                TokenType::DATABASE => create_database(tokens),
                TokenType::TABLE => match database {
                    Some(database) => create_table(tokens, database),
                    None => no_db()
                },
                _ => {},
            }
        },
        None => return Some("Nothing to create provided".to_string())
    }

    None
}

fn create_database(tokens: &mut IntoIter<Token>) {
    let token = match tokens.next() {
        Some(token) => token,
        None => return
    };

    let database_name = match token.token_type {
        TokenType::IDENTIFIER => token.value.unwrap(),
        _ => return
    };

    if should_execute(tokens.next()) {
        commands::create_database(database_name);
    }
}

fn create_table(tokens: &mut IntoIter<Token>, database: &String) {
    let table_name = match get_name(tokens) {
        Ok(name) => name,
        Err(..) => return
    };

    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::LEFTBRACE => {
                commands::create_table(Table {
                    name: table_name,
                    database: database.to_string(),
                    rows: get_table_rows(tokens)
                })
            },
            TokenType::SEMICOLON => {
                commands::create_table(Table {
                    name: table_name,
                    database: database.to_string(),
                    rows: Vec::new()
                })
            },
            _ => return
        },
        None => return
    }

}

fn get_table_rows(tokens: &mut IntoIter<Token>) -> Vec<TableColumn> {
    let mut rows: Vec<TableColumn> = Vec::new();

    loop {
        let name = match tokens.next() {
            Some(token) => match token.token_type {
                TokenType::IDENTIFIER => token.value.unwrap(),
                _ => break
            },
            None => break
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
                TokenType::SEMICOLON => rows.push(TableColumn { name, data_type }),
                _ => break
            },
            None => break
        };
    }

    rows
}


