use std::fs::{create_dir, remove_dir_all};

pub fn create_database(name: String) {
    let _ = create_dir(get_db_path(name));
}

pub fn delete_database(name: String) {
    let _ = remove_dir_all(get_db_path(name));
}

fn get_db_path(db_name: String) -> String {
    format!("./data/{}", db_name)
}

