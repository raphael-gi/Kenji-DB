use lexer::Token;

pub fn err_semicolon() -> Result<(), DBError> {
    Err(DBErrorKind::MissingSemicolon.into())
}

pub fn err_not_interpreted(token: Token) -> DBError {
    DBError::new(format!("Value: '{}' could not be interpreted", token.get_string()))
}

pub fn err_invalid_datatype(token: Token) -> DBError {
    DBError::new(format!("Invalid datatype: '{}' could not be interpreted", token.get_string()))
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

