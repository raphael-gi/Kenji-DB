use std::fs::{read, read_dir};

use crate::io::util::{get_db_path, get_table_config_path};

pub fn show_databases(database: &Option<String>) {
    if let Ok(files) = read_dir("./data") {
        let mut max_len = 9;

        let mut file_names = files.map(|file| {
            if let Ok(f) = file {
                let filename = f.file_name();
                if let Some(name) = filename.to_str() {
                    let mut name = String::from(name);
                    if let Some(database) = database {
                        if database == &name {
                            name.insert(0, '*');
                        }
                    }
                    if max_len < name.len() {
                        max_len = name.len();
                    }
                    return name;
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
                let mut max_lengths: [usize;3] = [3,5,4];

                let columns = content.split(";");
                let mut rows: Vec<[String;3]> = columns.map(|column| {
                    let mut rows = column.split(",");
                    let key = match rows.next() {
                        Some(key) => String::from(key),
                        None => String::new()
                    };
                    let field = rows.next().expect("Field name doesn't exists");
                    if field.len() > max_lengths[1] {
                        max_lengths[1] = field.len();
                    }
                    let data_type = rows.next().expect("Data Type doesn't exists");
                    if data_type.len() > max_lengths[2] {
                        max_lengths[2] = data_type.len();
                    }

                    return [key, String::from(field), String::from(data_type)];
                }).collect();

                rows.insert(0, [
                    String::from("Key"),
                    String::from("Field"),
                    String::from("Type")
                ]);

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

fn decorate_table(list: &mut Vec<[String; 3]>, max_lengths: [usize; 3])  {
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
