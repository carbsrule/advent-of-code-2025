const DEBUG_OUTPUT: bool = false;

fn count_invalid_in_range_part1(left: u64, right: u64) -> u64 {
    if DEBUG_OUTPUT {
        println!("Range: {left}-{right}");
    }
    let mut sum_invalid_ids = 0;
    let mut invalid_ids = 0;
    for i in left..=right {
        let i_str = i.to_string();
        let len = i_str.len();
        if i_str.len() % 2 != 0 {
            continue;
        }
        if i_str[0..len / 2] == i_str[len / 2..] {
            invalid_ids += 1;
            sum_invalid_ids += i;
            println!("    ({invalid_ids}): {i_str}");
        }
    }
    return sum_invalid_ids;
}

fn chunk_repeated(chunk: &str, remnant: &str) -> bool {
    if remnant.len() % chunk.len() != 0 {
        if DEBUG_OUTPUT {
            println!("lengths don't match");
        }
        return false;
    }
    if chunk != &remnant[0..chunk.len()] {
        if DEBUG_OUTPUT {
            println!("no match ({chunk} in {remnant})");
        }
        return false;
    }
    if chunk == remnant {
        if DEBUG_OUTPUT {
            println!("match");
        }
        return true;
    }
    return chunk_repeated(chunk, &remnant[chunk.len()..]);
}

fn count_invalid_in_range_part2(left: u64, right: u64) -> u64 {
    if DEBUG_OUTPUT {
        println!("Range: {left}-{right}");
    }
    let mut sum_invalid_ids = 0;
    let mut invalid_ids = 0;
    for i in left..=right {
        let i_str = i.to_string();
        let len = i_str.len();
        if DEBUG_OUTPUT {
            println!("  i: {i} ({len})");
        }
        for chunk_len in 1..=len / 2 {
            let chunk = &i_str[0..chunk_len];

            if DEBUG_OUTPUT {
                print!("    chunk '{chunk}': ");
            }
            if chunk_repeated(chunk, &i_str[chunk_len..]) {
                invalid_ids += 1;
                sum_invalid_ids += i;
                if DEBUG_OUTPUT {
                    println!("    ({invalid_ids}): {i_str}");
                }
                break;
            }
        }
    }
    return sum_invalid_ids;
}

fn count_invalid_ids(lines: Vec<String>, part: u8) -> u64 {
    let mut sum_invalid_ids = 0;
    for line in lines {
        if line == "" {
            continue;
        }
        let ranges = line.split(",");
        for range in ranges {
            let sides: Vec<&str> = range.split("-").collect();
            let left: u64 = sides[0].parse().expect("Bad int on left");
            let right: u64 = sides[1].parse().expect("Bad int on right");
            sum_invalid_ids += match part {
                1 => count_invalid_in_range_part1(left, right),
                2 => count_invalid_in_range_part2(left, right),
                _ => panic!("Invalid part"),
            }
        }
    }
    println!("Total: {sum_invalid_ids}");
    return sum_invalid_ids;
}

pub fn part1(lines: Vec<String>) {
    count_invalid_ids(lines, 1);
}

pub fn part2(lines: Vec<String>) {
    count_invalid_ids(lines, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_count_invalid_ids() {
        assert_eq!(count_invalid_ids(vec!["10-30".to_string()], 1), 11 + 22);
        assert_eq!(count_invalid_ids(vec!["90-120".to_string()], 2), 99 + 111);
    }
}
