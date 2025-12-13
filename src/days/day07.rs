use std::collections::HashMap;

const DEBUG_OUTPUT: bool = false;

pub fn part1(lines: Vec<String>) {
    let mut beams = vec![];
    let total_width = lines[0].len();
    let mut total_splits = 0;
    for line in lines {
        let mut new_beams = vec![];
        let mut pos = 0;
        for char in line.chars() {
            let mut split = false;
            match char {
                'S' => new_beams.push(pos),
                '^' => {
                    if beams.contains(&pos) {
                        total_splits += 1;
                        split = true;
                        if pos > 0 {
                            new_beams.push(pos - 1);
                        }
                        if pos < total_width - 1 {
                            new_beams.push(pos + 1);
                        }
                    }
                }
                '.' => (),
                _ => panic!("Unknown char"),
            }
            // Beam didn't hit a splitter; continue as normal
            if !split && beams.contains(&pos) {
                new_beams.push(pos);
            }
            pos += 1;
        }
        beams = new_beams;
    }
    println!("Total splits: {total_splits}");
}

struct Map {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Pos {
    row: usize,
    col: usize,
}

fn load_grid(lines: Vec<String>) -> (Pos, Map) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut line_num = 0;
    let mut start: Pos = Pos { row: 0, col: 0 };
    let mut width = 0;
    for line in lines {
        grid.push(vec![]);
        let mut char_num = 0;
        for char in line.chars() {
            grid[line_num].push(char);
            if char == 'S' {
                start.row = line_num;
                start.col = char_num;
            }
            char_num += 1;
        }
        if line_num == 0 {
            width = char_num;
        }
        line_num += 1;
    }
    return (
        start,
        Map {
            grid,
            width,
            height: line_num,
        },
    );
}

fn count_timelines(start: Pos, map: &Map, known: &mut HashMap<Pos, u64>) -> u64 {
    match known.get(&start) {
        Some(value) => {
            if DEBUG_OUTPUT {
                println!("REPEAT: ({}, {})", start.row, start.col);
            }
            return *value;
        }
        None => (),
    }
    let mut pos = start.clone();
    while pos.row < map.height - 1 {
        pos.row += 1;
        if map.grid[pos.row][pos.col] == '^' {
            let mut split_positions = vec![];
            if pos.col > 0 {
                split_positions.push(Pos {
                    row: pos.row,
                    col: pos.col - 1,
                });
            }
            if pos.col < map.width - 1 {
                split_positions.push(Pos {
                    row: pos.row,
                    col: pos.col + 1,
                });
            }
            let mut sum = 0;
            for split_pos in split_positions {
                sum += count_timelines(split_pos, &map, known);
            }
            known.insert(start, sum);
            return sum;
        }
    }
    known.insert(start, 1);
    return 1;
}

pub fn part2(lines: Vec<String>) {
    let (start, map) = load_grid(lines);
    let mut known: HashMap<Pos, u64> = HashMap::new();
    let timelines = count_timelines(start, &map, &mut known);
    println!("Timelines: {timelines}");
}
