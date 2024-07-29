
pub fn err<S: Into<String>>(error: S) -> Option<String> {
    Some(error.into())
}

pub fn no_db() -> Option<String> {
    err("Not using a database")
}

pub fn err_semicolon() -> Option<String> {
    err("Missing ';'")
}

pub fn err_abrupt_ending() -> Option<String> {
    err("Unexpected end of statement")
}

