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
