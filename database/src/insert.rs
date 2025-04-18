use std::vec::IntoIter;
use lexer::{Token, TokenType};

use crate::errors::{DBError, DBErrorKind};
use crate::io::insert;
use crate::io::{get_table_column_types, table_exists};

pub fn insert(tokens: &mut IntoIter<Token>, database: &Option<String>) -> Result<(), DBError> {
    let db = database.as_ref().ok_or(DBErrorKind::NotUsingDB.into())?;
    let table_token = tokens.next().ok_or(DBErrorKind::InvalidTableName.into())?;

    if !matches!(table_token.token_type, TokenType::IDENTIFIER) {
        return Err(DBErrorKind::InvalidTableName.into());
    }
    let table_name = &table_token.value.unwrap();
    if !table_exists(db, table_name) {
        return Err(DBError::new("Table not found"));
    }

    handle_columns(tokens, table_name, db)
}

fn handle_columns(tokens: &mut IntoIter<Token>, table_name: &String, database: &String) -> Result<(), DBError> {
    let columns = get_table_column_types(table_name, &database);
    let brace_tocken = tokens.next().ok_or(DBErrorKind::ExpectedOpenBracket.into())?;

    if !matches!(brace_tocken.token_type, TokenType::LEFTBRACE) {
        return Err(DBErrorKind::ExpectedOpenBracket.into());
    }

    let values = get_insert_values(tokens)?;

    if columns.len() != values.len() {
        return Err(DBError::new(format!(
                    "Incorrect amount of parameters provided\nExpected: {} but found {}",
                    columns.len(), values.len()
                    )));
    }
    for (i, token) in values.iter().enumerate() {
        let token_type = token.token_type;
        if !TokenType::is_same_datatype(columns[i], token_type) {
            return Err(DBError::new(format!(
                        "Incorrect data type for column {}\nExpected {} but found {}",
                        i + 1, columns[i].to_string(), token_type.to_string()
                        )));
        }
    }

    let row = values.into_iter().map(|token| { token.value.unwrap() }).collect();
    println!("{:?}", row);
    let column_size = columns.iter().map(|column| { column.get_type_size() }).collect::<Vec<usize>>();
    println!("{:?}", column_size);

    insert::insert_table(table_name, database, row, column_size);

    Ok(())
}

fn get_insert_values(tokens: &mut IntoIter<Token>) -> Result<Vec<Token>, DBError> {
    let mut insert_values: Vec<Token> = Vec::new();
    loop {
        let token = tokens.next().ok_or(DBErrorKind::EndOfStatement.into())?;
        match token.token_type {
            TokenType::STR => insert_values.push(token),
            TokenType::INT => insert_values.push(token),
            _ => return Err(DBError::new("Couldn't identify inserted value"))
        }

        let next_token = tokens.next().ok_or(DBErrorKind::EndOfStatement.into())?;
        match next_token.token_type {
            TokenType::RIGHTBRACE => return Ok(insert_values),
            TokenType::COLON => continue,
            _ => return Err(DBError::new("You must seperate your values with a ','"))
        }
    }
}

