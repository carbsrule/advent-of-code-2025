fn count_invalid_ids(lines: Vec<String>) -> u64 {
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
            println!("Range: {left}-{right}");
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
                    println!("    ({invalid_ids}): {i_str}")
                }
            }
        }
    }
    println!("Total: {sum_invalid_ids}");
    return sum_invalid_ids;
}

pub fn part1(lines: Vec<String>) {
    count_invalid_ids(lines);
}

pub fn part2(_lines: Vec<String>) {
    println!("TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_count_invalid_ids() {
        let lines = vec!["10-30".to_string()];
        assert_eq!(count_invalid_ids(lines), 33);
    }
}
