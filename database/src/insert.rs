use std::vec::IntoIter;
use lexer::{Token, TokenType};

use crate::{commands::{get_table_column_types, table_exists}, errors::{err, err_abrupt_ending, no_db}};

pub fn insert(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Option<String> {
    match database {
        Some(database) => {
            match tokens.next() {
                Some(table_token) => {
                    if !matches!(table_token.token_type, TokenType::IDENTIFIER) {
                        return err("Invalid table name provided");
                    }
                    let table_name = &table_token.value.unwrap();
                    if !table_exists(database, table_name) {
                        return err("Table not found");
                    }
                    let columns = get_table_column_types(table_name, database);

                    match tokens.next() {
                        Some(brace_tocken) => {
                            if !matches!(brace_tocken.token_type, TokenType::LEFTBRACE) {
                                return err("Expected '(' but nothing provided");
                            }

                            let insert_values = get_insert_values(tokens);

                            match insert_values {
                                Ok(values) => {
                                    if columns.len() != values.len() {
                                        return Some(format!("Incorrect amount of parameters provided\nExpected: {} but found {}", columns.len(), values.len()));
                                    }
                                    for (i, token) in values.iter().enumerate() {
                                        let token_type = token.token_type;
                                        if matches!(columns[i], token_type) {

                                        }
                                    }
                                    println!("{:?}", columns);
                                    println!("{:?}", values);

                                    None
                                },
                                Err(err) => err
                            }
                        },
                        None => err("Expected '(' but nothing provided")
                    }
                },
                None => err("Invalid table name provided")
            }
        },
        None => no_db()
    }
}

fn get_insert_values(tokens: &mut IntoIter<Token>) -> Result<Vec<Token>, Option<String>> {
    let mut insert_values: Vec<Token> = Vec::new();
    loop {
        match tokens.next() {
            Some(token) => match token.token_type {
                TokenType::STR => insert_values.push(token),
                TokenType::INT => insert_values.push(token),
                _ => return Err(err("Couldn't identify inserted value"))
            }
            None => return Err(err_abrupt_ending())
        };
        match tokens.next() {
            Some(token) => match token.token_type {
                TokenType::RIGHTBRACE => return Ok(insert_values),
                TokenType::COMMA => continue,
                _ => return Err(err("You must seperate your values with a ','"))
            },
            None => return Err(err_abrupt_ending())
        }
    }
}

