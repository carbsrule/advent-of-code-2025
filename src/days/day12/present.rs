use termcolor::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GridSquare {
    Filled,
    Empty,
}

#[allow(dead_code)]
pub enum Direction {
    Clockwise,
    Anti,
}

pub enum Orientation {
    Horizontal, // i.e. flip along the vertical axis
    Vertical,   // i.e. flip along the horizontal axis
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Present {
    pub id: u8,
    pub grid: Vec<Vec<GridSquare>>,
    pub display_as: (char, Color),
    rotations: u8,
}

impl Present {
    pub fn new(id: u8, lines: Vec<String>) -> Present {
        let mut grid = vec![];
        let mut row = 0;
        for line in lines {
            grid.push(vec![]);
            for ch in line.chars() {
                let square = match ch {
                    '.' =>  GridSquare::Empty,
                    '#' => GridSquare::Filled,
                    _ => panic!("Invalid char in present shape"),
                };
                grid[row].push(square);
            }
            row += 1;
        }
        return Present { id, grid, display_as: ('#', Color::Black), rotations: 0 };
    }

    pub fn rotate(&mut self, dir: Direction) {
        let mut new_grid = vec![];
        let new_width = self.grid.len();
        let new_height = self.grid[0].len();

        match dir {
            Direction::Clockwise => {
                for _ in 0..new_height {
                    new_grid.push(vec![]);
                }
                for row in (0..new_width).rev() {
                    for col in 0..new_height {
                        new_grid[col].push(self.grid[row][col]);
                    }
                }
                self.rotations = (self.rotations + 1) % 4;
            },
            Direction::Anti => {
                for _ in 0..new_height {
                    new_grid.push(vec![]);
                }
                for row in 0..new_width {
                    for col in (0..new_height).rev() {
                        let new_col = new_height - 1 - col;
                        new_grid[new_col].push(self.grid[row][col]);
                    }
                }
                self.rotations = (self.rotations + 3) % 4;
            }
        }

        self.grid = new_grid;
    }

    pub fn flip(&mut self, orientation: Orientation) {
        let mut new_grid = vec![];
        let height = self.grid.len();
        let width = self.grid[0].len();
        for _ in 0..height {
            new_grid.push(vec![]);
        }
        match orientation {
            Orientation::Vertical => {
                let mut new_row = 0;
                for row in (0..height).rev() {
                    for col in 0..width {
                        new_grid[new_row].push(self.grid[row][col]);
                    }
                    new_row += 1;
                }
            },
            Orientation::Horizontal => {
                for row in 0..height {
                    for col in (0..width).rev() {
                        new_grid[row].push(self.grid[row][col]);
                    }
                }
            }
        }
        self.grid = new_grid;
    }

    #[allow(dead_code)]
    pub fn draw(&self) {
        let height = self.grid.len();
        let width = self.grid[0].len();
        for row in 0..height {
            for col in 0..width {
                match self.grid[row][col] {
                    GridSquare::Empty => print!("."),
                    GridSquare::Filled => print!("#"),
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const X: GridSquare = GridSquare::Filled;
    const O: GridSquare = GridSquare::Empty;

    #[test]
    fn test_load() {
        let present = Present::new(1, vec![
            "##.".to_string(),
            "#..".to_string(),
            "###".to_string(),
        ]);

        println!("Draw:");
        present.draw();
        println!();

        assert_eq!(present.grid.len(), 3);
        assert_eq!(present.grid[1][1], GridSquare::Empty);
    }

    #[test]
    fn test_rotate() {
        let mut present = Present::new(1, vec![
            "####".to_string(),
            "#...".to_string(),
            "###.".to_string(),
        ]);
        println!("Start:");
        present.draw();
        println!();

        let mut present2 = present.clone();
        present2.rotate(Direction::Clockwise);
        println!("Clockwise:");
        present2.draw();
        println!();

        assert_eq!(present2.grid.len(), 4);
        assert_eq!(present2.grid[0].len(), 3);
        assert_eq!(present2.grid, vec![
            vec![X, X, X],
            vec![X, O, X],
            vec![X, O, X],
            vec![O, O, X],
        ]);

        present.rotate(Direction::Anti);
        println!("Anti-clockwise:");
        present.draw();
        println!();

        assert_eq!(present.grid.len(), 4);
        assert_eq!(present.grid[0].len(), 3);
        assert_eq!(present.grid, vec![
            vec![X, O, O],
            vec![X, O, X],
            vec![X, O, X],
            vec![X, X, X],
        ]);

        // 3 clockwise rotations = 1 anti-clockwise rotation
        present2.rotate(Direction::Clockwise);
        present2.rotate(Direction::Clockwise);
        assert_eq!(present.grid, present2.grid);
    }

    #[test]
    fn test_flip() {
        let mut present = Present::new(1, vec![
            "####".to_string(),
            "#...".to_string(),
            "###.".to_string(),
        ]);
        let original = present.clone();

        println!("Start:");
        present.draw();
        println!();

        present.flip(Orientation::Vertical);
        println!("Flip vertically:");
        present.draw();
        println!();

        assert_eq!(present.grid, vec![
            vec![X, X, X, O],
            vec![X, O, O, O],
            vec![X, X, X, X],
        ]);

        // Flip vertically again to get back to original
        present.flip(Orientation::Vertical);
        assert_eq!(present.grid, original.grid);

        present.flip(Orientation::Horizontal);
        println!("Flip horizontally:");
        present.draw();
        println!();

        assert_eq!(present.grid, vec![
            vec![X, X, X, X],
            vec![O, O, O, X],
            vec![O, X, X, X],
        ]);

        // Flip horizontally again to get back to original
        present.flip(Orientation::Horizontal);
        assert_eq!(present.grid, original.grid);
    }
}

