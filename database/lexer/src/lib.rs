use std::{collections::HashMap, u8, vec::IntoIter};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>
}

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    // Single Character
    LEFTBRACE, RIGHTBRACE, COMMA, SEMICOLON, QUOTATION, MINUS,

    // Literals
    IDENTIFIER, STR, INT, PK,

    // Keywords
    CREATE, DELETE, INSERT, USE, SHOW, DESC,
    DATABASE, DATABASES, TABLE, TABLES,
}

impl TokenType {
    pub fn is_same_datatype(first: TokenType, second: TokenType) -> bool {
        match (first, second) {
            (Self::STR, Self::STR) => true,
            (Self::INT, Self::INT) => true,
            _ => false
        }
    }
    pub fn get_type_from_str(input: &str) -> Option<TokenType> {
        match input {
            "STR" => Some(TokenType::STR),
            "INT" => Some(TokenType::INT),
            _ => None
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            &Self::STR => String::from("STRING"),
            &Self::INT => String::from("INTEGER"),
            _ => String::new()
        }
    }
}

pub fn scan_tokens(characters: Vec<u8>) -> Vec<Token> {
    let mut characters = characters.into_iter();
    let mut result: Vec<Token> = Vec::new();

    loop {
        if let Some(char) = characters.next() {
            let token_type = scan_token(char);
            if let Ok(token_type) = token_type {
                if let Some(token_type) = token_type {
                    match token_type {
                        TokenType::QUOTATION => match scan_str(&mut characters) {
                            Ok(str_token) => result.push(str_token),
                            Err(..) => break
                        },
                        TokenType::MINUS => match scan_number(&mut characters, vec![b'-']) {
                            Ok((digit_token, last_token)) => {
                                result.push(digit_token);
                                if let Some(last_token) = last_token {
                                    result.push(last_token);
                                }
                            },
                            Err(..) => break
                        },
                        _ => result.push(Token { value: None, token_type })
                    };
                }
                continue;
            }

            if char.is_ascii_digit() {
                match scan_number(&mut characters, vec![char]) {
                    Ok((digit_token, last_token)) => {
                        result.push(digit_token);
                        if let Some(last_token) = last_token {
                            result.push(last_token);
                        }
                    },
                    Err(..) => break
                }
                continue;
            }

            if char.is_ascii_alphabetic() {
                match scan_word(&mut characters, vec![char]) {
                    Ok((token, identifier_token)) => {
                        result.push(token);
                        if let Some(token) = identifier_token {
                            result.push(token);
                        }
                    },
                    Err(..) => break
                }
                continue;
            }
        }
        break;
    }

    result
}

fn scan_number(characters: &mut IntoIter<u8>, mut digits: Vec<u8>) -> Result<(Token, Option<Token>), ()> {
    let char = characters.next();

    match char {
        Some(char) => {
            if !char.is_ascii_digit() {
                let second_token = match scan_token(char) {
                    Ok(token_type) => match token_type {
                        Some(token_type) => Some(Token {
                            value: None,
                            token_type
                        }),
                        None => None
                    },
                    Err(..) => None
                };
                return Ok((Token {
                    token_type: TokenType::INT,
                    value: Some(String::from_utf8(digits).unwrap())
                }, second_token));
            }

            digits.push(char);
            scan_number(characters, digits)
        },
        None => Err(())
    }
}

fn scan_word(characters: &mut IntoIter<u8>, mut word: Vec<u8>) -> Result<(Token, Option<Token>), ()> {
    let mut keywords: HashMap<String, TokenType> = HashMap::new();
    keywords.insert(String::from("CREATE"), TokenType::CREATE);
    keywords.insert(String::from("DELETE"), TokenType::DELETE);
    keywords.insert(String::from("INSERT"), TokenType::INSERT);
    keywords.insert(String::from("USE"), TokenType::USE);
    keywords.insert(String::from("SHOW"), TokenType::SHOW);
    keywords.insert(String::from("LS"), TokenType::SHOW);
    keywords.insert(String::from("DESC"), TokenType::DESC);
    keywords.insert(String::from("DATABASE"), TokenType::DATABASE);
    keywords.insert(String::from("DB"), TokenType::DATABASE);
    keywords.insert(String::from("DATABASES"), TokenType::DATABASES);
    keywords.insert(String::from("DBS"), TokenType::DATABASES);
    keywords.insert(String::from("TABLE"), TokenType::TABLE);
    keywords.insert(String::from("TB"), TokenType::TABLE);
    keywords.insert(String::from("TABLES"), TokenType::TABLES);
    keywords.insert(String::from("TBS"), TokenType::TABLES);
    keywords.insert(String::from("STR"), TokenType::STR);
    keywords.insert(String::from("INT"), TokenType::INT);
    keywords.insert(String::from("PK"), TokenType::PK);

    let key = match String::from_utf8(word.clone()) {
        Ok(key) => key,
        Err(..) => {
            return Err(());
        }
    };
    let uppercase_key = key.to_uppercase();

    let current_char = match characters.next() {
        Some(current_char) => current_char,
        None => {
            if keywords.contains_key(&uppercase_key) {
                return Ok((
                    Token {
                        value: None,
                        token_type: keywords.get(&uppercase_key).copied().unwrap()
                    }, None
                ))
            } 
            return Ok((
                Token {
                    value: Some(key),
                    token_type: TokenType::IDENTIFIER
                }, None
            ));
        }
    };

    if let Ok(found_token) = scan_token(current_char) {
        let identifier_token = match found_token {
            Some(token_type) => Some(Token { value: None, token_type }),
            None => None
        };

        if keywords.contains_key(&uppercase_key) {
            return Ok((
                Token {
                    value: None,
                    token_type: keywords.get(&uppercase_key).copied().unwrap()
                }, identifier_token
            ));
        } 
        return Ok((
            Token {
                value: Some(key),
                token_type: TokenType::IDENTIFIER
            }, identifier_token
        ));
    }

    word.push(current_char);
    scan_word(characters, word)
}

fn scan_str(characters: &mut IntoIter<u8>) -> Result<Token, ()> {
    let mut str: Vec<u8> = Vec::new();

    loop {
        let char = characters.next();
        match char {
            Some(char) => match char {
                b'"' => break,
                _ => str.push(char)
            },
            None => return Err(())
        }
    }

    let value = String::from_utf8(str);
    match value {
        Ok(str) => Ok(Token {
            token_type: TokenType::STR,
            value: Some(str)
        }),
        Err(..) => Err(())
    }
}

fn scan_token(character: u8) -> Result<Option<TokenType>, ()> {
    match character {
        b' ' => Ok(None),
        b'\n' => Ok(None),
        b'-' => Ok(Some(TokenType::MINUS)),
        b';' => Ok(Some(TokenType::SEMICOLON)),
        b'(' => Ok(Some(TokenType::LEFTBRACE)),
        b')' => Ok(Some(TokenType::RIGHTBRACE)),
        b',' => Ok(Some(TokenType::COMMA)),
        b'"' => Ok(Some(TokenType::QUOTATION)),
        _ => Err(())
    }
}

