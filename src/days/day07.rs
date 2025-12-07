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
