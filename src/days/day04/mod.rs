mod map_grid;

use map_grid::Map;

pub fn count_accessible(lines: Vec<String>) -> u32 {
    let map = Map::load(lines);
    let mut num_accessible = 0;
    for row in 0..map.grid.len() {
        for col in 0..map.grid[0].len() {
            if map.grid[row][col] == false {
                continue;
            }
            if map.surrounding_rolls(row, col) < 4 {
                num_accessible += 1;
            }
        }
    }
    return num_accessible;
}

pub fn part1(lines: Vec<String>) {
    println!("Accessible: {}", count_accessible(lines));
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn part2(lines: Vec<String>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_accessible() {
        assert_eq!(
            count_accessible(vec![
                "@@@".to_string(),
                "@@@".to_string(),
                "@@@".to_string(),
            ]),
            4
        );
        assert_eq!(
            count_accessible(vec![
                ".@.".to_string(),
                "@@@".to_string(),
                ".@.".to_string(),
            ]),
            4
        );
    }
}
