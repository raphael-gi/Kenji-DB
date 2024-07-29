use std::{fs::{create_dir, read, read_dir, remove_dir_all, remove_file, File}, io::Write, path::Path, u8, vec};

use lexer::TokenType;

pub struct TableColumn {
    pub pk: bool,
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

pub fn insert_table(table: &String, database: &String, _columns: Vec<String>) {
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

pub fn desc_table(table_name: String, database: &String) {
    let path = get_table_config_path(database, &table_name);
    let content = read(path);
    match content {
        Ok(content) => match String::from_utf8(content) {
            Ok(content) => {
                let mut max_lengths: [usize;2] = [5,4];

                let columns = content.split(";");
                let mut rows: Vec<[String;2]> = columns.map(|column| {
                    let mut rows = column.split(",");
                    let field = rows.next().expect("Field name doesn't exists");
                    if field.len() > max_lengths[0] {
                        max_lengths[0] = field.len();
                    }
                    let data_type = rows.next().expect("Data Type doesn't exists");
                    if data_type.len() > max_lengths[1] {
                        max_lengths[1] = data_type.len();
                    }

                    return [String::from(field), String::from(data_type)];
                }).collect();

                rows.insert(0, [String::from("Field"), String::from("Type")]);

                decorate_table(&mut rows, max_lengths);

                let mut seperated_rows = rows.iter().map(|row| {
                    let mut res = row.join(" | ");
                    res.insert_str(0, "| ");
                    res.push_str(" |");
                    res
                }).collect::<Vec<String>>();

                let mut seperators = max_lengths.iter().map(|max_len| {
                    String::from_utf8(vec![b'-';max_len + 2]).unwrap()
                }).collect::<Vec<String>>().join("+");
                seperators.insert(0, '+');
                seperators.push('+');

                seperated_rows.insert(0, seperators.clone());
                seperated_rows.insert(2, seperators.clone());
                seperated_rows.push(seperators);

                println!("{}", seperated_rows.join("\n"));
            },
            Err(err) => println!("{}", err)
        },
        Err(err) => println!("{}", err)
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
        let data_type = rows.next().expect("No datatype found");
        TokenType::get_type_from_str(data_type).expect("Incorrect datatype")
    }).collect()
}

fn decorate_table(list: &mut Vec<[String; 2]>, max_lengths: [usize; 2])  {
    for row in list {
        for (i, cell) in row.into_iter().enumerate() {
            let whitespace_amount: usize = max_lengths[i] - cell.len();
            let whitespaces: Vec<u8> = vec![b' ';whitespace_amount];
            cell.push_str(&String::from_utf8(whitespaces).unwrap());
        }
    }
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

