use std::{fs::{remove_dir_all, remove_file}, path::Path};

use crate::io::util::{get_table_config_path, get_table_path, get_db_path};

pub fn delete_database(name: String) {
    match remove_dir_all(get_db_path(&name)) {
        Ok(..) => println!("Deleted database: {}", name),
        Err(..) => println!("Failed to delete database")
    }
}

pub fn delete_table(name: String, database: &String) {
    let path = get_table_path(database, &name);
    let config_path = get_table_config_path(database, &name);
    if !Path::new(&path).exists() || !Path::new(&config_path).exists() {
        return println!("Table doesn't exists");
    }
    if remove_file(path).is_err() {
        return println!("Failed to delete table");
    }

    if remove_file(config_path).is_err() {
        return println!("Failed to delete table");
    }

    println!("Deleted table: {}", name);
}

