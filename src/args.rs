use std::env;

pub fn get_day_part() -> (u8, u8) {
    let day: u8 = env::args()
        .nth(1)
        .unwrap_or("1".to_string())
        .parse()
        .expect("Failed to parse day arg");
    let mut part: u8 = env::args()
        .nth(2)
        .unwrap_or("1".to_string())
        .parse()
        .expect("Failed to parse part arg");
    if part != 2 {
        part = 1;
    }
    return (day, part);
}
