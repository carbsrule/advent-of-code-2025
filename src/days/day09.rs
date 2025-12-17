const DEBUG_OUTPUT: bool = false;

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

    for row in start_row..=end_row {
        for col in start_col..=end_col {
            if map[row][col] == TileColour::Other {
                return 0;
            }
        }
    }
    let height = end_row - start_row + 1;
    let width = end_col - start_col + 1;
    return height * width;
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

    println!("Calculated initial dimensions: {width}x{height}");

    let mut map = vec![];
    for row in 0..=height {
        map.push(vec![]);
        for _ in 0..=width {
            map[row].push(TileColour::Other);
        }
    }
    println!("Placed initial tiles in grid");

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
    let (mut map, red_tiles) = read_map2(lines);
    if DEBUG_OUTPUT {
        draw_map(&map);
        println!();
    }

    print!("Filling map...");
    fill_map(&mut map);
    println!(" done");
    if DEBUG_OUTPUT {
        draw_map(&map);
        println!();
    }

    let mut max_area = 0;
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            let tile1 = &red_tiles[i];
            let tile2 = &red_tiles[j];
            // print!(
            //     "Checking tiles {i} ({}, {}) and {j} ({}, {}) ... ",
            //     tile1.row, tile1.col, tile2.row, tile2.col,
            // );
            let area = tile_area2(tile1, tile2, &map);
            // print!("{area}");
            if area > max_area {
                max_area = area;
                // print!(" !");
            }
            // println!();
        }
    }
    println!("Max area: {max_area}");
}
