#[allow(dead_code)]
pub fn parse_int(num: &str) -> i32 {
    return num.parse().expect("Failed to parse number");
}

#[allow(dead_code)]
pub fn char_parse_int(ch: &char) -> i32 {
    let mut num = String::new();
    num.push(*ch);
    return parse_int(&num);
}
