use std::io;
use std::io::Write;
use std::time::Instant;

const DEBUG_OUTPUT: bool = true;

#[derive(PartialEq, Clone)]
struct Tile {
    row: usize,
    col: usize,
}

fn read_map(lines: Vec<String>) -> Vec<Tile> {
    let mut tiles = vec![];
    for line in lines {
        if line == "" {
            continue;
        }
        let mut part_num = 0;
        let mut col = 0;
        for part in line.split(",") {
            if part_num == 0 {
                col = part.parse().expect("Must be a number");
            } else {
                let row = part.parse().expect("Must be a number");
                let tile = Tile { row, col };
                tiles.push(tile);
            }
            part_num += 1;
        }
    }
    return tiles;
}

fn tile_area(tile1: &Tile, tile2: &Tile) -> usize {
    let height = if tile1.row > tile2.row {
        tile1.row - tile2.row + 1
    } else {
        tile2.row - tile1.row + 1
    };
    let width = if tile1.col > tile2.col {
        tile1.col - tile2.col + 1
    } else {
        tile2.col - tile1.col + 1
    };
    return height * width;
}

fn tile_area2(tile1: &Tile, tile2: &Tile, map: &Map) -> usize {
    let (start_row, end_row) = if tile1.row > tile2.row {
        (tile2.row, tile1.row)
    } else {
        (tile1.row, tile2.row)
    };
    let (start_col, end_col) = if tile1.col > tile2.col {
        (tile2.col, tile1.col)
    } else {
        (tile1.col, tile2.col)
    };

    // Quickly check incrementing distance by 1/2 (and 1/4, 1/8, ...)
    // If this fails then no need to do a laborious run through of every tile
    let mut checked = vec![];
    let mut factor = 2;
    while factor < 1024 {
        let col_increment = (end_col + start_col) / factor;
        let row_increment = (end_row + start_row) / factor;
        if col_increment < 3 || row_increment < 3 {
            break;
        }
        print!(" 1/{factor}");
        flush();
        let mut row = start_row + row_increment;
        while row <= end_row {
            let mut col = start_col + col_increment;
            while col <= end_col {
                let tile = Tile { row, col };
                if checked.contains(&tile) {
                    col += col_increment;
                    continue;
                }
                if map[tile.row][tile.col] == TileColour::Other {
                    print!(
                        " quick invalid (factor {factor}) at ({}, {})",
                        tile.row, tile.col
                    );
                    return 0;
                }
                checked.push(tile);
                col += col_increment;
            }
            row += row_increment;
        }
        factor *= 2;
    }
    print!(" out");
    flush();

    for row in start_row..=end_row {
        for col in start_col..=end_col {
            if map[row][col] == TileColour::Other {
                print!(" invalid at ({row}, {col})");
                return 0;
            }
        }
    }
    let height = end_row - start_row + 1;
    let width = end_col - start_col + 1;
    let area = height * width;
    print!(" {area}");
    return area;
}

pub fn part1(lines: Vec<String>) {
    let tiles = read_map(lines);
    let mut max_area = 0;
    for i in 0..tiles.len() {
        for j in i..tiles.len() {
            let tile1 = &tiles[i];
            let tile2 = &tiles[j];
            let area = tile_area(tile1, tile2);
            if area > max_area {
                max_area = area;
            }
        }
    }
    println!("Max area: {max_area}");
}

#[derive(PartialEq, Copy, Clone)]
enum TileColour {
    Red,
    Green,
    Other,
}

#[derive(PartialEq)]
enum Scanning {
    Blank,
    OpenEdge,
    Paintable,
    CloseEdge,
}

type Map = Vec<Vec<TileColour>>;

// draw the map as per the puzzle definition
#[allow(dead_code)]
fn draw_map(map: &Map) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                TileColour::Red => print!("#"),
                TileColour::Green => print!("X"),
                TileColour::Other => print!("."),
            }
        }
        println!();
    }
}

fn flush() {
    io::stdout().flush().expect("Err on flush");
}

fn line_fill(tile1: &Tile, tile2: &Tile, map: &mut Map) {
    let mut direction: (i32, i32) = (0, 0);
    if tile1.row < tile2.row {
        direction = (1, 0);
    } else if tile1.row > tile2.row {
        direction = (-1, 0);
    } else if tile1.col < tile2.col {
        direction = (0, 1);
    } else if tile1.col > tile2.col {
        direction = (0, -1);
    }
    let mut tile: Tile = tile1.clone();
    tile.row = ((tile.row as i32) + direction.0) as usize;
    tile.col = ((tile.col as i32) + direction.1) as usize;
    while &tile != tile2 {
        map[tile.row][tile.col] = TileColour::Green;
        tile.row = ((tile.row as i32) + direction.0) as usize;
        tile.col = ((tile.col as i32) + direction.1) as usize;
    }
}

fn read_map2(lines: Vec<String>) -> (Map, Vec<Tile>) {
    let red_tiles = read_map(lines);
    println!("Loaded red tiles: {}", red_tiles.len());

    print!("Calculating initial dimensions...");
    flush();

    let mut height = 0;
    let mut width = 0;
    for i in 0..red_tiles.len() {
        if red_tiles[i].row > height {
            height = red_tiles[i].row;
        }
        if red_tiles[i].col > width {
            width = red_tiles[i].col;
        }
    }

    // Always need 1 row of padding
    height += 1;
    width += 1;

    println!(" {width}x{height}");

    print!("Placing initial tiles in grid...");
    flush();
    let mut map = vec![];
    for row in 0..=height {
        map.push(vec![]);
        for _ in 0..=width {
            map[row].push(TileColour::Other);
        }
    }
    println!(" done");

    println!("Filling lines");
    for i in 0..red_tiles.len() {
        let tile = &red_tiles[i];
        map[tile.row][tile.col] = TileColour::Red;
        let next = &red_tiles[(i + 1) % red_tiles.len()];
        if DEBUG_OUTPUT {
            println!(
                "Filling line from ({}, {}) to ({}, {})",
                tile.row, tile.col, next.row, next.col,
            );
        }
        line_fill(tile, next, &mut map);
    }
    println!("Filled lines");

    return (map, red_tiles);
}

fn fill_map(map: &mut Map) {
    let mut scanning;
    for row in 0..map.len() {
        scanning = Scanning::Blank;
        let mut paintable = vec![];
        for col in 0..map[row].len() {
            if map[row][col] != TileColour::Other {
                if scanning == Scanning::Blank {
                    scanning = Scanning::OpenEdge
                } else if scanning == Scanning::Paintable {
                    scanning = Scanning::CloseEdge;
                    for paint_col in paintable {
                        map[row][paint_col] = TileColour::Green;
                    }
                    paintable = vec![];
                }
            } else {
                if scanning == Scanning::OpenEdge {
                    scanning = Scanning::Paintable;
                } else if scanning == Scanning::CloseEdge {
                    scanning = Scanning::Blank;
                }
            }
            if scanning == Scanning::Paintable {
                paintable.push(col);
            }
        }
    }
}

pub fn part2(lines: Vec<String>) {
    let start = Instant::now();
    let (mut map, red_tiles) = read_map2(lines);
    println!("Time so far: {:?}", start.elapsed());
    // if DEBUG_OUTPUT {
    //     draw_map(&map);
    //     println!();
    // }

    print!("Filling map...");
    flush();
    fill_map(&mut map);
    println!(" done (time so far: {:?})", start.elapsed());
    // if DEBUG_OUTPUT {
    //     draw_map(&map);
    //     println!();
    // }

    let mut rectangles = vec![];

    print!("Calculating possible sizes... ");
    flush();
    let mut max_area = 0;
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            let tile1 = &red_tiles[i];
            let tile2 = &red_tiles[j];
            let area = tile_area(tile1, tile2);
            rectangles.push((tile1, tile2, area));
        }
    }
    println!(" done (time so far: {:?})", start.elapsed());

    print!("Sorting possible sizes...");
    flush();
    rectangles.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    rectangles.reverse();
    println!(" done (time so far: {:?})", start.elapsed());

    println!("Checking {} rectangles", rectangles.len());
    let mut rectangles_checked = 0;
    for (tile1, tile2, possible_area) in rectangles {
        print!(
            "Checking tiles ({}, {}) and ({}, {}) with area: {possible_area} ...",
            tile1.row, tile1.col, tile2.row, tile2.col,
        );
        flush();
        let area = tile_area2(tile1, tile2, &map);
        if area > max_area {
            max_area = area;
            println!(" !");
            break;
        }
        println!();
        rectangles_checked += 1;
        if rectangles_checked % 1000 == 0 {
            println!(
                "{rectangles_checked} rectangles checked in {:?}",
                start.elapsed()
            );
        }
    }
    println!("Max area: {max_area}");
    println!("Total time: {:?}", start.elapsed());
}
