use std::fs::{create_dir, remove_dir_all, remove_file, File};

pub struct TableColumn {
    pub name: String,
    pub data_type: String
}

pub struct Table {
    pub name: String,
    pub database: String,
    pub rows: Vec<TableColumn>
}

pub fn create_database(name: String) {
    match create_dir(get_db_path(name)) {
        Ok(..) => println!("Created database"),
        Err(..) => println!("Failed to create database")
    };
}

pub fn delete_database(name: String) {
    match remove_dir_all(get_db_path(name)) {
        Ok(..) => println!("Deleted database"),
        Err(..) => println!("Failed to delete database")
    }
}

pub fn create_table(table: Table) {
    let path = get_db_path(table.database);
    match File::create(format!("{}/{}", path, table.name)) {
        Ok(..) => println!("Created table"),
        Err(..) => println!("Failed to create table")
    }
}

pub fn delete_table(name: String, database: String) {
    match remove_file(get_table_path(database, name)) {
        Ok(..) => println!("Deleted table"),
        Err(..) => println!("Failed to delete database")
    }
}

fn get_table_path(db_name: String, table_name: String) -> String {
    let path = get_db_path(db_name);
    format!("{}/{}", path, table_name)
}

fn get_db_path(db_name: String) -> String {
    format!("./data/{}", db_name)
}

