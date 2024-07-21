use std::{fs::{create_dir, remove_dir_all, remove_file, File}, io::Write};

pub struct TableColumn {
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
        self.rows.iter().map(|table| {
            format!("{},{}", table.name, table.data_type)
        }).collect::<Vec<String>>().join(";")
    }
}

pub fn create_database(name: String) {
    match create_dir(get_db_path(&name)) {
        Ok(..) => println!("Created database: {}", name),
        Err(..) => println!("Failed to create database")
    };
}

pub fn delete_database(name: String) {
    match remove_dir_all(get_db_path(&name)) {
        Ok(..) => println!("Deleted database: {}", name),
        Err(..) => println!("Failed to delete database")
    }
}

pub fn create_table(table: Table) {
    let path = get_table_path(&table.database, &table.name);
    let config_path = get_table_config_path(&table.database, &table.name);

    if File::create(path).is_err() {
        return println!("Failed to create table");
    }
    match File::create(config_path) {
        Ok(mut config_file) => {
            let content = table.get_row_string();
            config_file.write(content.as_bytes()).unwrap();
            println!("Created table: {}", table.name);
        },
        Err(..) => println!("Failed to create table")
    };
}

pub fn delete_table(name: String, database: &String) {
    if remove_file(get_table_path(database, &name)).is_err() {
        return println!("Failed to delete database");
    }

    if remove_file(get_table_config_path(database, &name)).is_err() {
        return println!("Failed to delete database");
    }

    println!("Deleted table: {}", name);
}

pub fn insert_table(table: String, database: &String) {
    let path = get_table_path(database, &table);
}

fn get_table_config_path(db_name: &String, table_name: &String) -> String {
    let path = get_db_path(db_name);
    format!("{}/.{}", path, table_name)
}
fn get_table_path(db_name: &String, table_name: &String) -> String {
    let path = get_db_path(db_name);
    format!("{}/{}", path, table_name)
}

fn get_db_path(db_name: &String) -> String {
    format!("./data/{}", db_name)
}

