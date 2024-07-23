use std::{fs::{create_dir, read_dir, remove_dir_all, remove_file, File}, io::Write, path::Path, u8};

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

pub fn list_databases() {
    if let Ok(files) = read_dir("./data") {
        let mut max_len = 0;

        let mut file_names = files.map(|file| {
            if let Ok(f) = file {
                let filename = f.file_name();
                if max_len < filename.len() {
                    max_len = filename.len();
                }
                if let Some(name) = filename.to_str() {
                    return String::from(name);
                }
            }

            String::from("| Couldn't read db name |")
        }).collect::<Vec<String>>();

        for name in &mut file_names {
            let whitespace_amount: usize = max_len - name.len();
            let whitespaces: Vec<u8> = vec![b' ';whitespace_amount];
            name.push_str(&String::from_utf8(whitespaces).unwrap());
            name.push(' ');
            name.push('|');
            name.insert_str(0, "| ");
        }

        println!("{}", file_names.join("\n"));
    }
}

pub fn list_tables(database: &String) {
    if let Ok(files) = read_dir(get_db_path(database)) {
        let file_names = files.map(|file| {
            match file {
                Ok(f) => format!("| {:?}      |", f.file_name()),
                Err(..) => String::from("| Couldn't read table name |")
            }
        }).collect::<Vec<String>>().join("\n");

        println!("{}", file_names);
    }
}

pub fn database_exists(database: &String) -> bool {
    let path = get_db_path(database);
    Path::new(&path).exists()
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

