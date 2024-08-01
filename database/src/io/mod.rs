use std::{fs::read, path::Path};

use lexer::TokenType;
use util::{get_db_path, get_table_config_path};

pub mod create;
pub mod delete;
pub mod insert;
pub mod display;
mod util;

pub struct TableColumn {
    pub key: Option<TokenType>,
    pub name: String,
    pub data_type: String
}

pub struct Table {
    pub name: String,
    pub database: String,
    pub rows: Vec<TableColumn>
}

impl Table {
    pub fn get_row_string(&self) -> String {
        self.rows.iter().map(|column| {
            let key = match column.key {
                Some(key) => key.to_string(),
                None => String::new()
            };
            format!("{},{},{}", key, column.name, column.data_type)
        }).collect::<Vec<String>>().join(";")
    }
}


pub fn database_exists(database: &String) -> bool {
    let path = get_db_path(database);
    Path::new(&path).exists()
}
pub fn table_exists(database: &String, table_name: &String) -> bool {
    let path = get_table_config_path(database, table_name);
    Path::new(&path).exists()
}

pub fn get_table_column_types(table_name: &String, database: &String) -> Vec<TokenType> {
    let config_path = get_table_config_path(database, table_name);
    let content = read(config_path).expect("Table config file not found");
    let columns = String::from_utf8(content).expect("Non utf8 characters as columns");
    columns.split(";").map(|column| {
        let mut rows = column.split(",");
        let _ = rows.next();
        let _ = rows.next();
        let data_type = rows.next().expect("No datatype found");
        TokenType::get_type_from_str(data_type).expect("Incorrect datatype")
    }).collect()
}

