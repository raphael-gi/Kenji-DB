use std::fs::{create_dir, remove_dir_all, File};

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

pub fn create_table(name: String, database: &String) {
    let path = get_db_path(database.to_string());
    match File::create(format!("{}/{}", path, name)) {
        Ok(..) => println!("Created table"),
        Err(..) => println!("Failed to create table")
    }
}

fn get_db_path(db_name: String) -> String {
    format!("./data/{}", db_name)
}

