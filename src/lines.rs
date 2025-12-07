use std::io;

pub fn read_line() -> (usize, String) {
    let mut line = String::new();
    let num_bytes = io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    if num_bytes == 0 {
        line = "".to_string();
    } else {
        line = line.trim_end_matches('\n').to_string();
    }
    return (num_bytes, line);
}

/**
 * Dummy function
 */
#[allow(dead_code)]
pub fn write_lines(lines: Vec<String>) {
    let mut x = 1;
    for line in lines {
        println!("Line {x}: {line}");
        x += 1;
    }
}
