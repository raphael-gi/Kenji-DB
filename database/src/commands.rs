use std::fs::{create_dir, remove_dir_all};

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

fn get_db_path(db_name: String) -> String {
    format!("./data/{}", db_name)
}

