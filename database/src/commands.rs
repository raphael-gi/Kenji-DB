use std::{fs::{create_dir, read_dir, remove_dir_all, remove_file, File}, io::Write, path::Path, u8, vec};

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

pub fn _insert_table(table: String, database: &String) {
    let _path = get_table_path(database, &table);
}

pub fn show_databases() {
    if let Ok(files) = read_dir("./data") {
        let mut max_len = 9;

        let mut file_names = files.map(|file| {
            if let Ok(f) = file {
                let filename = f.file_name();
                if let Some(name) = filename.to_str() {
                    if max_len < name.len() {
                        max_len = name.len();
                    }
                    return String::from(name);
                }
            }

            String::from("| Couldn't read db name |")
        }).collect::<Vec<String>>();

        file_names.insert(0 ,String::from("Databases"));
        decorate_listing(&mut file_names, max_len);

        println!("{}", file_names.join("\n"));
    }
}

pub fn show_tables(database: &String) {
    if let Ok(files) = read_dir(get_db_path(database)) {
        let mut max_len = 6;

        let mut file_names: Vec<String> = Vec::new();
        for file in files {
            if let Ok(f) = file {
                let filename = f.file_name();
                if let Some(name) = filename.to_str() {
                    if name.starts_with(".") {
                        continue;
                    }
                    if max_len < name.len() {
                        max_len = name.len();
                    }
                    file_names.push(String::from(name));
                    continue;
                }
            }

            file_names.push(String::from("Couldn't read table name"));
        }

        file_names.insert(0 ,String::from("Tables"));
        decorate_listing(&mut file_names, max_len);

        println!("{}", file_names.join("\n"));
    }
}

pub fn database_exists(database: &String) -> bool {
    let path = get_db_path(database);
    Path::new(&path).exists()
}

fn decorate_listing(list: &mut Vec<String>, max_len: usize) {
    for name in &mut *list {
        let whitespace_amount: usize = max_len - name.len();
        let whitespaces: Vec<u8> = vec![b' ';whitespace_amount];
        name.push_str(&String::from_utf8(whitespaces).unwrap());
        name.push(' ');
        name.push('|');
        name.insert_str(0, "| ");
    }

    let mut seperator_lign = vec![b'-';max_len + 2];
    seperator_lign.insert(0, b'+');
    seperator_lign.push(b'+');
    let seperator = String::from_utf8(seperator_lign).unwrap();

    list.insert(0, seperator.clone());
    list.insert(2, seperator.clone());
    list.push(seperator);
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

