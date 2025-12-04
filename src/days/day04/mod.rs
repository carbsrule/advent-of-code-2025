mod map_grid;

use map_grid::Map;

pub fn find_accessible(map: &Map) -> Vec<(usize, usize)> {
    let mut accessible = vec![];
    for row in 0..map.grid.len() {
        for col in 0..map.grid[0].len() {
            if map.grid[row][col] == false {
                continue;
            }
            if map.surrounding_rolls(row, col) < 4 {
                accessible.push((row, col));
            }
        }
    }
    return accessible;
}

pub fn part1(lines: Vec<String>) {
    let map = Map::load(lines);
    let accessible = find_accessible(&map);
    println!("Accessible: {}: {:?}", accessible.len(), accessible);
}

pub fn part2(lines: Vec<String>) {
    let mut map = Map::load(lines);
    let mut round = 0;
    let mut total_removed = 0;
    loop {
        round += 1;
        let accessible = find_accessible(&map);
        println!("Round {round}: removing {} rolls", accessible.len());
        if accessible.len() == 0 {
            break;
        }
        total_removed += accessible.len();
        for rem in accessible {
            map.set(rem.0, rem.1, false);
        }
    }
    println!("Total removed: {total_removed}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_accessible() {
        assert_eq!(
            find_accessible(&Map::load(vec![
                "@@@".to_string(),
                "@@@".to_string(),
                "@@@".to_string(),
            ]))
            .len(),
            4
        );
        assert_eq!(
            find_accessible(&Map::load(vec![
                ".@.".to_string(),
                "@@@".to_string(),
                ".@.".to_string(),
            ]))
            .len(),
            4
        );
    }
}
