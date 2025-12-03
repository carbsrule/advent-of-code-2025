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
        println!("    pos: {pos}, max_digit: {max_digit} ({line})");
    }
    return (pos, max_digit);
}

// Highest joltage for 2 batteries in bank
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

// Highest joltage for 12 batteries in bank
fn highest_joltage_12(line: &str) -> u64 {
    let mut digits = vec![];
    let mut digit_pos = 0;
    if DEBUG_OUTPUT {
        println!("Line: {line} {:}", line.len());
    }
    for step in 0..12 {
        let remaining_digits = 11 - digits.len();
        if DEBUG_OUTPUT {
            println!("    step {step}: {digit_pos} -- {}", &line[digit_pos..]);
            // println!("    use {step}: {}", &line[digit_pos..]);
        }
        let (offset, digit) = max_digit(&line[digit_pos..line.len() - remaining_digits]);
        if DEBUG_OUTPUT {
            println!("    digit at {offset}: {digit}");
        }
        digits.push(digit);
        digit_pos = digit_pos + offset + 1;
    }

    if DEBUG_OUTPUT {
        println!("    Joltage: {:?}", digits);
    }
    let mut joltage = 0;
    let mut exp = 0;
    while digits.len() > 0 {
        let digit = digits.pop().unwrap();
        joltage += 10u64.pow(exp) * u64::from(digit);
        exp += 1;
    }
    return joltage;
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

pub fn part2(lines: Vec<String>) {
    let num_lines = lines.len();
    if DEBUG_OUTPUT {
        println!("{num_lines} line(s)");
    }
    let mut total_joltage = 0;
    for line in lines {
        total_joltage += highest_joltage_12(&line);
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

    #[test]
    fn test_highest_joltage12() {
        assert_eq!(highest_joltage_12("234234234234278"), 434234234278);
    }
}
