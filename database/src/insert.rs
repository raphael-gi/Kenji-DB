use std::vec::IntoIter;
use lexer::{Token, TokenType};

use crate::{commands::{get_table_column_types, table_exists}, errors::{err, no_db}};

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

                            println!("{:?}", columns);
                            println!("{:?}", insert_values);

                            None
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

fn get_insert_values(tokens: &mut IntoIter<Token>) -> Result<Vec<Token>, ()> {
    let mut insert_values: Vec<Token> = Vec::new();
    loop {
        match tokens.next() {
            Some(token) => match token.token_type {
                TokenType::RIGHTBRACE => return Ok(insert_values),
                TokenType::COMMA => continue,
                TokenType::STR => insert_values.push(token),
                TokenType::INT => insert_values.push(token),
                _ => return Err(())
            }
            None => return Err(())
        };
    }
}

