use super::*;

#[test]
fn scan_tokens_test() {
    let input = "CREATE TABLE store () DATABASE".as_bytes().to_vec();
    let tokens = scan_tokens(input);
    println!("{:?}", tokens);
}

