
pub fn err(error: &str) -> Option<String> {
    Some(String::from(error))
}

pub fn no_db() -> Option<String> {
    err("Not using a database")
}

pub fn err_semicolon() -> Option<String> {
    err("Missing ';'")
}
