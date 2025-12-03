const DEBUG_OUTPUT: bool = false;

fn max_digit(line: &str) -> (usize, u8) {
    let mut max_digit = 0;
    let mut pos = 0;
    for i in 0..line.len() {
        let digit: u8 = line[i..i + 1].parse().expect("Must be a number");
        if digit > max_digit {
            pos = i;
            max_digit = digit;
        }
    }
    if DEBUG_OUTPUT {
        println!("    pos: {pos}, max_digit: {max_digit}");
    }
    return (pos, max_digit);
}

fn highest_joltage(line: &str) -> u8 {
    if DEBUG_OUTPUT {
        println!("Line: {line}");
    }
    let (start, digit1) = max_digit(&line[0..line.len() - 1]);
    let (_, digit2) = max_digit(&line[start + 1..]);
    if DEBUG_OUTPUT {
        println!("    Joltage: {digit1}{digit2}");
    }
    return (digit1 * 10) + digit2;
}

pub fn part1(lines: Vec<String>) {
    let num_lines = lines.len();
    if DEBUG_OUTPUT {
        println!("{num_lines} line(s)");
    }
    let mut total_joltage: u32 = 0;
    for line in lines {
        total_joltage += u32::from(highest_joltage(&line));
    }
    println!("Total joltage: {total_joltage}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_joltage() {
        assert_eq!(highest_joltage("12345"), 45);
        assert_eq!(highest_joltage("54321"), 54);
        assert_eq!(highest_joltage("92468"), 98);
        assert_eq!(highest_joltage("919293"), 99);
        assert_eq!(highest_joltage("9999"), 99);
        assert_eq!(highest_joltage("212121"), 22);
        assert_eq!(highest_joltage("97423391"), 99);
    }
}
