use std::{fs::OpenOptions, io::Write};
use crate::io::util::get_table_path;

pub fn insert_table(table: &String, database: &String, rows: Vec<String>, column_sizes: Vec<usize>) {
    let row_size = column_sizes.iter().sum::<usize>();

    let mut insert_values: Vec<u8> = Vec::new();

    for (i, cell) in rows.iter().enumerate() {
        let size = column_sizes[i];
        let val = cell.as_bytes();
        if val.len() > size {
            println!("Value '{}' is too large", cell);
            return;
        }
        insert_values.write_all(val).expect("Failed to write to buffer");
    }

    if insert_values.len() > row_size {
        println!("Provided data too big");
        return;
    }

    let path = get_table_path(database, &table);
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(path)
        .expect("Failed to open table");

    let _ = file.write_all(&insert_values);
}

