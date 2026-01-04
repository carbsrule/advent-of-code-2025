use crate::days::day12::present::*;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

type PresentCount = u8;
type PresentCountList = Vec<PresentCount>;

pub struct Area {
    pub height: usize,
    pub width: usize,
    pub presents_reqd: PresentCountList,
    grid: Vec<Vec<(char, Color)>>,
    next_present: (char, Color),
}

impl Area {
    pub fn new(height: usize, width: usize) -> Area {
        let mut grid: Vec<Vec<(char, Color)>> = vec![];
        for row in 0..height {
            grid.push(vec![]);
            for _ in 0..width {
                grid[row].push(('.', Color::Rgb(150, 150, 150)));
            }
        }
        return Area {
            height,
            width,
            presents_reqd: vec![],
            grid,
            next_present: ('A', Color::White),
        }
    }

    fn place_present(&mut self, row: usize, col: usize, present: &mut Present) -> bool {
        // println!("    Placing present {} at ({row},{col}): {}", present.id, self.grid[row][col]);
        let present_height = present.grid.len();
        let present_width = present.grid[0].len();

        if row + present_height > self.height {
            return false;
        }
        if col + present_width > self.width {
            return false;
        }

        let mut attempt = 0;

        // Try 0: plain
        // Try 1: flipped horizontally
        // Try 2: flipped vertically
        // Repeat another 3x, rotating right each time (total 12 tries)
        'attempts: while attempt < 12 {
            let remainder = attempt % 3;
            match remainder {
                0 => {
                    if attempt > 0 {
                        // restore to original vertical layout before rotation
                        present.flip(Orientation::Vertical);
                        present.rotate(Direction::Clockwise);
                    }
                },
                1 => {
                    present.flip(Orientation::Horizontal);
                },
                2 => {
                    // restore to original horizontal layout before vertical flip
                    present.flip(Orientation::Horizontal);
                    present.flip(Orientation::Vertical);
                },
                _ => panic!("Invalid remainder")
            }
            for row_off in 0..present_height {
                for col_off in 0..present_width {
                    if present.grid[row_off][col_off] == GridSquare::Empty {
                        continue;
                    }
                    if self.grid[row + row_off][col + col_off].0 != '.' {
                        attempt += 1;
                        continue 'attempts;
                    }
                }
            }
            for row_off in 0..present_height {
                for col_off in 0..present_width {
                    if present.grid[row_off][col_off] == GridSquare::Empty {
                        continue;
                    }
                    self.grid[row + row_off][col + col_off] = present.display_as;
                }
            }
            // println!("Placed present {} at ({row},{col}): {}", present.id, self.grid[row][col].0);
            // self.print();
            return true;
        }
        return false;
    }

    fn find_and_place_present(&mut self, present: &mut Present) -> bool {
        let mut row = 0;
        let mut col = 0;
        // println!("    Placing present {}:", present.id);
        // present.draw();

        loop {
            if self.place_present(row, col, present) {
                return true;
            }
            col += 1;
            if col >= self.width {
                col = 0;
                row += 1;
                if row >= self.height {
                    return false;
                }
            }
        }
    }

    pub fn can_hold_presents(&mut self, presents: &Vec<Present>) -> bool {
        for present_idx in 0..self.presents_reqd.len() {
            let num_presents = self.presents_reqd[present_idx];
            if num_presents == 0 {
                continue;
            }
            let mut present = presents[present_idx].clone();
            for _ in 0..num_presents {
                present.display_as = self.next_present;
                if !self.find_and_place_present(&mut present) {
                    // println!("Failed to place present {} (of type {present_idx})", self.next_present.0);
                    return false;
                }
                // println!("Placed present {} (of type {present_idx})", self.next_present.0);
                // self.print();

                let mut next_ch = (self.next_present.0 as u8 + 1) as char;
                let mut next_colour= self.next_present.1;
                match next_ch {
                    '[' => next_ch = 'a', // after Z, go to a
                    '{' => next_ch = '0', // after z, to to 0
                    ':' => {
                        next_ch = 'A'; // after 9, return to A with different colour
                        next_colour = match next_colour {
                            Color::White => Color::Green,
                            Color::Green => Color::Red,
                            Color::Red => Color::Blue,
                            _ => Color::Black,
                        }
                    },
                    _ => (),
                }
                self.next_present = (next_ch, next_colour);

            }
        }
        return true;
    }

    pub fn print(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let height = self.grid.len();
        let width = self.grid[0].len();
        // println!("Print self");
        for row in 0..height {
            for col in 0..width {
                let (ch, color) = self.grid[row][col];
                stdout.set_color(ColorSpec::new().set_fg(Some(color))).expect("Can't set colour");
                write!(&mut stdout, "{}", ch).expect("Can't write coloured text");
                stdout.reset().expect("Failed to reset colour");
            }
            println!();
        }
    }
}
