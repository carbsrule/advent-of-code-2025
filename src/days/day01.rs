use crate::lines;
use crate::num;

pub fn part1(lines: Vec<String>) {
    let mut dial = 50;
    let mut zeroes = 0;
    for line in lines {
        if line == "" {
            break
        }
        let dir = line.chars().nth(0).unwrap();
        let clicks = num::parse_int(&line[1..]);
        match dir {
            'L' => dial -= clicks,
            'R' => dial += clicks,
            _ => panic!("Unknown direction: {dir}")
        }
        dial %= 100;
        if dial == 0 {
            zeroes += 1;
        }
    }
    println!("Zeroes: {zeroes}");
}

pub fn part2(lines: Vec<String>) {
    lines::write_lines(lines);
}
