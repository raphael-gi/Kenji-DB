use super::*;

#[test]
fn scan_tokens_test() {
    let input = "CREATE TABLE store () DATABASE".as_bytes().to_vec();
    let tokens = scan_tokens(input);
    println!("{:?}", tokens);
}

#[test]
fn scan_str_test() {
    let input = r#"INSERT users ("John", 45)"#.as_bytes().to_vec();
    let tokens = scan_tokens(input);
    println!("{:?}", tokens);
}

