pub fn err_semicolon() -> Result<(), DBError> {
    Err(DBErrorKind::MissingSemicolon.into())
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
    pub fn get_message(self) -> String {
        self.message
    }
}

pub enum DBErrorKind {
    MissingSemicolon,
    NotUsingDB,
    EndOfStatement,
    NothingToShow,
    CantShow,
    InvalidTableName,
    ExpectedOpenBracket,
}

impl DBErrorKind {
    pub fn into(self) -> DBError {
        let message = match self {
            Self::MissingSemicolon => "Missing ';'",
            Self::NotUsingDB => "Not using a database",
            Self::EndOfStatement => "Unexpected end of statement",
            Self::NothingToShow => "Nothing to show provided",
            Self::CantShow => "Can't show what was provided",
            Self::InvalidTableName => "Invalid table name provided",
            Self::ExpectedOpenBracket => "Expected '(' but nothing provided"
        };

        DBError::new(message)
    }
}

