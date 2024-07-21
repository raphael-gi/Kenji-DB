use std::{collections::HashMap, vec::IntoIter};


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
    LEFTBRACE, RIGHTBRACE, COMMA, SEMICOLON,

    // Literals
    IDENTIFIER, STR, INT,

    // Keywords
    CREATE, DELETE, USE, DATABASE, TABLE
}

pub fn scan_tokens(characters: Vec<u8>) -> Vec<Token> {
    let mut characters = characters.into_iter();
    let mut result: Vec<Token> = Vec::new();

    loop {
        if let Some(char) = characters.next() {
            let token_type = scan_token(char);
            if let Ok(token_type) = token_type {
                if let Some(token_type) = token_type {
                    result.push(Token { value: None, token_type });
                }
                continue;
            }

            if char.is_ascii_alphabetic() {
                let (token, identifier_token) = scan_word(&mut characters, vec![char]);
                result.push(token);
                if let Some(token) = identifier_token {
                    result.push(token);
                }
            }
        } else {
            break;
        }
    }

    result
}

fn scan_word(characters: &mut IntoIter<u8>, mut word: Vec<u8>) -> (Token, Option<Token>) {
    let mut keywords: HashMap<String, TokenType> = HashMap::new();
    keywords.insert(String::from("CREATE"), TokenType::CREATE);
    keywords.insert(String::from("DELETE"), TokenType::DELETE);
    keywords.insert(String::from("USE"), TokenType::USE);
    keywords.insert(String::from("DATABASE"), TokenType::DATABASE);
    keywords.insert(String::from("TABLE"), TokenType::TABLE);
    keywords.insert(String::from("STR"), TokenType::STR);
    keywords.insert(String::from("INT"), TokenType::INT);

    let key = String::from_utf8(word.clone()).unwrap();

    let current_char = match characters.next() {
        Some(current_char) => current_char,
        None => {
            if keywords.contains_key(&key) {
                return (
                    Token {
                        value: None,
                        token_type: keywords.get(&key).copied().unwrap()
                    }, None
                )
            } 
            return (
                Token {
                    value: Some(key),
                    token_type: TokenType::IDENTIFIER
                }, None
            );
        }
    };

    if let Ok(found_token) = scan_token(current_char) {
        let identifier_token = match found_token {
            Some(token_type) => Some(Token { value: None, token_type }),
            None => None
        };

        if keywords.contains_key(&key) {
            return (
                Token {
                    value: None,
                    token_type: keywords.get(&key).copied().unwrap()
                }, identifier_token
            )
        } 
        return (
            Token {
                value: Some(key),
                token_type: TokenType::IDENTIFIER
            }, identifier_token
        );
    }

    word.push(current_char);
    scan_word(characters, word)
}

fn scan_token(character: u8) -> Result<Option<TokenType>, ()> {
    match character {
        b' ' => Ok(None),
        b'\n' => Ok(None),
        b';' => Ok(Some(TokenType::SEMICOLON)),
        b'(' => Ok(Some(TokenType::LEFTBRACE)),
        b')' => Ok(Some(TokenType::RIGHTBRACE)),
        b',' => Ok(Some(TokenType::COMMA)),
        _ => Err(())
    }
}

