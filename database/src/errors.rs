
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

pub struct DBError {
    message: String
}

impl DBError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        DBError {
            message: message.into()
        }
    }
}

pub enum DBErrorKind {
    MissingSemicolon,
    NotUsingDB,
    EndOfStatement,
    NothingToShow,
    CantShow,
    InvalidTableName,
    ExpectedOpenBracket
}

impl DBErrorKind {
    pub fn into(self) -> DBError {
        let message = match self {
            Self::MissingSemicolon => "Missing ';'",
            Self::NotUsingDB => "asdfk",
            Self::EndOfStatement => "Unexpected end of statement",
            Self::NothingToShow => "Nothing to show provided",
            Self::CantShow => "Can't show what was provided",
            Self::InvalidTableName => "Invalid table name provided",
            Self::ExpectedOpenBracket => "Expected '(' but nothing provided"
        };

        DBError::new(message)
    }
}

