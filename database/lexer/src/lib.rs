
enum TokenType {
    // Single Character
    LEFTBRACE, RIGHTBRACE, COMMA,

    // Keywords
    CREATE, DELETE, DATABASE, TABLE
}

fn scan_tokens(characters: Vec<u8>) -> Vec<TokenType> {
    let mut result = Vec::new();
    for character in characters {
        match scan_token(character) {
            Ok(char) => result.push(char),
            Err(..) => println!("Token Type not found")
        }
    }

    result
}

fn scan_token(character: u8) -> Result<TokenType, ()> {
    match character {
        b'(' => Ok(TokenType::LEFTBRACE),
        _ => Err(())
    }
}

