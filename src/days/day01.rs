use crate::num;

fn count_clicks(lines: Vec<String>, part: u8, mut dial: i32) -> u32 {
    let mut zeroes = 0;
    for line in lines {
        if line == "" {
            break;
        }
        let mut dial_was_zero = dial == 0;
        print!("{dial}: {line}");
        let dir = line.chars().nth(0).unwrap();
        let clicks = num::parse_int(&line[1..]);
        match dir {
            'L' => dial -= clicks,
            'R' => dial += clicks,
            _ => panic!("Unknown direction: {dir}"),
        }

        print!("({clicks}) -> {dial}");

        if part == 1 {
            dial %= 100;
            if dial == 0 {
                zeroes += 1;
            }
        } else if part == 2 {
            if dial == 0 {
                zeroes += 1;
                print!(", click");
            }
            while dial > 99 {
                dial -= 100;
                zeroes += 1;
                print!(", click (R)");
            }
            while dial < 0 {
                dial += 100;
                if !dial_was_zero {
                    zeroes += 1;
                    print!(", click (L)");
                } else {
                    print!(", ignore (L)");
                    dial_was_zero = false;
                }
                if dial == 0 {
                    zeroes += 1;
                    print!(", click");
                }
            }

            println!(", end: {dial}, clicks: {zeroes}");
        }
    }
    println!("Zeroes: {zeroes}");
    return zeroes;
}

pub fn part1(lines: Vec<String>) {
    count_clicks(lines, 1, 50);
}

pub fn part2(lines: Vec<String>) {
    count_clicks(lines, 2, 50);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_rotate_left() {
        assert_eq!(count_clicks(vec!["L100".to_string()], 2, 50), 1);
        assert_eq!(count_clicks(vec!["L150".to_string()], 2, 50), 2);
        assert_eq!(count_clicks(vec!["L1".to_string()], 2, 1), 1);
        assert_eq!(count_clicks(vec!["L1".to_string()], 2, 0), 0);
    }

    #[test]
    fn test_part2_full_rotate_left() {
        assert_eq!(count_clicks(vec!["L100".to_string()], 2, 0), 1);
    }

    #[test]
    fn test_part2_full_rotate_right() {
        assert_eq!(count_clicks(vec!["R100".to_string()], 2, 0), 1);
    }
}
