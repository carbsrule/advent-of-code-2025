pub struct Map {
    pub grid: Vec<Vec<bool>>, // True if grid location has a roll of paper
}

impl Map {
    pub fn new() -> Map {
        Map { grid: vec![] }
    }

    pub fn load(lines: Vec<String>) -> Map {
        let mut map = Map::new();
        let mut row: usize = 0;
        let mut col: usize;
        for line in lines {
            col = 0;
            for ch in line.bytes() {
                match ch {
                    b'@' => map.set(row, col, true),
                    _ => map.set(row, col, false),
                }
                col += 1;
            }
            row += 1;
        }
        return map;
    }

    pub fn set(&mut self, row: usize, col: usize, val: bool) {
        if self.grid.len() <= row {
            self.grid.push(vec![]);
        }
        if self.grid[row].len() <= col {
            self.grid[row].push(false);
        }
        self.grid[row][col] = val;
    }

    #[allow(dead_code)]
    pub fn render(&self) -> String {
        let mut chars = vec![];
        for row in &self.grid {
            for col in row {
                chars.push(match col {
                    true => b'@',
                    false => b'.',
                });
            }
            chars.push(b'\n');
        }
        return String::from_utf8(chars).unwrap();
    }

    pub fn surrounding_rolls(&self, row: usize, col: usize) -> u8 {
        let mut rolls = 0;

        let left_ok = col > 0;
        let right_ok = col < self.grid[0].len() - 1;
        let up_ok = row > 0;
        let down_ok = row < self.grid.len() - 1;

        // above
        if up_ok {
            if left_ok && self.grid[row - 1][col - 1] {
                rolls += 1;
            }
            if self.grid[row - 1][col] {
                rolls += 1;
            }
            if right_ok && self.grid[row - 1][col + 1] {
                rolls += 1;
            }
        }

        // same row
        if left_ok && self.grid[row][col - 1] {
            rolls += 1;
        }
        if right_ok && self.grid[row][col + 1] {
            rolls += 1;
        }

        // below
        if down_ok {
            if left_ok && self.grid[row + 1][col - 1] {
                rolls += 1;
            }
            if self.grid[row + 1][col] {
                rolls += 1;
            }
            if right_ok && self.grid[row + 1][col + 1] {
                rolls += 1;
            }
        }

        return rolls;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let map = Map::load(vec![
            "@@@".to_string(),
            "@@@".to_string(),
            "@@@".to_string(),
        ]);

        assert_eq!(map.render(), "@@@\n@@@\n@@@\n".to_string());

        assert_eq!(map.surrounding_rolls(0, 0), 3);
        assert_eq!(map.surrounding_rolls(0, 1), 5);
        assert_eq!(map.surrounding_rolls(0, 2), 3);
        assert_eq!(map.surrounding_rolls(1, 0), 5);
        assert_eq!(map.surrounding_rolls(1, 1), 8);
        assert_eq!(map.surrounding_rolls(1, 2), 5);
        assert_eq!(map.surrounding_rolls(2, 0), 3);
        assert_eq!(map.surrounding_rolls(2, 1), 5);
        assert_eq!(map.surrounding_rolls(2, 2), 3);
    }
}
